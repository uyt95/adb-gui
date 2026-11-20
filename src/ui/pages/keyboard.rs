use std::rc::Rc;

use adw::prelude::*;
use gtk::gdk;
use gtk::glib;

use super::{Callback, Page};
use crate::l10n;
use crate::state::AppState;
use crate::ui::widgets;

/// Sends text and special keys to the active device.
///
/// Shortcuts in the text field: Ctrl+Enter sends the text, Ctrl+Backspace
/// sends Backspace and Ctrl+Tab sends Tab.
pub fn page(state: &Rc<AppState>) -> Page {
    let text_view = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::WordChar)
        .top_margin(8)
        .bottom_margin(8)
        .left_margin(8)
        .right_margin(8)
        .accepts_tab(false)
        .build();
    text_view.update_property(&[gtk::accessible::Property::Label(
        l10n::field_keyboard_input(),
    )]);
    let buffer = text_view.buffer();
    let scrolled = gtk::ScrolledWindow::builder()
        .child(&text_view)
        .min_content_height(160)
        .hexpand(true)
        .css_classes(["card"])
        .build();

    let send_backspace = {
        let state = state.clone();
        move || send_key(&state, "DEL")
    };
    let send_tab = {
        let state = state.clone();
        move || send_key(&state, "TAB")
    };
    // While a send runs the field is read-only and Send is off, so text typed
    // meanwhile is not cleared with the sent text and a repeated Ctrl+Enter
    // does not type it twice.
    let send_text = {
        let state = state.clone();
        let text_view = text_view.clone();
        move || {
            if !text_view.is_editable() {
                return;
            }
            let Some(device) = state.active_device() else {
                return;
            };
            let buffer = text_view.buffer();
            let text = buffer
                .text(&buffer.start_iter(), &buffer.end_iter(), false)
                .to_string();
            if text.is_empty() {
                return;
            }
            text_view.set_editable(false);
            let text_view = text_view.clone();
            state.run_input(
                move |tools| tools.send_text(&device, &text),
                move |state, result| {
                    text_view.set_editable(true);
                    match result {
                        Ok(()) => text_view.buffer().set_text(""),
                        Err(error) => state.show_error(&error),
                    }
                },
            );
        }
    };
    let send_backspace = Rc::new(send_backspace);
    let send_tab = Rc::new(send_tab);
    let send_text = Rc::new(send_text);

    let backspace_button = widgets::icon_button("edit-clear-symbolic", l10n::button_backspace(), {
        let f = send_backspace.clone();
        move || f()
    });
    let tab_button = widgets::icon_button("format-indent-more-symbolic", l10n::button_tab(), {
        let f = send_tab.clone();
        move || f()
    });
    let send_button = widgets::icon_button("mail-send-symbolic", l10n::button_send(), {
        let f = send_text.clone();
        move || f()
    });
    send_button.set_valign(gtk::Align::Start);

    // Show each button's shortcut in its tooltip.
    for (button, label, shortcut) in [
        (
            &backspace_button,
            l10n::button_backspace(),
            "Ctrl+Backspace",
        ),
        (&tab_button, l10n::button_tab(), "Ctrl+Tab"),
        (&send_button, l10n::button_send(), "Ctrl+Enter"),
    ] {
        button.set_tooltip_text(Some(&format!("{label} ({shortcut})")));
        button.update_property(&[gtk::accessible::Property::KeyShortcuts(shortcut)]);
    }

    let keys = gtk::Box::new(gtk::Orientation::Vertical, 6);
    keys.append(&backspace_button);
    keys.append(&tab_button);

    let shortcuts = gtk::EventControllerKey::new();
    shortcuts.set_propagation_phase(gtk::PropagationPhase::Capture);
    shortcuts.connect_key_pressed(move |_, key, _, modifiers| {
        if !modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
            return glib::Propagation::Proceed;
        }
        match key {
            gdk::Key::Return | gdk::Key::KP_Enter => send_text(),
            gdk::Key::BackSpace => send_backspace(),
            gdk::Key::Tab | gdk::Key::ISO_Left_Tab => send_tab(),
            _ => return glib::Propagation::Proceed,
        }
        glib::Propagation::Stop
    });
    text_view.add_controller(shortcuts);

    // GtkTextView has no placeholder, so overlay a dimmed label while it is empty.
    let placeholder = gtk::Label::builder()
        .label(l10n::keyboard_input_placeholder())
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .margin_top(8)
        .margin_start(8)
        .can_target(false)
        .css_classes(["dim-label"])
        .build();
    buffer.connect_changed({
        let placeholder = placeholder.clone();
        move |buffer| placeholder.set_visible(buffer.char_count() == 0)
    });
    let field = gtk::Overlay::builder()
        .child(&scrolled)
        .hexpand(true)
        .build();
    field.add_overlay(&placeholder);

    let content = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    content.append(&keys);
    content.append(&field);
    content.append(&send_button);

    widgets::bind_to_device(state, &backspace_button);
    widgets::bind_to_device(state, &tab_button);
    let update = widgets::bind_sensitive(state, &send_button, {
        let text_view = text_view.clone();
        move || text_view.is_editable() && text_view.buffer().char_count() > 0
    });
    buffer.connect_changed({
        let update = update.clone();
        move |_| update()
    });
    text_view.connect_editable_notify(move |_| update());

    // Focus after the sidebar click has finished moving focus to its row.
    let on_show: Callback = Rc::new(move || {
        let text_view = text_view.clone();
        glib::idle_add_local_once(move || {
            text_view.grab_focus();
        });
    });

    Page {
        id: "keyboard",
        title: l10n::page_title_keyboard(),
        icon: "input-keyboard-symbolic",
        widget: widgets::page_body(&content),
        on_add: None,
        on_show: Some(on_show),
    }
}

fn send_key(state: &Rc<AppState>, key: &'static str) {
    if let Some(device) = state.active_device() {
        state.run_input(
            move |tools| tools.send_key(&device, key),
            |state, result| {
                if let Err(error) = result {
                    state.show_error(&error);
                }
            },
        );
    }
}
