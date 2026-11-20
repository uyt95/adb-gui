//! Persistence of user data as a JSON file in the XDG data directory.

use std::cell::Cell;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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

    /// Raises the id counter to the highest id in use, so that new records never
    /// reuse an id, and gives a fresh id to every record without one (id 0).
    /// Needed when ids or the counter are missing or too low in a loaded file,
    /// for example after a hand edit or an import.
    pub fn repair_next_id(&mut self) {
        let highest = (self.automations.iter().map(Record::id))
            .chain(self.connections.iter().map(Record::id))
            .chain(self.emulators.iter().map(Record::id))
            .chain(self.proxies.iter().map(Record::id))
            .max()
            .unwrap_or(0);
        self.next_id = self.next_id.max(highest);
        fn assign_missing<T: Record>(next_id: &mut u64, items: &mut [T]) {
            for item in items.iter_mut().filter(|i| i.id() == 0) {
                *next_id += 1;
                item.set_id(*next_id);
            }
        }
        assign_missing(&mut self.next_id, &mut self.automations);
        assign_missing(&mut self.next_id, &mut self.connections);
        assign_missing(&mut self.next_id, &mut self.emulators);
        assign_missing(&mut self.next_id, &mut self.proxies);
    }
}

pub struct Store {
    path: PathBuf,
    /// Set when an unreadable data file could not be moved aside, so that
    /// saving never overwrites the user's data.
    read_only: Cell<bool>,
}

/// Why the data file could not be loaded, and what was done with it.
#[derive(Debug, PartialEq, Eq)]
pub enum LoadProblem {
    /// The file was moved to this backup path and the app starts empty.
    BackedUp(PathBuf),
    /// The file could not be moved aside, so changes will not be saved.
    NotSaving,
}

impl Store {
    pub fn new(path: PathBuf) -> Store {
        Store {
            path,
            read_only: Cell::new(false),
        }
    }

    pub fn default_path() -> PathBuf {
        gtk::glib::user_data_dir().join("adb-gui").join("data.json")
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Loads the database. A missing file gives an empty database. A file
    /// that cannot be read or parsed is moved to a backup first, so that the
    /// next save does not overwrite it.
    pub fn load(&self) -> (Database, Option<LoadProblem>) {
        let error = match fs::read_to_string(&self.path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(mut database) => {
                    Database::repair_next_id(&mut database);
                    return (database, None);
                }
                Err(error) => error.to_string(),
            },
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return (Database::default(), None);
            }
            Err(error) => error.to_string(),
        };
        eprintln!("Failed to load {}: {error}", self.path.display());
        (Database::default(), Some(self.back_up()))
    }

    fn back_up(&self) -> LoadProblem {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let mut name = self.path.file_name().unwrap_or_default().to_os_string();
        name.push(format!(".bak-{seconds}"));
        let backup = self.path.with_file_name(name);
        match fs::rename(&self.path, &backup) {
            Ok(()) => LoadProblem::BackedUp(backup),
            Err(error) => {
                eprintln!("Failed to back up {}: {error}", self.path.display());
                self.read_only.set(true);
                LoadProblem::NotSaving
            }
        }
    }

    pub fn save(&self, database: &Database) {
        if self.read_only.get() {
            return;
        }
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
    fn load_raises_a_missing_id_counter() {
        let dir = std::env::temp_dir().join(format!("adb-gui-ids-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("data.json");
        fs::write(
            &path,
            r#"{"connections":[{"id":7,"name":"a","address":"1"}]}"#,
        )
        .unwrap();
        let (mut db, problem) = Store::new(path).load();
        assert_eq!(problem, None);
        db.put_connection(Connection {
            id: 0,
            name: "b".into(),
            address: "2".into(),
        });
        assert_eq!(db.connections[1].id, 8);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_gives_records_without_an_id_a_fresh_one() {
        let dir = std::env::temp_dir().join(format!("adb-gui-no-ids-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("data.json");
        fs::write(
            &path,
            r#"{"next_id":3,"connections":[{"name":"a","address":"1"},{"id":3,"name":"b","address":"2"},{"name":"c","address":"3"}]}"#,
        )
        .unwrap();
        let (db, problem) = Store::new(path).load();
        assert_eq!(problem, None);
        let ids: Vec<u64> = db.connections.iter().map(|c| c.id).collect();
        assert_eq!(ids, [4, 3, 5]);
        assert_eq!(db.next_id, 5);
        fs::remove_dir_all(&dir).unwrap();
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
        let (loaded, problem) = store.load();
        assert_eq!(problem, None);
        assert_eq!(loaded.settings.adb_path, "/usr/bin/adb");
        assert_eq!(loaded.automations[0].commands, db.automations[0].commands);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn missing_file_loads_empty() {
        let store = Store::new(std::env::temp_dir().join("adb-gui-test-missing/data.json"));
        let (db, problem) = store.load();
        assert!(db.connections.is_empty());
        assert_eq!(problem, None);
    }

    #[test]
    fn missing_nested_fields_use_defaults() {
        let json = r#"{
            "settings": {"adb_path": "/usr/bin/adb"},
            "emulators": [{"id": 1, "name": "a", "params": {}}],
            "proxies": [{"id": 2, "type": "forward",
                "from": {"type": "tcp"}, "to": {"type": "tcp", "name": "80"}}],
            "automations": [{"name": "x", "commands": [
                {"type": "adb", "operation": {"type": "reboot"}}]}]
        }"#;
        let db: Database = serde_json::from_str(json).unwrap();
        assert_eq!(db.settings.adb_path, "/usr/bin/adb");
        assert_eq!(db.settings.screenshot_path, "");
        assert!(!db.emulators[0].params.no_snapshot_load);
        assert_eq!(db.proxies[0].from.name, "");
        assert!(db.automations[0].commands[0].operation.arguments.is_empty());
    }

    #[test]
    fn unreadable_file_is_backed_up_not_overwritten() {
        let dir = std::env::temp_dir().join(format!("adb-gui-test-bak-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("data.json");
        fs::write(&path, "{ not json").unwrap();
        let store = Store::new(path.clone());
        let (db, problem) = store.load();
        assert!(db.connections.is_empty());
        let Some(LoadProblem::BackedUp(backup)) = problem else {
            panic!("expected a backup, got {problem:?}");
        };
        assert_eq!(fs::read_to_string(&backup).unwrap(), "{ not json");
        assert!(!path.exists());
        fs::remove_dir_all(dir).unwrap();
    }
}
