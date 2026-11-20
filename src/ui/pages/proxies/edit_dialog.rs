use std::cell::Cell;
use std::rc::Rc;

use adw::prelude::*;

use super::socket_editor::SocketEditor;
use crate::l10n;
use crate::models::{Proxy, ProxyType, Socket, SocketType};
use crate::state::AppState;
use crate::ui::widgets::{self, FormTitles, Validation};

/// Adds a proxy (`None`) or edits an existing one.
pub fn edit_dialog(state: &Rc<AppState>, parent: &impl IsA<gtk::Widget>, proxy: Option<Proxy>) {
    let is_new = proxy.is_none();
    let proxy = proxy.unwrap_or_else(|| {
        let socket = Socket {
            socket_type: SocketType::for_proxy_type(ProxyType::Forward)[0],
            name: String::new(),
        };
        Proxy {
            id: 0,
            proxy_type: ProxyType::Forward,
            from: socket.clone(),
            to: socket,
        }
    });

    let type_names: Vec<&str> = ProxyType::ALL.iter().map(|t| t.type_name()).collect();
    let proxy_type_row = widgets::combo_row(
        l10n::field_type(),
        &type_names,
        ProxyType::ALL
            .iter()
            .position(|t| *t == proxy.proxy_type)
            .unwrap_or(0),
    );
    let type_group = adw::PreferencesGroup::new();
    type_group.add(&proxy_type_row);

    let from = Rc::new(SocketEditor::new(
        l10n::field_from(),
        proxy.proxy_type,
        &proxy.from,
    ));
    let to = Rc::new(SocketEditor::new(
        l10n::field_to(),
        proxy.proxy_type,
        &proxy.to,
    ));

    let selected_type = Rc::new(Cell::new(proxy.proxy_type));
    proxy_type_row.connect_selected_notify({
        let (from, to, selected_type) = (from.clone(), to.clone(), selected_type.clone());
        move |row| {
            let proxy_type = ProxyType::ALL[row.selected() as usize % ProxyType::ALL.len()];
            selected_type.set(proxy_type);
            from.set_proxy_type(proxy_type, from.socket().socket_type);
            to.set_proxy_type(proxy_type, to.socket().socket_type);
        }
    });

    let content = gtk::Box::new(gtk::Orientation::Vertical, 18);
    content.append(&type_group);
    content.append(&from.group);
    content.append(&to.group);
    let validation = Validation::new();
    content.append(&validation.label);

    let titles = FormTitles {
        add: l10n::dialog_title_proxy_add(),
        edit: l10n::dialog_title_proxy_edit(),
    };
    let state = state.clone();
    widgets::form_dialog(
        parent,
        titles.title(is_new),
        FormTitles::confirm_label(is_new),
        &content,
        move || {
            let Some(from) = from.validated_socket(&validation) else {
                return false;
            };
            let Some(to) = to.validated_socket(&validation) else {
                return false;
            };
            let updated = Proxy {
                id: proxy.id,
                proxy_type: selected_type.get(),
                from,
                to,
            };
            state.update_db(|db| db.put_proxy(updated));
            true
        },
    );
}
