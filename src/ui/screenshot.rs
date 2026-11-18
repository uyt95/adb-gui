use std::rc::Rc;

use adw::prelude::*;
use gtk::{gio, glib};

use crate::l10n;
use crate::models::SettingKey;
use crate::state::AppState;
use crate::ui::{self, Page};

pub fn page(state: &Rc<AppState>) -> Page {
    let directory = adw::EntryRow::builder()
        .title(l10n::field_screenshot_directory())
        .text(state.setting(SettingKey::ScreenshotPath))
        .build();
    let browse = gtk::Button::builder()
        .label(l10n::button_browse())
        .valign(gtk::Align::Center)
        .css_classes(["flat"])
        .build();
    directory.add_suffix(&browse);
    let group = adw::PreferencesGroup::new();
    group.add(&directory);

    let capture = gtk::Button::builder()
        .label(l10n::button_capture_screenshot())
        .css_classes(["suggested-action", "pill"])
        .halign(gtk::Align::Center)
        .build();
    let picture = gtk::Picture::builder()
        .content_fit(gtk::ContentFit::Contain)
        .vexpand(true)
        .height_request(320)
        .visible(false)
        .build();
    let saved_to = gtk::Label::builder()
        .css_classes(["dim-label", "caption"])
        .selectable(true)
        .wrap(true)
        .visible(false)
        .build();

    let content = gtk::Box::new(gtk::Orientation::Vertical, 18);
    content.append(&group);
    content.append(&capture);
    content.append(&picture);
    content.append(&saved_to);

    browse.connect_clicked({
        let directory = directory.clone();
        move |button| {
            let dialog = gtk::FileDialog::builder()
                .accept_label(l10n::button_select())
                .initial_folder(&gio::File::for_path(directory.text().as_str()))
                .build();
            let directory = directory.clone();
            let window = button.root().and_downcast::<gtk::Window>();
            dialog.select_folder(window.as_ref(), gio::Cancellable::NONE, move |result| {
                if let Some(path) = result.ok().and_then(|folder| folder.path()) {
                    directory.set_text(&path.to_string_lossy());
                }
            });
        }
    });

    capture.connect_clicked({
        let state = state.clone();
        let directory = directory.clone();
        move |button| {
            let Some(device) = state.active_device() else {
                return;
            };
            let target = directory.text().to_string();
            let timestamp = glib::DateTime::now_local()
                .and_then(|now| now.format("%Y%m%d-%H%M%S"))
                .map(|s| s.to_string())
                .unwrap_or_default();
            button.set_sensitive(false);
            let (button, picture, saved_to) = (button.clone(), picture.clone(), saved_to.clone());
            state.run(
                move |tools| tools.take_screenshot(&device, &target, &timestamp),
                move |state, result| {
                    button.set_sensitive(state.active_device().is_some());
                    match result {
                        Ok(path) => {
                            picture.set_filename(Some(&path));
                            picture.set_visible(true);
                            saved_to.set_label(&path.to_string_lossy());
                            saved_to.set_visible(true);
                        }
                        Err(error) => state.show_error(&error),
                    }
                },
            );
        }
    });

    let update = {
        let state = state.clone();
        let capture = capture.clone();
        move || capture.set_sensitive(state.active_device().is_some())
    };
    update();
    state.subscribe(update);

    // Like the Flutter app, start from the default directory each time the page opens.
    let on_show: Rc<dyn Fn()> = {
        let state = state.clone();
        Rc::new(move || directory.set_text(&state.setting(SettingKey::ScreenshotPath)))
    };

    Page {
        id: "screenshot",
        title: l10n::page_title_screenshot(),
        icon: "camera-photo-symbolic",
        widget: ui::page_body(&content),
        on_add: None,
        on_show: Some(on_show),
    }
}
