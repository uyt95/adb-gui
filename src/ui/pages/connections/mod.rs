mod add_dialog;
mod edit_dialog;

use std::rc::Rc;

use adw::prelude::*;

use super::{Callback, Page};
use crate::l10n;
use crate::models::{Connection, Device};
use crate::state::AppState;
use crate::ui::widgets;

use add_dialog::add_dialog;
use edit_dialog::edit_dialog;

pub fn page(state: &Rc<AppState>) -> Page {
    let widget = widgets::list_page(
        state,
        widgets::EmptyState {
            icon: "network-wired-symbolic",
            title: l10n::empty_connections_title(),
            description: l10n::empty_connections_description(),
        },
        |state| {
            let devices = state.devices.borrow();
            state
                .db
                .borrow()
                .connections
                .iter()
                .map(|connection| {
                    let device = devices
                        .iter()
                        .find(|d| d.matches_connection(connection))
                        .cloned();
                    (connection.clone(), device)
                })
                .collect()
        },
        |state, list, (connection, device)| row(state, list, connection, device),
    );

    let on_add: Callback = {
        let state = state.clone();
        let parent = widget.clone();
        Rc::new(move || add_dialog(&state, &parent))
    };
    let on_show: Callback = {
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
    device: Option<Device>,
) -> adw::ActionRow {
    let subtitle = if connection.address.is_empty() {
        &connection.mdns_name
    } else {
        &connection.address
    };
    let row = widgets::action_row(&connection.name, Some(subtitle));
    let connected = device.is_some();
    row.add_suffix(&widgets::status_label(
        if connected {
            l10n::value_connected()
        } else {
            l10n::value_disconnected()
        },
        connected,
    ));

    row.add_suffix(&widgets::edit_button({
        let state = state.clone();
        let list = list.clone();
        let connection = connection.clone();
        move || edit_dialog(&state, &list, connection.clone())
    }));

    let (icon, tooltip) = if connected {
        ("network-offline-symbolic", l10n::button_disconnect())
    } else {
        ("network-transmit-receive-symbolic", l10n::button_connect())
    };
    row.add_suffix(&widgets::icon_button(icon, tooltip, {
        let state = state.clone();
        let connection = connection.clone();
        move || match &device {
            // Disconnect by the serial adb lists, which is the mDNS name when
            // adb connected the device by itself.
            Some(device) => {
                let serial = device.serial.clone();
                state.run_action(
                    move |tools| tools.disconnect(&serial),
                    |state| state.load_devices(),
                );
            }
            None => {
                let connection = connection.clone();
                state.run(
                    move |tools| tools.connect_saved(&connection),
                    |state, result| {
                        match result {
                            // Remember the new port for the next time. Only
                            // the address and mDNS name are written back, so
                            // a rename made while connecting is kept.
                            Ok(reached) => {
                                let stored = state
                                    .db
                                    .borrow()
                                    .connections
                                    .iter()
                                    .find(|c| c.id == reached.id)
                                    .cloned();
                                if let Some(mut stored) = stored
                                    && (stored.address != reached.address
                                        || stored.mdns_name != reached.mdns_name)
                                {
                                    stored.address = reached.address;
                                    stored.mdns_name = reached.mdns_name;
                                    state.update_db(|db| db.put_connection(stored));
                                }
                            }
                            Err(error) => state.show_error(&error),
                        }
                        state.load_devices();
                    },
                );
            }
        }
    }));

    row.add_suffix(&widgets::delete_button(
        list,
        l10n::dialog_title_connection_delete(),
        connection.name.clone(),
        {
            let state = state.clone();
            let id = connection.id;
            move || state.update_db(|db| db.connections.retain(|c| c.id != id))
        },
    ));
    row
}
