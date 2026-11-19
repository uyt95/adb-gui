mod edit_dialog;
mod socket_editor;

use std::rc::Rc;

use adw::prelude::*;

use super::{Callback, Page};
use crate::l10n;
use crate::models::Proxy;
use crate::state::AppState;
use crate::ui::widgets;

use edit_dialog::edit_dialog;

pub fn page(state: &Rc<AppState>) -> Page {
    let widget = widgets::list_page(
        state,
        |state| {
            state
                .db
                .borrow()
                .proxies
                .iter()
                .map(|proxy| (proxy.clone(), state.is_proxy_active(proxy)))
                .collect()
        },
        |state, list, (proxy, active)| row(state, list, proxy, active),
    );

    let on_add: Callback = {
        let state = state.clone();
        let parent = widget.clone();
        Rc::new(move || edit_dialog(&state, &parent, None))
    };
    let on_show: Callback = {
        let state = state.clone();
        Rc::new(move || state.load_active_proxies())
    };

    Page {
        id: "proxies",
        title: l10n::page_title_proxies(),
        icon: "network-transmit-receive-symbolic",
        widget,
        on_add: Some(on_add),
        on_show: Some(on_show),
    }
}

fn row(state: &Rc<AppState>, list: &gtk::ListBox, proxy: Proxy, active: bool) -> adw::ActionRow {
    let subtitle = format!(
        "{}: {} {}  ·  {}: {} {}",
        l10n::field_from(),
        proxy.from.socket_type.type_name(),
        proxy.from.name,
        l10n::field_to(),
        proxy.to.socket_type.type_name(),
        proxy.to.name,
    );
    let row = widgets::action_row(proxy.proxy_type.type_name(), Some(&subtitle));
    row.add_suffix(&widgets::status_label(
        if active {
            l10n::value_active()
        } else {
            l10n::value_inactive()
        },
        active,
    ));

    let (icon, tooltip) = if active {
        ("media-playback-stop-symbolic", l10n::button_deactivate())
    } else {
        ("media-playback-start-symbolic", l10n::button_activate())
    };
    let toggle = widgets::icon_button(icon, tooltip, {
        let state = state.clone();
        let proxy = proxy.clone();
        move || {
            let Some(device) = state.active_device() else {
                return;
            };
            let proxy = proxy.clone();
            state.run_action(
                move |tools| {
                    if active {
                        tools.deactivate_proxy(&device, &proxy)
                    } else {
                        tools.activate_proxy(&device, &proxy)
                    }
                },
                |state| state.load_active_proxies(),
            );
        }
    });
    // Rows are rebuilt on every state change, so this needs no binding.
    toggle.set_sensitive(state.active_device().is_some());
    row.add_suffix(&toggle);

    row.add_suffix(&widgets::edit_button({
        let state = state.clone();
        let list = list.clone();
        let proxy = proxy.clone();
        move || edit_dialog(&state, &list, Some(proxy.clone()))
    }));
    row.add_suffix(&widgets::delete_button(
        list,
        l10n::dialog_title_proxy_delete(),
        proxy.to_string(),
        {
            let state = state.clone();
            let id = proxy.id;
            move || state.update_db(|db| db.proxies.retain(|p| p.id != id))
        },
    ));
    row
}
