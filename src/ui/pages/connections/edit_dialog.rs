use std::rc::Rc;

use adw::prelude::*;

use crate::l10n;
use crate::models::Connection;
use crate::state::AppState;
use crate::ui::widgets::{self, FormTitles, Validation};

/// Edits a saved connection.
pub fn edit_dialog(state: &Rc<AppState>, parent: &impl IsA<gtk::Widget>, connection: Connection) {
    let name = widgets::entry_row(l10n::field_name(), &connection.name);
    let address = widgets::entry_row(l10n::field_address(), &connection.address);
    let group = adw::PreferencesGroup::new();
    group.add(&name);
    group.add(&address);
    let validation = Validation::new();
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.append(&group);
    content.append(&validation.label);

    let save = Rc::new({
        let state = state.clone();
        let (name, address) = (name.clone(), address.clone());
        move || {
            let Some(name) = validation.require(&name, l10n::error_missing_name()) else {
                return false;
            };
            // A wireless debugging device is found by its mDNS name, so
            // its address may be left empty.
            let address = if connection.mdns_name.is_empty() {
                let Some(address) = validation.require(&address, l10n::error_missing_address())
                else {
                    return false;
                };
                address
            } else {
                address.text().trim().to_string()
            };
            let updated = Connection {
                name,
                address,
                ..connection.clone()
            };
            state.update_db(|db| db.put_connection(updated));
            true
        }
    });

    let dialog = widgets::form_dialog(
        parent,
        l10n::dialog_title_connection_edit(),
        FormTitles::confirm_label(false),
        &content,
        {
            let save = save.clone();
            move || save()
        },
    );
    // Enter in the address field saves, like the Flutter app.
    let weak = dialog.downgrade();
    address.connect_entry_activated(move |_| {
        if save()
            && let Some(dialog) = weak.upgrade()
        {
            dialog.close();
        }
    });
    name.grab_focus();
}
