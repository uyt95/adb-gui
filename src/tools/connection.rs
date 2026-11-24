//! Connecting to devices over the network: classic `adb connect`, and
//! pairing and connecting for wireless debugging (Android 11+), where a
//! device is found over mDNS because its port changes.

use std::time::Duration;

use super::parse::{parse_mdns_services, parse_paired_guid};
use super::{Result, Tools, strings};
use crate::l10n;
use crate::models::{Connection, Device};

/// A service that a device advertises over mDNS; see [`Tools::mdns_services`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MdnsService {
    /// The instance name. For a connect service this is the device's stable
    /// mDNS name, e.g. `adb-1A2B3C4D-x7Yz9Q`; for a pairing service started
    /// from a QR code it is the name in that code.
    pub instance: String,
    pub service_type: MdnsServiceType,
    /// `ip:port` of the service.
    pub address: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MdnsServiceType {
    /// `_adb-tls-connect._tcp`: a device with wireless debugging turned on.
    Connect,
    /// `_adb-tls-pairing._tcp`: a device showing a pairing code or a scanned QR code.
    Pairing,
}

/// The result of [`Tools::pair_and_connect`].
#[derive(Clone, Debug)]
pub struct Paired {
    /// The new connection, named after the device model when it could be read.
    pub connection: Connection,
    /// Whether the device's connect service was found after pairing. When
    /// it was not, no connect was tried: without its port, adb would try
    /// port 5555, which wireless debugging never uses.
    pub found: bool,
    /// Why connecting failed after pairing succeeded. The connection is
    /// still worth saving: Connect looks the device up again later.
    pub connect_error: Option<String>,
}

/// The host part of `host[:port]`.
fn address_host(address: &str) -> &str {
    let address = address.trim();
    match address.rsplit_once(':') {
        Some((host, port)) if !port.is_empty() && port.bytes().all(|b| b.is_ascii_digit()) => host,
        _ => address,
    }
}

/// How many times, a second apart, [`Tools::pair_and_connect`] looks for the
/// connect service of a device it just paired with.
const PAIRED_LOOKUP_ATTEMPTS: usize = 6;

impl Tools {
    pub fn connect(&self, address: &str) -> Result<()> {
        self.adb(
            None,
            strings(&["connect", address]),
            &["(already )?connected to .*"],
        )
        .map(drop)
    }

    /// Connects to a saved connection and returns it with the address it was
    /// reached at. A wireless debugging device gets a new port each time
    /// debugging starts, so its current address is looked up over mDNS
    /// first; the saved address is the fallback.
    pub fn connect_saved(&self, connection: &Connection) -> Result<Connection> {
        let mut reached = connection.clone();
        if !connection.mdns_name.is_empty()
            && let Some(service) = self.mdns_services().into_iter().find(|s| {
                s.service_type == MdnsServiceType::Connect && s.instance == connection.mdns_name
            })
        {
            reached.address = service.address;
        }
        if reached.address.trim().is_empty() {
            return Err(l10n::error_device_not_found().to_string());
        }
        // adb connects a paired device by itself over mDNS, under its mDNS
        // name. Connecting again by ip:port would list the device twice.
        if self.connected_device(&reached).is_none() {
            self.connect(&reached.address)?;
        }
        Ok(reached)
    }

    /// The device adb already has ready for a saved connection, listed under
    /// its address or its mDNS name.
    fn connected_device(&self, connection: &Connection) -> Option<Device> {
        self.list_devices()
            .ok()?
            .into_iter()
            .find(|d| d.is_ready() && d.matches_connection(connection))
    }

    /// Services that devices on the network advertise for wireless
    /// debugging. Gives none when adb has no mDNS support or fails.
    pub fn mdns_services(&self) -> Vec<MdnsService> {
        parse_mdns_services(&self.adb_lines(None, &["mdns", "services"]))
    }

    /// Pairs with a device for wireless debugging, using the `ip:port` and
    /// code (or QR password) of its pairing service, then connects to it.
    pub fn pair_and_connect(&self, address: &str, code: &str) -> Result<Paired> {
        let output = self.adb(
            None,
            strings(&["pair", address, code]),
            &["^Successfully paired to "],
        )?;
        let host = address_host(address);
        let mut connection = Connection {
            address: host.to_string(),
            mdns_name: parse_paired_guid(&output).unwrap_or_default(),
            ..Connection::default()
        };
        // The device starts advertising its connect service, with a port of
        // its own, once pairing is done. That can take a moment.
        let mut found = false;
        for attempt in 0..PAIRED_LOOKUP_ATTEMPTS {
            if attempt > 0 {
                std::thread::sleep(Duration::from_secs(1));
            }
            let service = self.mdns_services().into_iter().find(|s| {
                s.service_type == MdnsServiceType::Connect
                    && if connection.mdns_name.is_empty() {
                        address_host(&s.address) == host
                    } else {
                        s.instance == connection.mdns_name
                    }
            });
            if let Some(service) = service {
                connection.mdns_name = service.instance;
                connection.address = service.address;
                found = true;
                break;
            }
        }
        // Without the connect service there is no port to connect to, so
        // leave connecting to the Connect button. connect_saved looks the
        // port up once more, so keep the address it reached.
        let connect_error = if found {
            match self.connect_saved(&connection) {
                Ok(reached) => {
                    connection.address = reached.address;
                    None
                }
                Err(error) => Some(error),
            }
        } else {
            None
        };
        if found && connect_error.is_none() {
            // Read the model through the entry adb has for the device, which
            // is the mDNS name when adb connected it by itself.
            let device = self.connected_device(&connection).unwrap_or(Device {
                serial: connection.address.clone(),
                state: "device".into(),
            });
            connection.name = self
                .adb_lines(Some(&device), &["shell", "getprop", "ro.product.model"])
                .into_iter()
                .map(|line| line.trim().to_string())
                .find(|line| !line.is_empty())
                .unwrap_or_default();
        }
        if connection.name.is_empty() {
            connection.name = if connection.mdns_name.is_empty() {
                connection.address.clone()
            } else {
                connection.mdns_name.clone()
            };
        }
        Ok(Paired {
            connection,
            found,
            connect_error,
        })
    }

    /// Disconnects every entry adb has for a saved connection. One device can
    /// be listed twice: under its ip:port and under its mDNS name.
    pub fn disconnect_connection(&self, connection: &Connection) -> Result<()> {
        let mut result = Ok(());
        for device in self
            .list_devices()?
            .into_iter()
            .filter(|d| d.matches_connection(connection))
        {
            if let Err(error) = self.disconnect(&device.serial)
                && result.is_ok()
            {
                result = Err(error);
            }
        }
        result
    }

    /// Disconnects a device by its serial or address.
    pub fn disconnect(&self, address: &str) -> Result<()> {
        self.adb(
            None,
            strings(&["disconnect", address]),
            &["disconnected .*", "error: no such device '.*'"],
        )
        .map(drop)
    }
}

#[cfg(test)]
mod tests {
    use super::address_host;

    #[test]
    fn address_host_drops_only_a_port() {
        assert_eq!(address_host("192.168.1.5:41235"), "192.168.1.5");
        assert_eq!(address_host(" 192.168.1.5 "), "192.168.1.5");
        assert_eq!(address_host("tv.local:"), "tv.local:");
        assert_eq!(address_host("[fe80::1]:5555"), "[fe80::1]");
    }
}
