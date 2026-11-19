use std::cell::RefCell;

use adw::prelude::*;

use crate::l10n;
use crate::models::{ProxyType, Socket, SocketType};
use crate::ui::widgets;

/// Type selector and name field for one end of a proxy.
pub struct SocketEditor {
    pub group: adw::PreferencesGroup,
    socket_type: adw::ComboRow,
    name: adw::EntryRow,
    /// Socket types currently offered, matching the combo row's items.
    types: RefCell<Vec<SocketType>>,
}

impl SocketEditor {
    pub fn new(title: &str, proxy_type: ProxyType, socket: &Socket) -> SocketEditor {
        let editor = SocketEditor {
            group: adw::PreferencesGroup::builder().title(title).build(),
            socket_type: adw::ComboRow::builder().title(l10n::field_type()).build(),
            name: adw::EntryRow::builder()
                .title(l10n::field_name())
                .text(&socket.name)
                .build(),
            types: Default::default(),
        };
        editor.group.add(&editor.socket_type);
        editor.group.add(&editor.name);
        editor.set_proxy_type(proxy_type, socket.socket_type);
        editor
    }

    /// Offers the socket types of `proxy_type`, keeping `current` when it is allowed.
    pub fn set_proxy_type(&self, proxy_type: ProxyType, current: SocketType) {
        let types = SocketType::for_proxy_type(proxy_type);
        let names: Vec<&str> = types.iter().map(|t| t.type_name()).collect();
        let index = types.iter().position(|t| *t == current).unwrap_or(0);
        self.socket_type
            .set_model(Some(&widgets::string_list(&names)));
        self.socket_type.set_selected(index as u32);
        self.types.replace(types);
    }

    pub fn socket(&self) -> Socket {
        let types = self.types.borrow();
        Socket {
            socket_type: types
                .get(self.socket_type.selected() as usize)
                .copied()
                .unwrap_or(types[0]),
            name: self.name.text().trim().to_string(),
        }
    }
}
