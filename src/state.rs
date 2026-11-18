//! Shared application state and change notification.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::{gio, glib};

use crate::adb::{self, Tools};
use crate::l10n;
use crate::models::{Device, Emulator, EmulatorParams, Proxy, SettingKey};
use crate::store::{Database, Store};

type Listener = Rc<dyn Fn()>;

pub struct AppState {
    store: Store,
    pub db: RefCell<Database>,
    pub devices: RefCell<Vec<Device>>,
    pub active_device: RefCell<Option<Device>>,
    /// AVD names reported by `emulator -list-avds`.
    pub available_emulators: RefCell<Vec<String>>,
    /// Proxies currently active on the selected device.
    pub active_proxies: RefCell<Vec<Proxy>>,
    listeners: RefCell<Vec<Listener>>,
    notifying: Cell<bool>,
    pending: Cell<bool>,
    toast_overlay: RefCell<Option<adw::ToastOverlay>>,
}

impl AppState {
    pub fn new(store: Store) -> Rc<AppState> {
        let mut db = store.load();
        fill_default_settings(&mut db);
        store.save(&db);
        Rc::new(AppState {
            store,
            db: RefCell::new(db),
            devices: RefCell::default(),
            active_device: RefCell::default(),
            available_emulators: RefCell::default(),
            active_proxies: RefCell::default(),
            listeners: RefCell::default(),
            notifying: Cell::new(false),
            pending: Cell::new(false),
            toast_overlay: RefCell::default(),
        })
    }

    pub fn set_toast_overlay(&self, overlay: &adw::ToastOverlay) {
        self.toast_overlay.replace(Some(overlay.clone()));
    }

    pub fn show_error(&self, message: &str) {
        eprintln!("{message}");
        if let Some(overlay) = self.toast_overlay.borrow().as_ref() {
            let toast = adw::Toast::new(&l10n::error_message(message));
            toast.set_timeout(5);
            overlay.add_toast(toast);
        }
    }

    /// Registers a callback run after any change to the state.
    pub fn subscribe(&self, listener: impl Fn() + 'static) {
        self.listeners.borrow_mut().push(Rc::new(listener));
    }

    pub fn notify(&self) {
        // A listener may change the state again; run another round afterwards
        // instead of recursing.
        self.pending.set(true);
        if self.notifying.replace(true) {
            return;
        }
        while self.pending.replace(false) {
            let listeners: Vec<Listener> = self.listeners.borrow().clone();
            for listener in listeners {
                listener();
            }
        }
        self.notifying.set(false);
    }

    /// Applies a change to the database, saves it and notifies listeners.
    pub fn update_db(&self, change: impl FnOnce(&mut Database)) {
        {
            let mut db = self.db.borrow_mut();
            change(&mut db);
            self.store.save(&db);
        }
        self.notify();
    }

    pub fn tools(&self) -> Tools {
        let db = self.db.borrow();
        Tools {
            adb: db.settings.adb_path.clone(),
            emulator: db.settings.emulator_path.clone(),
        }
    }

    pub fn active_device(&self) -> Option<Device> {
        self.active_device.borrow().clone()
    }

    pub fn set_active_device(self: &Rc<Self>, device: Option<Device>) {
        if *self.active_device.borrow() == device {
            return;
        }
        self.active_device.replace(device);
        self.active_proxies.borrow_mut().clear();
        self.notify();
        self.load_active_proxies();
    }

    /// Runs `work` on a worker thread with the current tool paths, then `done`
    /// on the main thread with its result.
    pub fn run<T: Send + 'static>(
        self: &Rc<Self>,
        work: impl FnOnce(Tools) -> T + Send + 'static,
        done: impl FnOnce(&Rc<AppState>, T) + 'static,
    ) {
        let tools = self.tools();
        let state = self.clone();
        glib::spawn_future_local(async move {
            match gio::spawn_blocking(move || work(tools)).await {
                Ok(result) => done(&state, result),
                Err(_) => state.show_error("worker thread panicked"),
            }
        });
    }

    /// Like [`run`](Self::run) for actions that only report errors.
    pub fn run_action(
        self: &Rc<Self>,
        work: impl FnOnce(Tools) -> adb::Result<()> + Send + 'static,
        done: impl FnOnce(&Rc<AppState>) + 'static,
    ) {
        self.run(work, move |state, result| {
            if let Err(error) = result {
                state.show_error(&error);
            }
            done(state);
        });
    }

    pub fn load_devices(self: &Rc<Self>) {
        self.run(
            |tools| tools.list_devices(),
            |state, devices| {
                let active = state.active_device();
                let keep = active.filter(|a| devices.contains(a));
                let next = keep.or_else(|| devices.first().cloned());
                state.devices.replace(devices);
                if *state.active_device.borrow() == next {
                    state.notify();
                } else {
                    state.set_active_device(next);
                }
            },
        );
    }

    pub fn load_emulators(self: &Rc<Self>) {
        self.run(
            |tools| tools.list_avds(),
            |state, names| {
                let missing: Vec<String> = {
                    let db = state.db.borrow();
                    names
                        .iter()
                        .filter(|name| !db.emulators.iter().any(|e| &e.name == *name))
                        .cloned()
                        .collect()
                };
                state.available_emulators.replace(names);
                state.update_db(|db| {
                    for name in missing {
                        db.put_emulator(Emulator {
                            id: 0,
                            name,
                            params: EmulatorParams::default(),
                        });
                    }
                });
            },
        );
    }

    /// Saved emulators that `emulator -list-avds` currently reports.
    pub fn emulators(&self) -> Vec<Emulator> {
        let available = self.available_emulators.borrow();
        self.db
            .borrow()
            .emulators
            .iter()
            .filter(|e| available.contains(&e.name))
            .cloned()
            .collect()
    }

    /// Reloads the active proxies of the selected device and saves unknown ones.
    pub fn load_active_proxies(self: &Rc<Self>) {
        let Some(device) = self.active_device() else {
            self.active_proxies.borrow_mut().clear();
            self.notify();
            return;
        };
        self.run(
            move |tools| (device.clone(), tools.list_active_proxies(&device)),
            |state, (device, proxies)| {
                if state.active_device().as_ref() != Some(&device) {
                    return;
                }
                let unknown: Vec<Proxy> = {
                    let db = state.db.borrow();
                    proxies
                        .iter()
                        .filter(|p| !db.proxies.iter().any(|s| s.same_mapping(p)))
                        .cloned()
                        .collect()
                };
                state.active_proxies.replace(proxies);
                state.update_db(|db| unknown.into_iter().for_each(|p| db.put_proxy(p)));
            },
        );
    }

    pub fn is_proxy_active(&self, proxy: &Proxy) -> bool {
        self.active_proxies
            .borrow()
            .iter()
            .any(|p| p.same_mapping(proxy))
    }

    pub fn setting(&self, key: SettingKey) -> String {
        self.db.borrow().settings.get(key).to_string()
    }
}

fn fill_default_settings(db: &mut Database) {
    let home = glib::home_dir();
    let home = home.to_string_lossy();
    let settings = &mut db.settings;
    if settings.adb_path.is_empty() {
        settings.adb_path = adb::find_executable("adb")
            .unwrap_or_else(|| format!("{home}/Android/Sdk/platform-tools/adb"));
    }
    if settings.emulator_path.is_empty() {
        settings.emulator_path = adb::find_executable("emulator")
            .unwrap_or_else(|| format!("{home}/Android/Sdk/emulator/emulator"));
    }
    if settings.screenshot_path.is_empty() {
        settings.screenshot_path = format!("{home}/Pictures/Screenshots");
    }
}
