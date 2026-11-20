//! Shared application state and change notification.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

use gtk::{gio, glib};

use crate::l10n;
use crate::models::{Device, Emulator, EmulatorParams, Proxy, SettingKey};
use crate::store::{Database, LoadProblem, Store};
use crate::tools::{self, Tools};

type Listener = Rc<dyn Fn()>;
/// A queued input send; it starts its worker when called.
type InputJob = Box<dyn FnOnce(&Rc<AppState>)>;

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
    /// Number of the newest device and proxy loads. A result from an older load
    /// arrives too late to be current and is dropped.
    device_load: Cell<u64>,
    proxy_load: Cell<u64>,
    /// Shown once the toast overlay exists.
    load_problem: RefCell<Option<LoadProblem>>,
    /// Key and text sends waiting for the one that is running, so they reach
    /// the device in the order the user made them.
    input_queue: RefCell<VecDeque<InputJob>>,
    input_busy: Cell<bool>,
}

impl AppState {
    pub fn new(store: Store) -> Rc<AppState> {
        let (mut db, load_problem) = store.load();
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
            device_load: Cell::new(0),
            proxy_load: Cell::new(0),
            load_problem: RefCell::new(load_problem),
            input_queue: RefCell::default(),
            input_busy: Cell::new(false),
        })
    }

    pub fn set_toast_overlay(&self, overlay: &adw::ToastOverlay) {
        self.toast_overlay.replace(Some(overlay.clone()));
        if let Some(problem) = self.load_problem.take() {
            self.show_error(&match problem {
                LoadProblem::BackedUp(backup) => l10n::data_backed_up(&backup.to_string_lossy()),
                LoadProblem::NotSaving => l10n::data_not_saving().to_string(),
            });
        }
    }

    pub fn show_error(&self, message: &str) {
        eprintln!("{message}");
        if let Some(overlay) = self.toast_overlay.borrow().as_ref() {
            // Toast titles are parsed as Pango markup, so escape the raw
            // error text: `<` or `&` in it would otherwise blank the toast.
            let text = glib::markup_escape_text(&l10n::error_message(message));
            let toast = adw::Toast::new(&text);
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

    /// Like [`run`](Self::run) for key and text sends to the device. Sends run
    /// one at a time, in the order they were made, because separate adb
    /// processes started together can reach the device in any order.
    pub fn run_input<T: Send + 'static>(
        self: &Rc<Self>,
        work: impl FnOnce(Tools) -> T + Send + 'static,
        done: impl FnOnce(&Rc<AppState>, T) + 'static,
    ) {
        self.input_queue
            .borrow_mut()
            .push_back(Box::new(move |state: &Rc<AppState>| {
                let tools = state.tools();
                let state = state.clone();
                glib::spawn_future_local(async move {
                    match gio::spawn_blocking(move || work(tools)).await {
                        Ok(result) => done(&state, result),
                        Err(_) => state.show_error("worker thread panicked"),
                    }
                    // Also after a panic, so later sends are not stuck.
                    state.input_busy.set(false);
                    state.next_input();
                });
            }));
        self.next_input();
    }

    fn next_input(self: &Rc<Self>) {
        if self.input_busy.get() {
            return;
        }
        let Some(job) = self.input_queue.borrow_mut().pop_front() else {
            return;
        };
        self.input_busy.set(true);
        job(self);
    }

    /// Like [`run`](Self::run) for actions that only report errors.
    pub fn run_action(
        self: &Rc<Self>,
        work: impl FnOnce(Tools) -> tools::Result<()> + Send + 'static,
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
        let load = next_load(&self.device_load);
        self.run(
            |tools| tools.list_devices(),
            move |state, result| {
                if state.device_load.get() != load {
                    return;
                }
                let devices = result.unwrap_or_else(|error| {
                    state.show_error(&error);
                    Vec::new()
                });
                let next = choose_active_device(&devices, state.active_device().as_ref());
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
            |state, result| {
                let names = result.unwrap_or_else(|error| {
                    state.show_error(&error);
                    Vec::new()
                });
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
        let load = next_load(&self.proxy_load);
        let Some(device) = self.active_device() else {
            self.active_proxies.borrow_mut().clear();
            self.notify();
            return;
        };
        self.run(
            move |tools| (device.clone(), tools.list_proxies(&device)),
            move |state, (device, listed)| {
                if state.proxy_load.get() != load || state.active_device().as_ref() != Some(&device)
                {
                    return;
                }
                // Proxies of other devices are saved too, so the page lists them,
                // but only the selected device's proxies count as active.
                let mut unknown: Vec<Proxy> = Vec::new();
                {
                    let db = state.db.borrow();
                    for p in listed.active.iter().chain(&listed.other_devices) {
                        let known = db.proxies.iter().chain(&unknown).any(|s| s.same_mapping(p));
                        if !known {
                            unknown.push(p.clone());
                        }
                    }
                }
                state.active_proxies.replace(listed.active);
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

/// Picks the active device after a reload of the device list.
///
/// The current device is kept while it is listed, with its new state, unless
/// it is not ready (for example `unauthorized` or `offline`) and another
/// device is. Otherwise the first ready device is chosen, or the first device
/// when none is ready.
fn choose_active_device(devices: &[Device], current: Option<&Device>) -> Option<Device> {
    let first_ready = devices.iter().find(|d| d.is_ready());
    let kept = current
        .and_then(|current| devices.iter().find(|d| d.serial == current.serial))
        .filter(|d| d.is_ready() || first_ready.is_none());
    kept.or(first_ready).or(devices.first()).cloned()
}

/// Starts a new load: bumps the counter and returns the new load's number.
fn next_load(counter: &Cell<u64>) -> u64 {
    let load = counter.get() + 1;
    counter.set(load);
    load
}

fn fill_default_settings(db: &mut Database) {
    let home = glib::home_dir();
    let home = home.to_string_lossy();
    let settings = &mut db.settings;
    if settings.adb_path.is_empty() {
        settings.adb_path = tools::find_executable("adb")
            .unwrap_or_else(|| format!("{home}/Android/Sdk/platform-tools/adb"));
    }
    if settings.emulator_path.is_empty() {
        settings.emulator_path = tools::find_executable("emulator")
            .unwrap_or_else(|| format!("{home}/Android/Sdk/emulator/emulator"));
    }
    if settings.screenshot_path.is_empty() {
        settings.screenshot_path = format!("{home}/Pictures/Screenshots");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(serial: &str, state: &str) -> Device {
        Device {
            serial: serial.into(),
            state: state.into(),
        }
    }

    #[test]
    fn active_device_prefers_ready_devices() {
        let offline = device("192.168.1.5:5555", "offline");
        let ready = device("emulator-5554", "device");
        let devices = vec![offline.clone(), ready.clone()];
        assert_eq!(choose_active_device(&devices, None), Some(ready.clone()));
        assert_eq!(
            choose_active_device(&devices, Some(&offline)),
            Some(ready.clone())
        );
        let other = device("emulator-5556", "device");
        let devices = vec![ready.clone(), other.clone()];
        assert_eq!(choose_active_device(&devices, Some(&other)), Some(other));
    }

    #[test]
    fn active_device_falls_back_to_unready_devices() {
        let booting = device("emulator-5554", "offline");
        let devices = vec![device("emulator-5554", "device")];
        assert_eq!(
            choose_active_device(&devices, Some(&booting)),
            Some(devices[0].clone())
        );
        let devices = vec![booting.clone()];
        assert_eq!(choose_active_device(&devices, None), Some(booting));
        assert_eq!(choose_active_device(&[], None), None);
    }
}
