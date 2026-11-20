//! A device reported by adb.

/// A device as listed by `adb devices`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    /// The full serial, e.g. `emulator-5554` or `192.168.1.5:5555`.
    pub serial: String,
}

impl Device {
    /// Whether a saved connection address refers to this device.
    ///
    /// An address without a port means port 5555, as it does for `adb connect`
    /// and `adb disconnect`, so the shown status matches what Disconnect acts on.
    pub fn matches_address(&self, address: &str) -> bool {
        let address = address.trim();
        address == self.serial
            || (!address.contains(':') && self.serial == format!("{address}:5555"))
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
    fn address_without_port_only_matches_port_5555() {
        let device = Device {
            serial: "192.168.1.5:5556".into(),
        };
        assert!(!device.matches_address("192.168.1.5"));
        assert!(device.matches_address("192.168.1.5:5556"));
    }
}
