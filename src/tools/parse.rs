//! Parsing adb output.

use super::{MdnsService, MdnsServiceType};
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
                state: columns[1].trim().to_string(),
            })
        })
        .collect()
}

/// Parses `emulator -list-avds` output. Only lines that are valid AVD names
/// (letters, digits, `.`, `_` and `-`) are kept, which drops log lines such as
/// `INFO    | Storing crashdata in: ...` that newer emulators print there.
pub fn parse_avds(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| {
            !line.is_empty()
                && line
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
        })
        .map(str::to_string)
        .collect()
}

/// Parses `adb forward --list` / `adb reverse --list` lines: `<serial> <from> <to>`.
/// Returns each proxy with the serial from its line.
pub fn parse_proxies(proxy_type: ProxyType, lines: &[String]) -> Vec<(String, Proxy)> {
    lines
        .iter()
        .filter_map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() != 3 {
                return None;
            }
            let proxy = Proxy {
                id: 0,
                proxy_type,
                from: Socket::parse(tokens[1])?,
                to: Socket::parse(tokens[2])?,
            };
            Some((tokens[0].to_string(), proxy))
        })
        .collect()
}

/// Parses `adb mdns services` lines: `<instance>\t<service type>\t<ip>:<port>`.
/// The header line and unknown service types are skipped.
pub fn parse_mdns_services(lines: &[String]) -> Vec<MdnsService> {
    lines
        .iter()
        .filter_map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let [instance, service_type, address] = tokens[..] else {
                return None;
            };
            let service_type = if service_type.starts_with("_adb-tls-connect.") {
                MdnsServiceType::Connect
            } else if service_type.starts_with("_adb-tls-pairing.") {
                MdnsServiceType::Pairing
            } else {
                return None;
            };
            address.contains(':').then(|| MdnsService {
                instance: instance.to_string(),
                service_type,
                address: address.to_string(),
            })
        })
        .collect()
}

/// The device's mDNS name from `adb pair` output such as
/// `Successfully paired to 192.168.1.5:37000 [guid=adb-1A2B-x7Yz9Q]`.
pub fn parse_paired_guid(output: &str) -> Option<String> {
    let start = output.find("[guid=")? + "[guid=".len();
    let rest = &output[start..];
    let guid = &rest[..rest.find(']')?];
    (!guid.is_empty()).then(|| guid.to_string())
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
                    serial: "emulator-5554".into(),
                    state: "device".into(),
                },
                Device {
                    serial: "192.168.1.5:5555".into(),
                    state: "offline".into(),
                },
            ]
        );
    }

    #[test]
    fn parses_mdns_services() {
        let output = "List of discovered mdns services\nadb-1A2B-x7Yz9Q\t_adb-tls-connect._tcp\t192.168.1.5:41235\nstudio-abc\t_adb-tls-pairing._tcp.\t192.168.1.6:37099\nadb-1A2B\t_adb._tcp\t192.168.1.5:5555\n\n";
        let services = parse_mdns_services(&lines(output));
        assert_eq!(
            services,
            vec![
                MdnsService {
                    instance: "adb-1A2B-x7Yz9Q".into(),
                    service_type: MdnsServiceType::Connect,
                    address: "192.168.1.5:41235".into(),
                },
                MdnsService {
                    instance: "studio-abc".into(),
                    service_type: MdnsServiceType::Pairing,
                    address: "192.168.1.6:37099".into(),
                },
            ]
        );
    }

    #[test]
    fn parses_the_guid_of_a_pairing() {
        assert_eq!(
            parse_paired_guid("Successfully paired to 192.168.1.5:37000 [guid=adb-1A2B-x7Yz9Q]\n")
                .as_deref(),
            Some("adb-1A2B-x7Yz9Q")
        );
        assert_eq!(
            parse_paired_guid("Successfully paired to 192.168.1.5:37000"),
            None
        );
        assert_eq!(
            parse_paired_guid("Failed: Wrong password or connection was dropped."),
            None
        );
    }

    #[test]
    fn parses_avds_without_log_lines() {
        let output = "INFO    | Storing crashdata in: /tmp/android/emu-crash.db\nPixel_7_API_34\n\nmy.avd-2\n";
        assert_eq!(parse_avds(&lines(output)), ["Pixel_7_API_34", "my.avd-2"]);
    }

    #[test]
    fn parses_proxies() {
        let output =
            "emulator-5554 tcp:8080 tcp:80\nemulator-5554 localabstract:x jdwp:12\ngarbage\n";
        let proxies = parse_proxies(ProxyType::Forward, &lines(output));
        assert_eq!(proxies.len(), 2);
        assert_eq!(proxies[0].0, "emulator-5554");
        assert_eq!(proxies[0].1.from.socket_type, SocketType::Tcp);
        assert_eq!(proxies[1].1.to.name, "12");
    }
}
