use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use gtk::{gio, glib};

use super::Page;
use crate::l10n;
use crate::models::SettingKey;
use crate::state::AppState;
use crate::ui::widgets;

pub fn page(state: &Rc<AppState>) -> Page {
    let group = adw::PreferencesGroup::new();
    for key in SettingKey::ALL {
        let title = match key {
            SettingKey::AdbPath => l10n::field_setting_adb_path(),
            SettingKey::EmulatorPath => l10n::field_setting_emulator_path(),
            SettingKey::ScreenshotPath => l10n::field_setting_screenshot_directory(),
        };
        let row = adw::EntryRow::builder()
            .title(title)
            .text(state.setting(key))
            .build();

        // Save one second after the last edit, like the Flutter app. A pending
        // edit is saved at once on Enter and when the app shuts down, so
        // closing the window right after typing does not lose it.
        let pending: Rc<RefCell<Option<glib::SourceId>>> = Rc::default();
        let save_now: Rc<dyn Fn()> = {
            let (state, pending, row) = (state.clone(), pending.clone(), row.downgrade());
            Rc::new(move || {
                let Some(source) = pending.take() else {
                    return;
                };
                source.remove();
                if let Some(row) = row.upgrade() {
                    let value = row.text().to_string();
                    state.update_db(|db| db.settings.set(key, value));
                }
            })
        };
        row.connect_changed({
            let (state, pending) = (state.clone(), pending.clone());
            move |row| {
                if let Some(source) = pending.take() {
                    source.remove();
                }
                let value = row.text().to_string();
                let state = state.clone();
                let pending_inner = pending.clone();
                let source = glib::timeout_add_local_once(Duration::from_secs(1), move || {
                    pending_inner.take();
                    state.update_db(|db| db.settings.set(key, value));
                });
                pending.replace(Some(source));
            }
        });
        row.connect_entry_activated({
            let save_now = save_now.clone();
            move |_| save_now()
        });
        if let Some(app) = gio::Application::default() {
            app.connect_shutdown(move |_| save_now());
        }
        group.add(&row);
    }

    Page {
        id: "settings",
        title: l10n::page_title_settings(),
        icon: "emblem-system-symbolic",
        widget: widgets::page_body(&group),
        on_add: None,
        on_show: None,
    }
}
