use std::rc::Rc;

use adw::prelude::*;

use crate::l10n;
use crate::models::Connection;
use crate::state::AppState;
use crate::ui::{self, Page};

pub fn page(state: &Rc<AppState>) -> Page {
    let list = ui::boxed_list();
    let widget = ui::page_body(&list);

    let refresh = {
        let state = state.clone();
        let list = list.clone();
        move || {
            list.remove_all();
            let connections = state.db.borrow().connections.clone();
            let devices = state.devices.borrow().clone();
            for connection in connections {
                let connected = devices
                    .iter()
                    .any(|d| d.matches_address(&connection.address));
                list.append(&row(&state, &list, connection, connected));
            }
        }
    };
    refresh();
    state.subscribe(refresh);

    let on_add: Rc<dyn Fn()> = {
        let state = state.clone();
        let widget = widget.clone();
        Rc::new(move || edit_dialog(&state, &widget, None))
    };
    let on_show: Rc<dyn Fn()> = {
        let state = state.clone();
        Rc::new(move || state.load_devices())
    };

    Page {
        id: "connections",
        title: l10n::page_title_connections(),
        icon: "network-wired-symbolic",
        widget,
        on_add: Some(on_add),
        on_show: Some(on_show),
    }
}

fn row(
    state: &Rc<AppState>,
    list: &gtk::ListBox,
    connection: Connection,
    connected: bool,
) -> adw::ActionRow {
    let row = ui::action_row(&connection.name, Some(&connection.address));
    row.add_suffix(&ui::status_label(
        if connected {
            l10n::value_connected()
        } else {
            l10n::value_disconnected()
        },
        connected,
    ));

    row.add_suffix(&ui::icon_button(
        "document-edit-symbolic",
        l10n::button_edit(),
        {
            let state = state.clone();
            let list = list.clone();
            let connection = connection.clone();
            move || edit_dialog(&state, &list, Some(connection.clone()))
        },
    ));

    let (icon, tooltip) = if connected {
        ("network-offline-symbolic", l10n::button_disconnect())
    } else {
        ("network-transmit-receive-symbolic", l10n::button_connect())
    };
    row.add_suffix(&ui::icon_button(icon, tooltip, {
        let state = state.clone();
        let address = connection.address.clone();
        move || {
            let address = address.clone();
            state.run_action(
                move |tools| {
                    if connected {
                        tools.disconnect(&address)
                    } else {
                        tools.connect(&address)
                    }
                },
                |state| state.load_devices(),
            );
        }
    }));

    row.add_suffix(&ui::icon_button(
        "user-trash-symbolic",
        l10n::button_delete(),
        {
            let state = state.clone();
            let list = list.clone();
            move || {
                let state = state.clone();
                let id = connection.id;
                ui::confirm_delete(
                    &list,
                    l10n::dialog_title_connection_delete(),
                    &connection.name,
                    move || state.update_db(|db| db.connections.retain(|c| c.id != id)),
                );
            }
        },
    ));
    row
}

fn edit_dialog(
    state: &Rc<AppState>,
    parent: &impl IsA<gtk::Widget>,
    connection: Option<Connection>,
) {
    let is_new = connection.is_none();
    let connection = connection.unwrap_or_default();

    let name = adw::EntryRow::builder()
        .title(l10n::field_name())
        .text(&connection.name)
        .build();
    let address = adw::EntryRow::builder()
        .title(l10n::field_address())
        .text(&connection.address)
        .build();
    let group = adw::PreferencesGroup::new();
    group.add(&name);
    group.add(&address);
    let error = gtk::Label::builder()
        .css_classes(["error"])
        .xalign(0.0)
        .margin_top(6)
        .visible(false)
        .build();
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.append(&group);
    content.append(&error);

    let save = {
        let state = state.clone();
        let (name, address) = (name.clone(), address.clone());
        move || {
            let name_text = name.text().trim().to_string();
            let address_text = address.text().trim().to_string();
            if name_text.is_empty() {
                ui::mark_invalid(&name);
                error.set_label(l10n::error_missing_name());
                error.set_visible(true);
                return false;
            }
            if address_text.is_empty() {
                ui::mark_invalid(&address);
                error.set_label(l10n::error_missing_address());
                error.set_visible(true);
                return false;
            }
            let updated = Connection {
                id: connection.id,
                name: name_text,
                address: address_text,
            };
            state.update_db(|db| db.put_connection(updated));
            true
        }
    };
    let save = Rc::new(save);

    let dialog = ui::form_dialog(
        parent,
        if is_new {
            l10n::dialog_title_connection_add()
        } else {
            l10n::dialog_title_connection_edit()
        },
        if is_new {
            l10n::button_add()
        } else {
            l10n::button_save()
        },
        &content,
        {
            let save = save.clone();
            move || save()
        },
    );
    let weak = dialog.downgrade();
    address.connect_entry_activated(move |_| {
        if save()
            && let Some(dialog) = weak.upgrade() {
                dialog.close();
            }
    });
    name.grab_focus();
}
