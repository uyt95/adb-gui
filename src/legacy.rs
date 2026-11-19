//! One-time import of the data saved by the old Flutter version of the app.
//!
//! The Flutter app stored its data with Isar 3 in `default.isar` in the XDG
//! documents directory. That file is a libmdbx database: every collection is a
//! named table keyed by object id, and every value is an object in Isar's
//! binary format. The `_info` table holds the JSON schema of each collection,
//! which gives the property order and types needed to decode the objects.
//!
//! The file is only read, never written, so the old app keeps working.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::models::{
    Automation, Command, Connection, Emulator, EmulatorParams, Proxy, ProxyType, SettingKey,
    Socket, SocketType,
};
use crate::store::Database;

type Result<T> = std::result::Result<T, String>;

/// Where the Flutter app kept its database, if it exists. That is
/// `getApplicationDocumentsDirectory()`: the XDG documents directory, or the
/// home directory when no documents directory is configured.
pub fn find() -> Option<PathBuf> {
    let home = gtk::glib::home_dir();
    gtk::glib::user_special_dir(gtk::glib::UserDirectory::Documents)
        .into_iter()
        .chain([home.join("Documents"), home])
        .map(|dir| dir.join("default.isar"))
        .find(|path| path.is_file())
}

/// Reads the old database at `path` and converts it to the new format.
pub fn import(path: &Path) -> Result<Database> {
    let data = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    import_bytes(&data)
}

fn import_bytes(data: &[u8]) -> Result<Database> {
    let mdbx = Mdbx::open(data)?;
    let mut schemas = HashMap::new();
    for (_, value) in mdbx.table("_info")? {
        let schema: Schema = serde_json::from_slice(value)
            .map_err(|error| format!("Invalid Isar schema: {error}"))?;
        schemas.insert(schema.name.clone(), schema);
    }
    let isar = Isar { mdbx, schemas };

    let mut db = Database::default();
    for object in isar.collection("Connection")? {
        db.put_connection(Connection {
            id: 0,
            name: string(&object, "name"),
            address: string(&object, "address"),
        });
    }
    for object in isar.collection("Emulator")? {
        let params = object.get("params").and_then(Value::as_object);
        let flag = |name| {
            params
                .and_then(|p| p.get(name))
                .and_then(Value::as_bool)
                .unwrap_or(false)
        };
        db.put_emulator(Emulator {
            id: 0,
            name: string(&object, "name"),
            params: EmulatorParams {
                no_snapshot_load: flag("noSnapshotLoad"),
                writeable_system: flag("writeableSystem"),
            },
        });
    }
    for object in isar.collection("Proxy")? {
        if let Some(proxy) = proxy(&object) {
            db.put_proxy(proxy);
        }
    }
    for object in isar.collection("Automation")? {
        let commands = object
            .get("commands")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|command| Command::deserialize(command).ok())
            .collect();
        db.put_automation(Automation {
            id: 0,
            name: string(&object, "name"),
            commands,
        });
    }
    // Settings are keyed by the index of the Dart `SettingKey` enum.
    for (id, object) in isar.collection_with_ids("Setting")? {
        let key = usize::try_from(id)
            .ok()
            .and_then(|i| SettingKey::ALL.get(i));
        let value = string(&object, "value");
        if let Some(&key) = key
            && !value.is_empty()
        {
            db.settings.set(key, value);
        }
    }
    Ok(db)
}

fn string(object: &Map<String, Value>, name: &str) -> String {
    object
        .get(name)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn proxy(object: &Map<String, Value>) -> Option<Proxy> {
    let proxy_type = match object.get("type")?.as_str()? {
        "forward" => ProxyType::Forward,
        "reverse" => ProxyType::Reverse,
        _ => return None,
    };
    // The Dart enum names are camel case versions of the adb socket names.
    let socket = |name| {
        let socket = object.get(name)?.as_object()?;
        let type_name = socket.get("type")?.as_str()?.to_lowercase();
        Some(Socket {
            socket_type: SocketType::from_type_name(&type_name)?,
            name: socket.get("name")?.as_str()?.to_string(),
        })
    };
    Some(Proxy {
        id: 0,
        proxy_type,
        from: socket("from")?,
        to: socket("to")?,
    })
}

// --- Isar objects -----------------------------------------------------------

#[derive(Deserialize)]
struct Schema {
    name: String,
    properties: Vec<Property>,
}

#[derive(Deserialize)]
struct Property {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    target: Option<String>,
}

const NULL_INT: i32 = i32::MIN;
const NULL_LONG: i64 = i64::MIN;
const TRUE_BOOL: u8 = 2;
const NULL_BOOL: u8 = 0;

struct Isar<'a> {
    mdbx: Mdbx<'a>,
    schemas: HashMap<String, Schema>,
}

impl Isar<'_> {
    fn collection(&self, name: &str) -> Result<Vec<Map<String, Value>>> {
        Ok(self
            .collection_with_ids(name)?
            .into_iter()
            .map(|(_, object)| object)
            .collect())
    }

    fn collection_with_ids(&self, name: &str) -> Result<Vec<(i64, Map<String, Value>)>> {
        let Some(schema) = self.schemas.get(name) else {
            return Ok(Vec::new());
        };
        self.mdbx
            .table(name)?
            .into_iter()
            .map(|(key, value)| {
                // Ids are stored as little endian with the sign bit flipped.
                let key: [u8; 8] = key.try_into().map_err(|_| format!("Invalid {name} id"))?;
                let id = (u64::from_le_bytes(key) ^ (1 << 63)) as i64;
                Ok((id, self.object(value, schema)?))
            })
            .collect()
    }

    /// Decodes an object: a u16 static size, the fixed size properties in
    /// schema order, then the variable size data that they point to.
    fn object(&self, data: &[u8], schema: &Schema) -> Result<Map<String, Value>> {
        let static_size = usize::from(u16::from_le_bytes(bytes(data, 0)?));
        let mut object = Map::new();
        let mut offset = 2;
        for property in &schema.properties {
            let size = match property.kind.as_str() {
                "Bool" | "Byte" => 1,
                "Int" | "Float" => 4,
                "Long" | "Double" | "DateTime" => 8,
                _ => 3,
            };
            // Properties added after the object was written are missing.
            if offset + size > static_size {
                break;
            }
            let value = self.property(data, offset, property)?;
            object.insert(property.name.clone(), value);
            offset += size;
        }
        Ok(object)
    }

    fn property(&self, data: &[u8], offset: usize, property: &Property) -> Result<Value> {
        let target = || {
            property
                .target
                .as_ref()
                .and_then(|target| self.schemas.get(target))
                .ok_or_else(|| format!("Unknown Isar schema for {}", property.name))
        };
        Ok(match property.kind.as_str() {
            "Bool" => match bytes::<1>(data, offset)?[0] {
                NULL_BOOL => Value::Null,
                value => Value::Bool(value == TRUE_BOOL),
            },
            "Byte" => bytes::<1>(data, offset)?[0].into(),
            "Int" => match i32::from_le_bytes(bytes(data, offset)?) {
                NULL_INT => Value::Null,
                value => value.into(),
            },
            "Long" => match i64::from_le_bytes(bytes(data, offset)?) {
                NULL_LONG => Value::Null,
                value => value.into(),
            },
            "String" => match dynamic(data, offset)? {
                Some(content) => String::from_utf8_lossy(content).into(),
                None => Value::Null,
            },
            "Object" => match dynamic(data, offset)? {
                Some(content) => self.object(content, target()?)?.into(),
                None => Value::Null,
            },
            "StringList" => match list(data, offset)? {
                Some(items) => items
                    .into_iter()
                    .map(|item| item.map_or(Value::Null, |s| String::from_utf8_lossy(s).into()))
                    .collect(),
                None => Value::Null,
            },
            "ObjectList" => match list(data, offset)? {
                Some(items) => {
                    let schema = target()?;
                    items
                        .into_iter()
                        .map(|item| match item {
                            Some(content) => self.object(content, schema).map(Value::from),
                            None => Ok(Value::Null),
                        })
                        .collect::<Result<_>>()?
                }
                None => Value::Null,
            },
            // Not used by any of the old app's collections.
            _ => Value::Null,
        })
    }
}

fn bytes<const N: usize>(data: &[u8], offset: usize) -> Result<[u8; N]> {
    slice(data, offset, N).map(|s| s.try_into().unwrap())
}

fn slice(data: &[u8], offset: usize, len: usize) -> Result<&[u8]> {
    offset
        .checked_add(len)
        .and_then(|end| data.get(offset..end))
        .ok_or_else(|| "Isar object is truncated".to_string())
}

fn u24(data: &[u8], offset: usize) -> Result<usize> {
    let [a, b, c] = bytes(data, offset)?;
    Ok(usize::from(a) | usize::from(b) << 8 | usize::from(c) << 16)
}

/// A string or embedded object: a u24 offset to a u24 length and the content.
fn dynamic(data: &[u8], offset: usize) -> Result<Option<&[u8]>> {
    match u24(data, offset)? {
        0 => Ok(None),
        start => slice(data, start + 3, u24(data, start)?).map(Some),
    }
}

/// A list: a u24 offset to a u24 count, a u24 size per item (0 for null,
/// otherwise the length plus one) and the items one after the other.
fn list(data: &[u8], offset: usize) -> Result<Option<Vec<Option<&[u8]>>>> {
    let start = match u24(data, offset)? {
        0 => return Ok(None),
        start => start,
    };
    let count = u24(data, start)?;
    let mut content = start + 3 + count * 3;
    let mut items = Vec::with_capacity(count.min(data.len()));
    for i in 0..count {
        match u24(data, start + 3 + i * 3)? {
            0 => items.push(None),
            size => {
                items.push(Some(slice(data, content, size - 1)?));
                content += size - 1;
            }
        }
    }
    Ok(Some(items))
}

// --- libmdbx ----------------------------------------------------------------
//
// A minimal read-only reader for the libmdbx 0.12 file format: just enough to
// walk the B-trees of named tables. Layouts follow `MDBX_meta`, `MDBX_page`
// and `MDBX_node` in libmdbx' `internals.h`.

const PAGE_HEADER: usize = 20;
const META_COUNT: usize = 3;
const MDBX_MAGIC: u64 = 0x59659DBDEF4C11;
const MDBX_DATA_VERSION: u64 = 3;
const P_BRANCH: u16 = 0x01;
const P_LEAF: u16 = 0x02;
const P_LEAF2: u16 = 0x20;
const F_BIGDATA: u8 = 0x01;
const F_SUBDATA: u8 = 0x02;
const F_DUPDATA: u8 = 0x04;
const P_INVALID: u32 = u32::MAX;
const MAX_DEPTH: usize = 32;

struct Mdbx<'a> {
    data: &'a [u8],
    page_size: usize,
    main_root: u32,
}

struct Node<'a> {
    flags: u8,
    key: &'a [u8],
    value: &'a [u8],
}

impl<'a> Mdbx<'a> {
    fn open(data: &'a [u8]) -> Result<Mdbx<'a>> {
        let not_isar = || "Not an Isar database".to_string();
        // The page size is stored in every meta page, so read it from the first.
        let page_size = u32::from_le_bytes(bytes(data, PAGE_HEADER + 44).map_err(|_| not_isar())?);
        let page_size = page_size as usize;
        if !page_size.is_power_of_two() || !(256..=65536).contains(&page_size) {
            return Err(not_isar());
        }
        // The newest meta page whose two transaction ids agree is the current one.
        let mut current = None;
        for i in 0..META_COUNT {
            let meta = i * page_size + PAGE_HEADER;
            let Ok(magic) = bytes(data, meta).map(u64::from_le_bytes) else {
                continue;
            };
            let txnid_a = u64::from_le_bytes(bytes(data, meta + 8)?);
            let txnid_b = u64::from_le_bytes(bytes(data, meta + 176)?);
            let main_root = u32::from_le_bytes(bytes(data, meta + 88 + 8)?);
            if magic == MDBX_MAGIC << 8 | MDBX_DATA_VERSION
                && txnid_a == txnid_b
                && current.is_none_or(|(txnid, _)| txnid_a > txnid)
            {
                current = Some((txnid_a, main_root));
            }
        }
        let (_, main_root) = current.ok_or_else(not_isar)?;
        Ok(Mdbx {
            data,
            page_size,
            main_root,
        })
    }

    /// All key/value pairs of a named table, or none if it does not exist.
    fn table(&self, name: &str) -> Result<Vec<(&'a [u8], &'a [u8])>> {
        let mut main = Vec::new();
        self.walk(self.main_root, 0, &mut main)?;
        let Some(table) = main
            .iter()
            .find(|node| node.key == name.as_bytes() && node.flags & F_SUBDATA != 0)
        else {
            return Ok(Vec::new());
        };
        // The value is an `MDBX_db` with the root page number at offset 8.
        let root = u32::from_le_bytes(bytes(table.value, 8)?);
        let mut nodes = Vec::new();
        self.walk(root, 0, &mut nodes)?;
        Ok(nodes.into_iter().map(|n| (n.key, n.value)).collect())
    }

    fn walk(&self, pgno: u32, depth: usize, out: &mut Vec<Node<'a>>) -> Result<()> {
        if pgno == P_INVALID {
            return Ok(());
        }
        if depth > MAX_DEPTH {
            return Err("Isar database is corrupt".into());
        }
        let page = self.page(pgno)?;
        let flags = u16::from_le_bytes(bytes(page, 10)?);
        let count = usize::from(u16::from_le_bytes(bytes(page, 12)?)) / 2;
        if flags & P_LEAF2 != 0 {
            return Err("Unsupported Isar table layout".into());
        }
        for i in 0..count {
            let node =
                PAGE_HEADER + usize::from(u16::from_le_bytes(bytes(page, PAGE_HEADER + i * 2)?));
            let size = u32::from_le_bytes(bytes(page, node)?);
            let node_flags = bytes::<1>(page, node + 4)?[0];
            let key_size = usize::from(u16::from_le_bytes(bytes(page, node + 6)?));
            let key = slice(page, node + 8, key_size)?;
            if flags & P_BRANCH != 0 {
                self.walk(size, depth + 1, out)?;
            } else if flags & P_LEAF != 0 {
                if node_flags & F_DUPDATA != 0 {
                    return Err("Unsupported Isar table layout".into());
                }
                let value = if node_flags & F_BIGDATA != 0 {
                    // The value is on overflow pages that start with a page header.
                    let overflow = u32::from_le_bytes(bytes(page, node + 8 + key_size)?);
                    let start = (overflow as usize)
                        .checked_mul(self.page_size)
                        .ok_or("Isar database is corrupt")?;
                    slice(self.data, start + PAGE_HEADER, size as usize)?
                } else {
                    slice(page, node + 8 + key_size, size as usize)?
                };
                out.push(Node {
                    flags: node_flags,
                    key,
                    value,
                });
            } else {
                return Err("Isar database is corrupt".into());
            }
        }
        Ok(())
    }

    fn page(&self, pgno: u32) -> Result<&'a [u8]> {
        if (pgno as usize) < META_COUNT {
            return Err("Isar database is corrupt".into());
        }
        slice(self.data, pgno as usize * self.page_size, self.page_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ArgumentType, OperationType};

    // Written by the Flutter app's Isar 3.1.0+1 (libmdbx 0.12.4) with the old
    // app's schemas, then truncated after the last used page.
    const SAMPLE: &[u8] = include_bytes!("../tests/fixtures/legacy-sample.isar");
    // 301 connections, so the table has branch pages, one with a 20 kB address
    // that is stored on overflow pages.
    const LARGE: &[u8] = include_bytes!("../tests/fixtures/legacy-large.isar");

    #[test]
    fn imports_all_collections() {
        let db = import_bytes(SAMPLE).unwrap();

        let connections: Vec<_> = db
            .connections
            .iter()
            .map(|c| (c.name.as_str(), c.address.as_str()))
            .collect();
        assert_eq!(
            connections,
            [
                ("Pixel", "192.168.1.5:5555"),
                ("Tablet ünïcode 😀", "10.0.0.2")
            ]
        );

        assert_eq!(db.emulators.len(), 2);
        assert_eq!(db.emulators[0].name, "Pixel_7_API_34");
        assert!(db.emulators[0].params.no_snapshot_load);
        assert!(!db.emulators[0].params.writeable_system);
        assert!(!db.emulators[1].params.no_snapshot_load);
        assert!(db.emulators[1].params.writeable_system);

        let proxies: Vec<_> = db.proxies.iter().map(ToString::to_string).collect();
        assert_eq!(
            proxies,
            [
                "forward tcp:8080 localabstract:chrome_devtools_remote",
                "reverse tcp:3000 tcp:3001"
            ]
        );

        assert_eq!(db.automations.len(), 2);
        let login = &db.automations[0];
        assert_eq!(login.name, "Login");
        assert_eq!(login.commands.len(), 2);
        assert_eq!(
            login.commands[0].arguments(),
            ["shell", "input", "keyboard", "text", "'user@example.com'"]
        );
        let key_code = &login.commands[1].operation;
        assert_eq!(key_code.operation_type, OperationType::KeyCode);
        assert_eq!(key_code.arguments[3].argument_type, ArgumentType::KeyCode);
        assert_eq!(key_code.arguments[3].value, "66");
        assert_eq!(db.automations[1].commands[0].arguments(), ["reboot"]);

        assert_eq!(db.settings.adb_path, "/opt/android/platform-tools/adb");
        assert_eq!(db.settings.emulator_path, "/opt/android/emulator/emulator");
        assert_eq!(db.settings.screenshot_path, "/home/me/Pictures/Screenshots");

        // Imported items get fresh ids from the counter, so new items cannot clash.
        let mut ids: Vec<u64> = (db.automations.iter().map(|i| i.id))
            .chain(db.connections.iter().map(|i| i.id))
            .chain(db.emulators.iter().map(|i| i.id))
            .chain(db.proxies.iter().map(|i| i.id))
            .collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 8);
        let mut repaired = import_bytes(SAMPLE).unwrap();
        repaired.repair_next_id();
        repaired.put_connection(Connection::default());
        let new_id = repaired.connections.iter().map(|c| c.id).max().unwrap();
        assert!(!ids.contains(&new_id));
        assert_eq!(repaired.connections.len(), 3);
    }

    #[test]
    fn imports_multi_page_tables() {
        let db = import_bytes(LARGE).unwrap();
        assert_eq!(db.connections.len(), 301);
        let huge = db.connections.iter().find(|c| c.name == "huge").unwrap();
        assert_eq!(huge.address, "y".repeat(20000));
        assert!(db.connections.iter().any(|c| c.address == "10.0.1.43"));
        assert!(db.automations.is_empty());
    }

    #[test]
    fn rejects_other_files() {
        assert!(import_bytes(b"").is_err());
        assert!(import_bytes(&[0; 16384]).is_err());
        assert!(import_bytes(&SAMPLE[..8192]).is_err());
    }
}
