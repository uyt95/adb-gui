//! Blocking wrappers around the `adb` and `emulator` command line tools.
//!
//! Everything here runs on a worker thread; the UI calls it through
//! [`crate::state::AppState::run`]. Errors are plain messages for display.

mod log;
mod parse;
mod process;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::models::{
    Automation, CommandType, Device, EmulatorParams, InstallMode, Proxy, ProxyType,
};

pub use log::{CommandLog, Record, RecordState, format_duration};
use parse::{parse_avds, parse_devices, parse_proxies, split_lines};
pub use process::find_executable;
use process::{execute, execute_checked, execute_validated};

pub type Result<T> = std::result::Result<T, String>;

/// Proxies listed by adb for one device; see [`Tools::list_proxies`].
#[derive(Clone, Debug, Default)]
pub struct ListedProxies {
    /// Proxies active on the device the list was made for.
    pub active: Vec<Proxy>,
    /// Forwards that adb listed for other connected devices.
    pub other_devices: Vec<Proxy>,
}

/// Paths of the external tools, copied from the settings for use on a worker thread.
#[derive(Clone, Debug)]
pub struct Tools {
    pub adb: String,
    pub emulator: String,
    /// Where every command run through these tools is recorded.
    pub log: CommandLog,
}

/// Quotes `text` as one word for the device shell that `adb shell` runs its
/// arguments in. Embedded single quotes are closed, escaped and reopened.
pub fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', r"'\''"))
}

/// What `adb forward`/`adb reverse` prints when a proxy is turned on: nothing for
/// non-tcp sockets, or the bound tcp port (the allocated one for `tcp:0`).
/// `\A` and `\z` anchor to the whole output; `^` and `$` would match any line.
const PROXY_ACTIVATED: &str = r"\A\d*\z";

/// No output at all. A blank line inside error output must not pass, so this
/// anchors to the whole output rather than to a line.
const NO_OUTPUT: &str = r"\A\z";

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
        execute_validated(
            &self.log,
            &self.adb,
            &adb_arguments(device, arguments),
            patterns,
        )
    }

    /// Output lines of an adb command, without `* daemon ...` notices. Errors give no lines.
    fn adb_lines(&self, device: Option<&Device>, arguments: &[&str]) -> Vec<String> {
        execute(
            &self.log,
            &self.adb,
            &adb_arguments(device, strings(arguments)),
        )
        .map(|output| split_lines(&output))
        .unwrap_or_default()
    }

    /// Connected devices. Fails when adb cannot start or exits with an error.
    pub fn list_devices(&self) -> Result<Vec<Device>> {
        execute(&self.log, &self.adb, &strings(&["devices"]))
            .map(|output| parse_devices(&split_lines(&output)))
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

    /// Available AVD names. Fails when the emulator cannot start or exits with an error.
    pub fn list_avds(&self) -> Result<Vec<String>> {
        execute(&self.log, &self.emulator, &strings(&["-list-avds"]))
            .map(|output| parse_avds(&split_lines(&output)))
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
        let entry = self.log.start(&self.emulator, &arguments);
        let mut child = Command::new(&self.emulator)
            .args(&arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| {
                let message = format!("{}: {error}", self.emulator);
                entry.not_started(&message);
                message
            })?;
        entry.launched();
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
            &[NO_OUTPUT],
        )
        .map(drop)
    }

    pub fn send_text(&self, device: &Device, text: &str) -> Result<()> {
        let quoted = shell_quote(text);
        self.adb(
            Some(device),
            strings(&["shell", "input", "keyboard", "text", &quoted]),
            &[NO_OUTPUT],
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
            &[NO_OUTPUT],
        )
        .map(drop)
    }

    /// Forward and reverse proxies listed by adb, split into those active on
    /// `device` and those of other devices. `adb forward --list` lists the
    /// forwards of every device, even with `-s`; `reverse --list` asks the device
    /// itself, so all of its lines belong to `device`.
    pub fn list_proxies(&self, device: &Device) -> ListedProxies {
        let mut listed = ListedProxies::default();
        for proxy_type in ProxyType::ALL {
            let lines = self.adb_lines(Some(device), &[proxy_type.type_name(), "--list"]);
            for (serial, proxy) in parse_proxies(proxy_type, &lines) {
                if proxy_type == ProxyType::Reverse || serial == device.serial {
                    listed.active.push(proxy);
                } else {
                    listed.other_devices.push(proxy);
                }
            }
        }
        listed
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
            &[NO_OUTPUT],
        )?;
        self.adb(
            device,
            strings(&["shell", "screencap", "-p", &temp_path]),
            &[NO_OUTPUT],
        )?;
        std::fs::create_dir_all(directory).map_err(|error| format!("{directory}: {error}"))?;
        // A pull can exit with 0 and still print an error.
        execute_checked(
            &self.log,
            &self.adb,
            &adb_arguments(
                device,
                vec![
                    "pull".into(),
                    temp_path.clone(),
                    path.to_string_lossy().into_owned(),
                ],
            ),
            |output| !has_adb_error(output),
        )?;
        self.adb(device, strings(&["shell", "rm", &temp_path]), &[NO_OUTPUT])?;
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

/// Whether a line of the output is an adb error, e.g. from a pull that exits 0.
fn has_adb_error(output: &str) -> bool {
    output
        .lines()
        .any(|line| line.trim_start().starts_with("adb: error:"))
}

#[cfg(test)]
mod tests {
    use super::process::validate;
    use super::{NO_OUTPUT, PROXY_ACTIVATED, has_adb_error};

    #[test]
    fn no_output_rejects_errors_with_blank_lines() {
        assert!(validate("", &[NO_OUTPUT]));
        assert!(validate("\n", &[NO_OUTPUT]));
        assert!(!validate(
            "Error: Unknown keycode\n\nUsage: input",
            &[NO_OUTPUT]
        ));
        assert!(!validate("mkdir: failed\n", &[NO_OUTPUT]));
    }

    #[test]
    fn pull_errors_are_found_on_any_line() {
        assert!(!has_adb_error(
            "/sdcard/adb-gui/tmp-screenshot.png: 1 file pulled, 0 skipped.\n"
        ));
        assert!(!has_adb_error(""));
        assert!(has_adb_error(
            "adb: error: failed to stat remote object\n/sdcard/a.png: 0 files pulled\n"
        ));
    }

    #[test]
    fn proxy_activation_accepts_empty_output_or_a_port() {
        assert!(validate("", &[PROXY_ACTIVATED]));
        assert!(validate("8080\n", &[PROXY_ACTIVATED]));
        assert!(validate("41235\n", &[PROXY_ACTIVATED]));
        assert!(!validate("8080\n\nerror: closed", &[PROXY_ACTIVATED]));
        assert!(!validate(
            "adb: error: cannot bind listener\n",
            &[PROXY_ACTIVATED]
        ));
    }
}
