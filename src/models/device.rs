//! A device reported by adb.

use super::Connection;

/// A device as listed by `adb devices`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    /// The full serial, e.g. `emulator-5554`, `192.168.1.5:5555`, or for a
    /// device that adb connected by itself over mDNS
    /// `adb-1A2B3C4D-x7Yz9Q._adb-tls-connect._tcp`.
    pub serial: String,
    /// The state adb reports, e.g. `device`, `offline` or `unauthorized`.
    pub state: String,
}

impl Device {
    /// Whether adb can run commands on the device (state `device`).
    pub fn is_ready(&self) -> bool {
        self.state == "device"
    }

    /// Whether a saved connection address refers to this device.
    ///
    /// An address without a port means port 5555, as it does for `adb connect`
    /// and `adb disconnect`, so the shown status matches what Disconnect acts on.
    pub fn matches_address(&self, address: &str) -> bool {
        let address = address.trim();
        !address.is_empty()
            && (address == self.serial
                || (!address.contains(':') && self.serial == format!("{address}:5555")))
    }

    /// Whether this device is the one a saved connection refers to: by its
    /// address, or by its mDNS name when adb connected it by itself.
    pub fn matches_connection(&self, connection: &Connection) -> bool {
        self.matches_address(&connection.address)
            || (!connection.mdns_name.is_empty()
                && self
                    .serial
                    .strip_prefix(connection.mdns_name.as_str())
                    .is_some_and(|rest| rest.starts_with('.')))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(serial: &str) -> Device {
        Device {
            serial: serial.into(),
            state: "device".into(),
        }
    }

    #[test]
    fn device_matches_address_with_and_without_port() {
        let device = device("192.168.1.5:5555");
        assert!(device.matches_address("192.168.1.5"));
        assert!(device.matches_address("192.168.1.5:5555"));
        assert!(!device.matches_address("192.168.1.6"));
        assert!(!device.matches_address(""));
    }

    #[test]
    fn address_without_port_only_matches_port_5555() {
        let device = device("192.168.1.5:5556");
        assert!(!device.matches_address("192.168.1.5"));
        assert!(device.matches_address("192.168.1.5:5556"));
    }

    #[test]
    fn connection_matches_by_address_or_mdns_name() {
        let connection = Connection {
            address: "192.168.1.5:41235".into(),
            mdns_name: "adb-1A2B-x7Yz9Q".into(),
            ..Connection::default()
        };
        assert!(device("192.168.1.5:41235").matches_connection(&connection));
        assert!(device("adb-1A2B-x7Yz9Q._adb-tls-connect._tcp").matches_connection(&connection));
        assert!(!device("192.168.1.5:39001").matches_connection(&connection));
        assert!(!device("adb-1A2B-x7Yz9Qa._adb-tls-connect._tcp").matches_connection(&connection));
        assert!(!device("emulator-5554").matches_connection(&Connection::default()));
    }
}
