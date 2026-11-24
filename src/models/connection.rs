//! Saved remote connections.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Connection {
    pub id: u64,
    pub name: String,
    /// `host[:port]` for `adb connect`. For a wireless debugging device this
    /// is the last address it was reached at; its port changes, so connecting
    /// looks the current one up by [`mdns_name`](Self::mdns_name) first.
    pub address: String,
    /// The mDNS instance name of a device paired for wireless debugging
    /// (Android 11+), e.g. `adb-1A2B3C4D-x7Yz9Q`. It stays the same when the
    /// device's address or port changes. Empty for devices reached with
    /// classic `adb tcpip` debugging, which only have an address.
    pub mdns_name: String,
}
