use std::cell::Cell;
use std::rc::Rc;

use adw::prelude::*;
use gtk::glib;

use super::{Callback, Page};
use crate::l10n;
use crate::models::SettingKey;
use crate::state::AppState;
use crate::ui::widgets::{self, PathKind};

pub fn page(state: &Rc<AppState>) -> Page {
    let directory = widgets::path_row(l10n::field_screenshot_directory(), PathKind::Folder);
    directory.set_text(&state.setting(SettingKey::ScreenshotPath));
    let group = adw::PreferencesGroup::new();
    group.add(&directory);

    let capture = gtk::Button::builder()
        .label(l10n::button_capture())
        .tooltip_text(l10n::button_capture_screenshot())
        .css_classes(["suggested-action", "pill"])
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
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

    // The capture button sits next to the directory field, and moves below
    // it when the page is narrow (see the breakpoint below).
    group.set_hexpand(true);
    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    controls.append(&group);
    controls.append(&capture);

    let content = gtk::Box::new(gtk::Orientation::Vertical, 18);
    content.append(&controls);
    content.append(&picture);
    content.append(&saved_to);

    // State refreshes during a capture must not turn the button back on, or
    // a second capture would race the first on the device's temp file.
    let busy = Rc::new(Cell::new(false));
    let update_capture = widgets::bind_sensitive(state, &capture, {
        let busy = busy.clone();
        move || !busy.get()
    });

    capture.connect_clicked({
        let state = state.clone();
        let directory = directory.clone();
        move |_| {
            let Some(device) = state.active_device() else {
                return;
            };
            let target = directory.text().to_string();
            // Milliseconds keep two captures in the same second from overwriting
            // each other.
            let timestamp = glib::DateTime::now_local()
                .and_then(|now| {
                    let seconds = now.format("%Y%m%d-%H%M%S")?;
                    Ok(format!("{seconds}-{:03}", now.microsecond() / 1000))
                })
                .unwrap_or_default();
            busy.set(true);
            update_capture();
            let (busy, update_capture) = (busy.clone(), update_capture.clone());
            let (picture, saved_to) = (picture.clone(), saved_to.clone());
            state.run(
                move |tools| tools.take_screenshot(&device, &target, &timestamp),
                move |state, result| {
                    busy.set(false);
                    update_capture();
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

    // Like the Flutter app, start from the default directory each time the page opens.
    let on_show: Callback = {
        let state = state.clone();
        Rc::new(move || directory.set_text(&state.setting(SettingKey::ScreenshotPath)))
    };

    let widget = adw::BreakpointBin::builder()
        .child(&widgets::page_body(&content))
        .width_request(300)
        .height_request(200)
        .build();
    let narrow = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
        adw::BreakpointConditionLengthType::MaxWidth,
        500.0,
        adw::LengthUnit::Sp,
    ));
    narrow.add_setter(
        &controls,
        "orientation",
        Some(&gtk::Orientation::Vertical.to_value()),
    );
    narrow.add_setter(&controls, "spacing", Some(&18.to_value()));
    widget.add_breakpoint(narrow);

    Page {
        id: "screenshot",
        title: l10n::page_title_screenshot(),
        icon: "camera-photo-symbolic",
        widget: widget.upcast(),
        on_add: None,
        on_show: Some(on_show),
    }
}
