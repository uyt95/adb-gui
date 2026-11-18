use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use gtk::glib;

use crate::l10n;
use crate::models::SettingKey;
use crate::state::AppState;
use crate::ui::{self, Page};

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

        // Save one second after the last edit, like the Flutter app.
        let pending: Rc<RefCell<Option<glib::SourceId>>> = Rc::default();
        let state = state.clone();
        row.connect_changed(move |row| {
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
        });
        group.add(&row);
    }

    Page {
        id: "settings",
        title: l10n::page_title_settings(),
        icon: "emblem-system-symbolic",
        widget: ui::page_body(&group),
        on_add: None,
        on_show: None,
    }
}
