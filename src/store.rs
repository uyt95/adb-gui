//! Persistence of user data as a JSON file in the XDG data directory.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::models::{Automation, Connection, Emulator, Proxy, Settings};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Database {
    next_id: u64,
    pub automations: Vec<Automation>,
    pub connections: Vec<Connection>,
    pub emulators: Vec<Emulator>,
    pub proxies: Vec<Proxy>,
    pub settings: Settings,
}

/// Something stored in the database with an auto-assigned id.
pub trait Record {
    fn id(&self) -> u64;
    fn set_id(&mut self, id: u64);
}

macro_rules! record {
    ($($t:ty),*) => {$(
        impl Record for $t {
            fn id(&self) -> u64 { self.id }
            fn set_id(&mut self, id: u64) { self.id = id; }
        }
    )*};
}
record!(Automation, Connection, Emulator, Proxy);

impl Database {
    /// Inserts the record when its id is 0, otherwise replaces the record with that id.
    pub fn put<T: Record>(next_id: &mut u64, items: &mut Vec<T>, mut item: T) {
        if item.id() == 0 {
            *next_id += 1;
            item.set_id(*next_id);
            items.push(item);
        } else if let Some(existing) = items.iter_mut().find(|i| i.id() == item.id()) {
            *existing = item;
        } else {
            *next_id = (*next_id).max(item.id());
            items.push(item);
        }
    }

    pub fn put_automation(&mut self, item: Automation) {
        Self::put(&mut self.next_id, &mut self.automations, item);
        self.automations.sort_by_key(|a| a.name.to_lowercase());
    }

    pub fn put_connection(&mut self, item: Connection) {
        Self::put(&mut self.next_id, &mut self.connections, item);
        self.connections.sort_by_key(|c| c.name.to_lowercase());
    }

    pub fn put_emulator(&mut self, item: Emulator) {
        Self::put(&mut self.next_id, &mut self.emulators, item);
        self.emulators.sort_by_key(|e| e.name.to_lowercase());
    }

    pub fn put_proxy(&mut self, item: Proxy) {
        Self::put(&mut self.next_id, &mut self.proxies, item);
    }
}

pub struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new(path: PathBuf) -> Store {
        Store { path }
    }

    pub fn default_path() -> PathBuf {
        gtk::glib::user_data_dir().join("adb-gui").join("data.json")
    }

    pub fn load(&self) -> Database {
        match fs::read_to_string(&self.path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|error| {
                eprintln!("Failed to parse {}: {error}", self.path.display());
                Database::default()
            }),
            Err(_) => Database::default(),
        }
    }

    pub fn save(&self, database: &Database) {
        if let Err(error) = write_atomic(&self.path, database) {
            eprintln!("Failed to save {}: {error}", self.path.display());
        }
    }
}

fn write_atomic(path: &Path, database: &Database) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(database).map_err(std::io::Error::other)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json)?;
    fs::rename(tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_assigns_ids_and_replaces() {
        let mut db = Database::default();
        db.put_connection(Connection {
            id: 0,
            name: "b".into(),
            address: "1".into(),
        });
        db.put_connection(Connection {
            id: 0,
            name: "A".into(),
            address: "2".into(),
        });
        assert_eq!(db.connections[0].name, "A");
        let mut edited = db.connections[1].clone();
        edited.address = "3".into();
        db.put_connection(edited);
        assert_eq!(db.connections.len(), 2);
        assert_eq!(db.connections[1].address, "3");
        assert_ne!(db.connections[0].id, db.connections[1].id);
    }

    #[test]
    fn save_and_load_round_trip() {
        let dir = std::env::temp_dir().join(format!("adb-gui-test-{}", std::process::id()));
        let store = Store::new(dir.join("data.json"));
        let mut db = Database::default();
        db.settings.adb_path = "/usr/bin/adb".into();
        db.put_automation(Automation {
            id: 0,
            name: "x".into(),
            commands: vec![crate::models::Command::new(crate::models::CommandType::Adb)],
        });
        store.save(&db);
        let loaded = store.load();
        assert_eq!(loaded.settings.adb_path, "/usr/bin/adb");
        assert_eq!(loaded.automations[0].commands, db.automations[0].commands);
        fs::remove_dir_all(dir).unwrap();
    }
}
