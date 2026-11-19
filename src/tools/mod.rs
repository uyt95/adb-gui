//! Blocking wrappers around the `adb` and `emulator` command line tools.
//!
//! Everything here runs on a worker thread; the UI calls it through
//! [`crate::state::AppState::run`]. Errors are plain messages for display.

mod parse;
mod process;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::models::{
    Automation, CommandType, Device, EmulatorParams, InstallMode, Proxy, ProxyType,
};

use parse::{parse_devices, parse_proxies, split_lines};
pub use process::find_executable;
use process::{execute, execute_validated};

pub type Result<T> = std::result::Result<T, String>;

/// Paths of the external tools, copied from the settings for use on a worker thread.
#[derive(Clone, Debug)]
pub struct Tools {
    pub adb: String,
    pub emulator: String,
}

/// Quotes `text` as one word for the device shell that `adb shell` runs its
/// arguments in. Embedded single quotes are closed, escaped and reopened.
pub fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', r"'\''"))
}

/// What `adb forward`/`adb reverse` prints when a proxy is turned on: nothing for
/// non-tcp sockets, or the bound tcp port (the allocated one for `tcp:0`).
const PROXY_ACTIVATED: &str = r"^\d*$";

fn strings(arguments: &[&str]) -> Vec<String> {
    arguments.iter().map(|a| a.to_string()).collect()
}

fn adb_arguments(device: Option<&Device>, arguments: Vec<String>) -> Vec<String> {
    match device {
        Some(device) => {
            let mut all = vec!["-s".to_string(), device.serial.clone()];
            all.extend(arguments);
            all
        }
        None => arguments,
    }
}

impl Tools {
    fn adb(
        &self,
        device: Option<&Device>,
        arguments: Vec<String>,
        patterns: &[&str],
    ) -> Result<String> {
        execute_validated(&self.adb, &adb_arguments(device, arguments), patterns)
    }

    /// Output lines of an adb command, without `* daemon ...` notices. Errors give no lines.
    fn adb_lines(&self, device: Option<&Device>, arguments: &[&str]) -> Vec<String> {
        execute(&self.adb, &adb_arguments(device, strings(arguments)))
            .map(|output| split_lines(&output))
            .unwrap_or_default()
    }

    pub fn list_devices(&self) -> Vec<Device> {
        parse_devices(&self.adb_lines(None, &["devices"]))
    }

    pub fn connect(&self, address: &str) -> Result<()> {
        self.adb(
            None,
            strings(&["connect", address]),
            &["(already )?connected to .*"],
        )
        .map(drop)
    }

    pub fn disconnect(&self, address: &str) -> Result<()> {
        self.adb(
            None,
            strings(&["disconnect", address]),
            &["disconnected .*", "error: no such device '.*'"],
        )
        .map(drop)
    }

    pub fn list_avds(&self) -> Vec<String> {
        execute(&self.emulator, &strings(&["-list-avds"]))
            .map(|output| {
                split_lines(&output)
                    .into_iter()
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Starts an emulator in the background without waiting for it.
    pub fn start_emulator(&self, name: &str, params: &EmulatorParams) -> Result<()> {
        let mut arguments = strings(&["-avd", name]);
        if params.no_snapshot_load {
            arguments.push("-no-snapshot-load".into());
        }
        if params.writeable_system {
            arguments.push("-writable-system".into());
        }
        let mut child = Command::new(&self.emulator)
            .args(&arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| format!("{}: {error}", self.emulator))?;
        // Reap the process when it exits so it does not linger as a zombie.
        std::thread::spawn(move || child.wait());
        Ok(())
    }

    pub fn install(&self, device: &Device, path: &str, mode: InstallMode) -> Result<()> {
        let device = Some(device);
        match mode {
            InstallMode::Normal => {
                self.adb(device, strings(&["install", path]), &[".*success$"])?;
            }
            InstallMode::PrivApp => {
                self.adb(
                    device,
                    strings(&["root"]),
                    &[
                        "^restarting adbd as root$",
                        "^adbd is already running as root$",
                    ],
                )?;
                self.adb(device, strings(&["remount"]), &["^remount succeeded$"])?;
                self.adb(device, strings(&["push", path, "/system/priv-app"]), &[])?;
                self.adb(device, strings(&["reboot"]), &[])?;
            }
        }
        Ok(())
    }

    /// Sends `KEYCODE_<key>`, e.g. `send_key(device, "HOME")`.
    pub fn send_key(&self, device: &Device, key: &str) -> Result<()> {
        let code = format!("KEYCODE_{key}");
        self.adb(
            Some(device),
            strings(&["shell", "input", "keyevent", &code]),
            &["^$"],
        )
        .map(drop)
    }

    pub fn send_text(&self, device: &Device, text: &str) -> Result<()> {
        let quoted = shell_quote(text);
        self.adb(
            Some(device),
            strings(&["shell", "input", "keyboard", "text", &quoted]),
            &["^$"],
        )
        .map(drop)
    }

    pub fn activate_proxy(&self, device: &Device, proxy: &Proxy) -> Result<()> {
        self.adb(
            Some(device),
            vec![
                proxy.proxy_type.type_name().into(),
                proxy.from.spec(),
                proxy.to.spec(),
            ],
            &[PROXY_ACTIVATED],
        )
        .map(drop)
    }

    pub fn deactivate_proxy(&self, device: &Device, proxy: &Proxy) -> Result<()> {
        self.adb(
            Some(device),
            vec![
                proxy.proxy_type.type_name().into(),
                "--remove".into(),
                proxy.from.spec(),
            ],
            &["^$"],
        )
        .map(drop)
    }

    /// Forward and reverse proxies currently active on the device.
    pub fn list_active_proxies(&self, device: &Device) -> Vec<Proxy> {
        ProxyType::ALL
            .into_iter()
            .flat_map(|proxy_type| {
                let lines = self.adb_lines(Some(device), &[proxy_type.type_name(), "--list"]);
                parse_proxies(proxy_type, &lines)
            })
            .collect()
    }

    /// Captures the screen into `directory` and returns the path of the PNG file.
    pub fn take_screenshot(
        &self,
        device: &Device,
        directory: &str,
        timestamp: &str,
    ) -> Result<PathBuf> {
        const DEVICE_DIRECTORY: &str = "/sdcard/adb-gui";
        let temp_path = format!("{DEVICE_DIRECTORY}/tmp-screenshot.png");
        let path = Path::new(directory).join(format!("screenshot-{timestamp}.png"));
        let device = Some(device);
        self.adb(
            device,
            strings(&["shell", "mkdir", "-p", DEVICE_DIRECTORY]),
            &["^$"],
        )?;
        self.adb(
            device,
            strings(&["shell", "screencap", "-p", &temp_path]),
            &["^$"],
        )?;
        std::fs::create_dir_all(directory).map_err(|error| format!("{directory}: {error}"))?;
        self.adb(
            device,
            vec![
                "pull".into(),
                temp_path.clone(),
                path.to_string_lossy().into_owned(),
            ],
            &["^(?:adb: error:)?$|^[^a]|^a[^d]"],
        )?;
        self.adb(device, strings(&["shell", "rm", &temp_path]), &["^$"])?;
        Ok(path)
    }

    /// Runs the automation's commands in order, stopping at the first failure.
    pub fn run_automation(&self, device: &Device, automation: &Automation) -> Result<()> {
        for command in &automation.commands {
            match command.command_type {
                CommandType::Adb => {
                    self.adb(Some(device), command.arguments(), &[])?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PROXY_ACTIVATED;
    use super::process::validate;

    #[test]
    fn proxy_activation_accepts_empty_output_or_a_port() {
        assert!(validate("", &[PROXY_ACTIVATED]));
        assert!(validate("8080\n", &[PROXY_ACTIVATED]));
        assert!(validate("41235\n", &[PROXY_ACTIVATED]));
        assert!(!validate(
            "adb: error: cannot bind listener\n",
            &[PROXY_ACTIVATED]
        ));
    }
}
