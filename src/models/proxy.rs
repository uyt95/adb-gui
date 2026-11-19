//! Forward and reverse proxies (`adb forward` / `adb reverse`).

use serde::{Deserialize, Serialize};

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
    #[serde(default)]
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
    #[serde(default)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
