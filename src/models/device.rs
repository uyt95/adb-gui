//! A device reported by adb.

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
}
