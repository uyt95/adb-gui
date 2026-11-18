//! ADB GUI: a GTK user interface on top of adb (Android Debug Bridge).

mod adb;
mod keycodes;
mod l10n;
mod models;
mod state;
mod store;
mod ui;

use adw::prelude::*;
use gtk::glib;

use crate::state::AppState;
use crate::store::Store;

const APP_ID: &str = "io.github.uyt95.AdbGui";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| gtk::Window::set_default_icon_name(APP_ID));
    app.connect_activate(|app| {
        if let Some(window) = app.active_window() {
            window.present();
            return;
        }
        let state = AppState::new(Store::new(Store::default_path()));
        let window = ui::window::build(app, &state);
        window.present();
        state.load_devices();
    });
    app.run()
}
