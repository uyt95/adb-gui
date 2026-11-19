//! Parsing adb output.

use crate::models::{Device, Proxy, ProxyType, Socket};

/// Output lines without the `* daemon ...` notices adb prints when it starts.
pub fn split_lines(output: &str) -> Vec<String> {
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
}
