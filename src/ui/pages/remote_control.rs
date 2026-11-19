use std::rc::Rc;

use adw::prelude::*;

use super::Page;
use crate::l10n;
use crate::state::AppState;
use crate::ui::widgets;

enum Face {
    Icon(&'static str),
    Text(&'static str),
}

/// (face, description, key code suffix)
type RemoteButton = (Face, &'static str, &'static str);

const fn icon(
    name: &'static str,
    description: &'static str,
    key: &'static str,
) -> Option<RemoteButton> {
    Some((Face::Icon(name), description, key))
}

const fn text(
    label: &'static str,
    description: &'static str,
    key: &'static str,
) -> Option<RemoteButton> {
    Some((Face::Text(label), description, key))
}

const BUTTONS: [[Option<RemoteButton>; 3]; 12] = [
    [
        icon("system-shutdown-symbolic", "power", "POWER"),
        None,
        icon("system-search-symbolic", "assistant", "SEARCH"),
    ],
    [
        None,
        icon("go-up-symbolic", "up", "DPAD_UP"),
        icon("dialog-information-symbolic", "info", "INFO"),
    ],
    [
        icon("go-previous-symbolic", "left", "DPAD_LEFT"),
        text("OK", "ok", "DPAD_CENTER"),
        icon("go-next-symbolic", "right", "DPAD_RIGHT"),
    ],
    [
        icon("media-record-symbolic", "record", "MEDIA_RECORD"),
        icon("go-down-symbolic", "down", "DPAD_DOWN"),
        text("Guide", "guide", "GUIDE"),
    ],
    [
        icon("edit-undo-symbolic", "back", "BACK"),
        icon("go-home-symbolic", "home", "HOME"),
        icon("open-menu-symbolic", "menu", "MENU"),
    ],
    [
        icon("media-seek-backward-symbolic", "rewind", "MEDIA_REWIND"),
        icon("media-playback-start-symbolic", "play", "MEDIA_PLAY_PAUSE"),
        icon(
            "media-seek-forward-symbolic",
            "fast forward",
            "MEDIA_FAST_FORWARD",
        ),
    ],
    [
        icon("audio-volume-high-symbolic", "volume up", "VOLUME_UP"),
        None,
        icon("pan-up-symbolic", "channel down", "PAGE_DOWN"),
    ],
    [
        icon("audio-volume-low-symbolic", "volume down", "VOLUME_DOWN"),
        icon("audio-volume-muted-symbolic", "mute", "MUTE"),
        icon("pan-down-symbolic", "channel up", "PAGE_UP"),
    ],
    [
        text("1", "1", "1"),
        text("2", "2", "2"),
        text("3", "3", "3"),
    ],
    [
        text("4", "4", "4"),
        text("5", "5", "5"),
        text("6", "6", "6"),
    ],
    [
        text("7", "7", "7"),
        text("8", "8", "8"),
        text("9", "9", "9"),
    ],
    [None, text("0", "0", "0"), None],
];

pub fn page(state: &Rc<AppState>) -> Page {
    let grid = gtk::Grid::builder()
        .row_spacing(6)
        .column_spacing(6)
        .halign(gtk::Align::Center)
        .row_homogeneous(true)
        .column_homogeneous(true)
        .build();
    for (row, keys) in BUTTONS.iter().enumerate() {
        for (column, button) in keys.iter().enumerate() {
            let Some((face, description, key)) = button else {
                continue;
            };
            let widget = gtk::Button::builder()
                .tooltip_text(*description)
                .width_request(64)
                .height_request(48)
                .css_classes(["circular"])
                .build();
            match face {
                Face::Icon(name) => widget.set_icon_name(name),
                Face::Text(label) => widget.set_label(label),
            }
            widget.update_property(&[gtk::accessible::Property::Label(description)]);
            widgets::bind_to_device(state, &widget);
            let state = state.clone();
            let key: &'static str = key;
            widget.connect_clicked(move |_| {
                if let Some(device) = state.active_device() {
                    state.run_action(move |tools| tools.send_key(&device, key), |_| {});
                }
            });
            grid.attach(&widget, column as i32, row as i32, 1, 1);
        }
    }

    Page {
        id: "remote-control",
        title: l10n::page_title_remote_control(),
        icon: "video-display-symbolic",
        widget: widgets::page_body(&grid),
        on_add: None,
        on_show: None,
    }
}
