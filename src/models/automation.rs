//! Automations: chains of adb commands.

use serde::{Deserialize, Serialize};

use super::key_code::KEY_CODES;

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
    #[serde(default)]
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
            ArgumentType::Text => crate::tools::shell_quote(&self.value),
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
    #[serde(default)]
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
#[serde(default)]
pub struct Automation {
    pub id: u64,
    pub name: String,
    pub commands: Vec<Command>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_command_quotes_text() {
        let mut command = Command::new(CommandType::Adb);
        command.operation.arguments[4].value = "hello world".into();
        assert_eq!(
            command.arguments(),
            ["shell", "input", "keyboard", "text", "'hello world'"]
        );
    }

    #[test]
    fn keyboard_command_escapes_single_quotes() {
        let mut command = Command::new(CommandType::Adb);
        command.operation.arguments[4].value = "don't; reboot".into();
        assert_eq!(command.arguments()[4], r"'don'\''t; reboot'");
    }
}
