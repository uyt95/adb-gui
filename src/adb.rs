//! Blocking wrappers around the `adb` and `emulator` command line tools.
//!
//! Everything here runs on a worker thread; the UI calls it through
//! [`crate::state::AppState::run`]. Errors are plain messages for display.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use regex::RegexBuilder;

use crate::models::{
    Automation, CommandType, Device, EmulatorParams, InstallMode, Proxy, ProxyType, Socket,
};

pub type Result<T> = std::result::Result<T, String>;

/// Paths of the external tools, copied from the settings for use on a worker thread.
#[derive(Clone, Debug)]
pub struct Tools {
    pub adb: String,
    pub emulator: String,
}

/// Runs a command and returns its stdout, or stderr (falling back to stdout) on failure.
pub fn execute(command: &str, arguments: &[String]) -> Result<String> {
    let output = Command::new(command)
        .args(arguments)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("{command}: {error}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    if output.status.success() {
        return Ok(stdout);
    }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(if stderr.is_empty() {
        stdout.trim().to_string()
    } else {
        stderr
    })
}

/// Whether the output matches any of the patterns (multi-line, case-insensitive).
/// An empty pattern list accepts any output.
pub fn validate(output: &str, patterns: &[&str]) -> bool {
    patterns.is_empty()
        || patterns.iter().any(|pattern| {
            RegexBuilder::new(pattern)
                .multi_line(true)
                .case_insensitive(true)
                .build()
                .map(|regex| regex.is_match(output))
                .unwrap_or(false)
        })
}

/// Runs a command and fails with its output when the output does not validate.
pub fn execute_validated(command: &str, arguments: &[String], patterns: &[&str]) -> Result<String> {
    let output = execute(command, arguments)?;
    if validate(&output, patterns) {
        Ok(output)
    } else {
        Err(output.trim().to_string())
    }
}

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
        let quoted = format!("'{text}'");
        self.adb(
            Some(device),
            strings(&["shell", "input", "keyboard", "text", &quoted]),
            &["^$"],
        )
        .map(drop)
    }

    pub fn activate_proxy(&self, device: &Device, proxy: &Proxy) -> Result<()> {
        let pattern = format!(r"^\s*{}\s*$", regex::escape(&proxy.from.name));
        self.adb(
            Some(device),
            vec![
                proxy.proxy_type.type_name().into(),
                proxy.from.spec(),
                proxy.to.spec(),
            ],
            &[&pattern],
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

/// Finds an executable on `PATH`, like `which`.
pub fn find_executable(name: &str) -> Option<String> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|candidate| {
            use std::os::unix::fs::PermissionsExt;
            candidate
                .metadata()
                .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        })
        .map(|p| p.to_string_lossy().into_owned())
}

fn split_lines(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| !line.starts_with("* "))
        .map(str::to_string)
        .collect()
}

/// Parses the output of `adb devices` (after the header line).
pub fn parse_devices(lines: &[String]) -> Vec<Device> {
    lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let columns: Vec<&str> = line.split('\t').collect();
            (columns.len() == 2).then(|| Device {
                serial: columns[0].to_string(),
            })
        })
        .collect()
}

/// Parses `adb forward --list` / `adb reverse --list` lines: `<serial> <from> <to>`.
pub fn parse_proxies(proxy_type: ProxyType, lines: &[String]) -> Vec<Proxy> {
    lines
        .iter()
        .filter_map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() != 3 {
                return None;
            }
            Some(Proxy {
                id: 0,
                proxy_type,
                from: Socket::parse(tokens[1])?,
                to: Socket::parse(tokens[2])?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SocketType;

    fn lines(text: &str) -> Vec<String> {
        split_lines(text)
    }

    #[test]
    fn parses_devices() {
        let output = "* daemon not running; starting now at tcp:5037\n* daemon started successfully\nList of devices attached\nemulator-5554\tdevice\n192.168.1.5:5555\toffline\n\n";
        let devices = parse_devices(&lines(output));
        assert_eq!(
            devices,
            vec![
                Device {
                    serial: "emulator-5554".into()
                },
                Device {
                    serial: "192.168.1.5:5555".into()
                },
            ]
        );
    }

    #[test]
    fn parses_proxies() {
        let output =
            "emulator-5554 tcp:8080 tcp:80\nemulator-5554 localabstract:x jdwp:12\ngarbage\n";
        let proxies = parse_proxies(ProxyType::Forward, &lines(output));
        assert_eq!(proxies.len(), 2);
        assert_eq!(proxies[0].from.socket_type, SocketType::Tcp);
        assert_eq!(proxies[1].to.name, "12");
    }

    #[test]
    fn validates_like_the_flutter_app() {
        assert!(validate("", &["^$"]));
        assert!(validate(
            "connected to 1.2.3.4:5555\n",
            &["(already )?connected to .*"]
        ));
        assert!(validate(
            "Performing Streamed Install\nSuccess\n",
            &[".*success$"]
        ));
        assert!(!validate(
            "failed to connect\n",
            &["(already )?connected to .*"]
        ));
        assert!(validate("anything", &[]));
    }

    #[test]
    fn pull_validation_rejects_adb_errors() {
        let pattern = ["^(?:adb: error:)?$|^[^a]|^a[^d]"];
        assert!(validate("/sdcard/x.png: 1 file pulled.\n", &pattern));
        assert!(!validate(
            "adb: error: failed to stat remote object",
            &pattern
        ));
    }

    #[test]
    fn execute_reports_stderr() {
        let error = execute("sh", &["-c".into(), "echo oops >&2; exit 1".into()]).unwrap_err();
        assert_eq!(error, "oops");
        assert_eq!(
            execute("sh", &["-c".into(), "echo hi".into()]).unwrap(),
            "hi\n"
        );
    }
}
