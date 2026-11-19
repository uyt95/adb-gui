mod edit_dialog;

use std::rc::Rc;

use adw::prelude::*;

use super::{Callback, Page};
use crate::l10n;
use crate::models::Connection;
use crate::state::AppState;
use crate::ui::widgets;

use edit_dialog::edit_dialog;

pub fn page(state: &Rc<AppState>) -> Page {
    let widget = widgets::list_page(
        state,
        |state| {
            let devices = state.devices.borrow();
            state
                .db
                .borrow()
                .connections
                .iter()
                .map(|connection| {
                    let connected = devices
                        .iter()
                        .any(|d| d.matches_address(&connection.address));
                    (connection.clone(), connected)
                })
                .collect()
        },
        |state, list, (connection, connected)| row(state, list, connection, connected),
    );

    let on_add: Callback = {
        let state = state.clone();
        let parent = widget.clone();
        Rc::new(move || edit_dialog(&state, &parent, None))
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
    connected: bool,
) -> adw::ActionRow {
    let row = widgets::action_row(&connection.name, Some(&connection.address));
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
        move || edit_dialog(&state, &list, Some(connection.clone()))
    }));

    let (icon, tooltip) = if connected {
        ("network-offline-symbolic", l10n::button_disconnect())
    } else {
        ("network-transmit-receive-symbolic", l10n::button_connect())
    };
    row.add_suffix(&widgets::icon_button(icon, tooltip, {
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
