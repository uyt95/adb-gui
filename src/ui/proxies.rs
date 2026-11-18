use std::cell::Cell;
use std::rc::Rc;

use adw::prelude::*;

use crate::l10n;
use crate::models::{Proxy, ProxyType, Socket, SocketType};
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
            let proxies = state.db.borrow().proxies.clone();
            for proxy in proxies {
                let active = state.is_proxy_active(&proxy);
                list.append(&row(&state, &list, proxy, active));
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
    let row = ui::action_row(proxy.proxy_type.type_name(), Some(&subtitle));
    row.add_suffix(&ui::status_label(
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
    let toggle = ui::icon_button(icon, tooltip, {
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
    toggle.set_sensitive(state.active_device().is_some());
    row.add_suffix(&toggle);

    row.add_suffix(&ui::icon_button(
        "document-edit-symbolic",
        l10n::button_edit(),
        {
            let state = state.clone();
            let list = list.clone();
            let proxy = proxy.clone();
            move || edit_dialog(&state, &list, Some(proxy.clone()))
        },
    ));
    row.add_suffix(&ui::icon_button(
        "user-trash-symbolic",
        l10n::button_delete(),
        {
            let state = state.clone();
            let list = list.clone();
            move || {
                let state = state.clone();
                let id = proxy.id;
                ui::confirm_delete(
                    &list,
                    l10n::dialog_title_proxy_delete(),
                    &proxy.to_string(),
                    move || state.update_db(|db| db.proxies.retain(|p| p.id != id)),
                );
            }
        },
    ));
    row
}

/// Type selector and name field for one end of a proxy.
struct SocketEditor {
    group: adw::PreferencesGroup,
    socket_type: adw::ComboRow,
    name: adw::EntryRow,
    /// Socket types currently offered, matching the combo row's items.
    types: std::cell::RefCell<Vec<SocketType>>,
}

impl SocketEditor {
    fn new(title: &str, proxy_type: ProxyType, socket: &Socket) -> SocketEditor {
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
    fn set_proxy_type(&self, proxy_type: ProxyType, current: SocketType) {
        let types = SocketType::for_proxy_type(proxy_type);
        let names: Vec<&str> = types.iter().map(|t| t.type_name()).collect();
        let index = types.iter().position(|t| *t == current).unwrap_or(0);
        self.socket_type.set_model(Some(&ui::string_list(&names)));
        self.socket_type.set_selected(index as u32);
        self.types.replace(types);
    }

    fn socket(&self) -> Socket {
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

fn edit_dialog(state: &Rc<AppState>, parent: &impl IsA<gtk::Widget>, proxy: Option<Proxy>) {
    let is_new = proxy.is_none();
    let proxy = proxy.unwrap_or_else(|| {
        let socket_type = SocketType::for_proxy_type(ProxyType::Forward)[0];
        let socket = Socket {
            socket_type,
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
    let proxy_type_row = adw::ComboRow::builder()
        .title(l10n::field_type())
        .model(&ui::string_list(&type_names))
        .selected(
            ProxyType::ALL
                .iter()
                .position(|t| *t == proxy.proxy_type)
                .unwrap_or(0) as u32,
        )
        .build();
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

    let state = state.clone();
    ui::form_dialog(
        parent,
        if is_new {
            l10n::dialog_title_proxy_add()
        } else {
            l10n::dialog_title_proxy_edit()
        },
        if is_new {
            l10n::button_add()
        } else {
            l10n::button_save()
        },
        &content,
        move || {
            let updated = Proxy {
                id: proxy.id,
                proxy_type: selected_type.get(),
                from: from.socket(),
                to: to.socket(),
            };
            state.update_db(|db| db.put_proxy(updated));
            true
        },
    );
}
