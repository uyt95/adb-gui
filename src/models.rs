//! Data types shared by the store, the adb layer and the UI.

use serde::{Deserialize, Serialize};

use crate::keycodes::KEY_CODES;

/// A device as listed by `adb devices`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    /// The full serial, e.g. `emulator-5554` or `192.168.1.5:5555`.
    pub serial: String,
}

impl Device {
    /// The serial without a trailing `:port`, used to match saved connections.
    pub fn host(&self) -> &str {
        self.serial.split(':').next().unwrap_or(&self.serial)
    }

    /// Whether a saved connection address refers to this device.
    pub fn matches_address(&self, address: &str) -> bool {
        let address = address.trim();
        address == self.serial || address == self.host()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Connection {
    pub id: u64,
    pub name: String,
    pub address: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EmulatorParams {
    pub no_snapshot_load: bool,
    pub writeable_system: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Emulator {
    pub id: u64,
    pub name: String,
    pub params: EmulatorParams,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProxyType {
    Forward,
    Reverse,
}

impl ProxyType {
    pub const ALL: [ProxyType; 2] = [ProxyType::Forward, ProxyType::Reverse];

    pub fn type_name(self) -> &'static str {
        match self {
            ProxyType::Forward => "forward",
            ProxyType::Reverse => "reverse",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SocketType {
    AcceptFd,
    Dev,
    Jdwp,
    LocalAbstract,
    LocalFileSystem,
    LocalReserved,
    Tcp,
    Vsock,
}

impl SocketType {
    pub const ALL: [SocketType; 8] = [
        SocketType::AcceptFd,
        SocketType::Dev,
        SocketType::Jdwp,
        SocketType::LocalAbstract,
        SocketType::LocalFileSystem,
        SocketType::LocalReserved,
        SocketType::Tcp,
        SocketType::Vsock,
    ];

    pub fn type_name(self) -> &'static str {
        match self {
            SocketType::AcceptFd => "acceptfd",
            SocketType::Dev => "dev",
            SocketType::Jdwp => "jdwp",
            SocketType::LocalAbstract => "localabstract",
            SocketType::LocalFileSystem => "localfilesystem",
            SocketType::LocalReserved => "localreserved",
            SocketType::Tcp => "tcp",
            SocketType::Vsock => "vsock",
        }
    }

    pub fn from_type_name(name: &str) -> Option<SocketType> {
        SocketType::ALL.into_iter().find(|t| t.type_name() == name)
    }

    pub fn supports(self, proxy_type: ProxyType) -> bool {
        match self {
            SocketType::LocalAbstract
            | SocketType::LocalFileSystem
            | SocketType::LocalReserved
            | SocketType::Tcp => true,
            SocketType::AcceptFd | SocketType::Dev | SocketType::Jdwp | SocketType::Vsock => {
                proxy_type == ProxyType::Forward
            }
        }
    }

    /// Socket types usable for the given proxy type, in display order.
    pub fn for_proxy_type(proxy_type: ProxyType) -> Vec<SocketType> {
        SocketType::ALL
            .into_iter()
            .filter(|t| t.supports(proxy_type))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Socket {
    #[serde(rename = "type")]
    pub socket_type: SocketType,
    pub name: String,
}

impl Socket {
    /// Parses `tcp:8080` style specs as printed by `adb forward --list`.
    pub fn parse(input: &str) -> Option<Socket> {
        let (type_name, name) = input.split_once(':')?;
        Some(Socket {
            socket_type: SocketType::from_type_name(type_name)?,
            name: name.to_string(),
        })
    }

    pub fn spec(&self) -> String {
        format!("{}:{}", self.socket_type.type_name(), self.name)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proxy {
    pub id: u64,
    #[serde(rename = "type")]
    pub proxy_type: ProxyType,
    pub from: Socket,
    pub to: Socket,
}

impl Proxy {
    pub fn same_mapping(&self, other: &Proxy) -> bool {
        self.proxy_type == other.proxy_type && self.from == other.from && self.to == other.to
    }
}

impl std::fmt::Display for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.proxy_type.type_name(),
            self.from.spec(),
            self.to.spec()
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ArgumentType {
    KeyCode,
    Static,
    Text,
}

impl ArgumentType {
    pub fn label(self) -> &'static str {
        match self {
            ArgumentType::KeyCode => "Key code",
            ArgumentType::Static => "Static",
            ArgumentType::Text => "Text",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Argument {
    #[serde(rename = "type")]
    pub argument_type: ArgumentType,
    pub value: String,
}

impl Argument {
    pub fn new(argument_type: ArgumentType) -> Argument {
        let value = match argument_type {
            ArgumentType::KeyCode => KEY_CODES[0].code.to_string(),
            ArgumentType::Static | ArgumentType::Text => String::new(),
        };
        Argument {
            argument_type,
            value,
        }
    }

    pub fn fixed(value: &str) -> Argument {
        Argument {
            argument_type: ArgumentType::Static,
            value: value.to_string(),
        }
    }

    /// The value as passed on the adb command line. Text is single-quoted
    /// because `adb shell` hands its arguments to the device shell.
    pub fn command_value(&self) -> String {
        match self.argument_type {
            ArgumentType::Text => format!("'{}'", self.value),
            ArgumentType::KeyCode | ArgumentType::Static => self.value.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommandType {
    Adb,
}

impl CommandType {
    pub const ALL: [CommandType; 1] = [CommandType::Adb];

    pub fn label(self) -> &'static str {
        match self {
            CommandType::Adb => "ADB",
        }
    }

    pub fn operations(self) -> &'static [OperationType] {
        match self {
            CommandType::Adb => &[
                OperationType::KeyboardInput,
                OperationType::KeyCode,
                OperationType::Reboot,
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OperationType {
    KeyboardInput,
    KeyCode,
    Reboot,
}

impl OperationType {
    pub fn label(self) -> &'static str {
        match self {
            OperationType::KeyboardInput => "Send keyboard input",
            OperationType::KeyCode => "Send key code",
            OperationType::Reboot => "Reboot",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(rename = "type")]
    pub operation_type: OperationType,
    pub arguments: Vec<Argument>,
}

impl Operation {
    pub fn new(operation_type: OperationType) -> Operation {
        let arguments = match operation_type {
            OperationType::KeyboardInput => vec![
                Argument::fixed("shell"),
                Argument::fixed("input"),
                Argument::fixed("keyboard"),
                Argument::fixed("text"),
                Argument::new(ArgumentType::Text),
            ],
            OperationType::KeyCode => vec![
                Argument::fixed("shell"),
                Argument::fixed("input"),
                Argument::fixed("keyevent"),
                Argument::new(ArgumentType::KeyCode),
            ],
            OperationType::Reboot => vec![Argument::fixed("reboot")],
        };
        Operation {
            operation_type,
            arguments,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Command {
    #[serde(rename = "type")]
    pub command_type: CommandType,
    pub operation: Operation,
}

impl Command {
    pub fn new(command_type: CommandType) -> Command {
        Command {
            command_type,
            operation: Operation::new(command_type.operations()[0]),
        }
    }

    pub fn arguments(&self) -> Vec<String> {
        self.operation
            .arguments
            .iter()
            .map(Argument::command_value)
            .collect()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Automation {
    pub id: u64,
    pub name: String,
    pub commands: Vec<Command>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstallMode {
    Normal,
    PrivApp,
}

impl InstallMode {
    pub const ALL: [InstallMode; 2] = [InstallMode::Normal, InstallMode::PrivApp];

    pub fn label(self) -> &'static str {
        match self {
            InstallMode::Normal => "Normal",
            InstallMode::PrivApp => "Priv app",
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingKey {
    AdbPath,
    EmulatorPath,
    ScreenshotPath,
}

impl SettingKey {
    pub const ALL: [SettingKey; 3] = [
        SettingKey::AdbPath,
        SettingKey::EmulatorPath,
        SettingKey::ScreenshotPath,
    ];
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub adb_path: String,
    pub emulator_path: String,
    pub screenshot_path: String,
}

impl Settings {
    pub fn get(&self, key: SettingKey) -> &str {
        match key {
            SettingKey::AdbPath => &self.adb_path,
            SettingKey::EmulatorPath => &self.emulator_path,
            SettingKey::ScreenshotPath => &self.screenshot_path,
        }
    }

    pub fn set(&mut self, key: SettingKey, value: String) {
        match key {
            SettingKey::AdbPath => self.adb_path = value,
            SettingKey::EmulatorPath => self.emulator_path = value,
            SettingKey::ScreenshotPath => self.screenshot_path = value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_matches_address_with_and_without_port() {
        let device = Device {
            serial: "192.168.1.5:5555".into(),
        };
        assert!(device.matches_address("192.168.1.5"));
        assert!(device.matches_address("192.168.1.5:5555"));
        assert!(!device.matches_address("192.168.1.6"));
    }

    #[test]
    fn socket_parse_round_trip() {
        let socket = Socket::parse("localabstract:foo:bar").unwrap();
        assert_eq!(socket.socket_type, SocketType::LocalAbstract);
        assert_eq!(socket.name, "foo:bar");
        assert_eq!(socket.spec(), "localabstract:foo:bar");
        assert!(Socket::parse("bogus:1").is_none());
    }

    #[test]
    fn reverse_only_offers_shared_socket_types() {
        let types = SocketType::for_proxy_type(ProxyType::Reverse);
        assert_eq!(types.first(), Some(&SocketType::LocalAbstract));
        assert!(!types.contains(&SocketType::Jdwp));
    }

    #[test]
    fn keyboard_command_quotes_text() {
        let mut command = Command::new(CommandType::Adb);
        command.operation.arguments[4].value = "hello world".into();
        assert_eq!(
            command.arguments(),
            ["shell", "input", "keyboard", "text", "'hello world'"]
        );
    }
}
