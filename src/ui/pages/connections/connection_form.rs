//! The name and address fields of a connection, shared by the Edit dialog
//! and the Address tab of the Add dialog.

use adw::prelude::*;

use crate::l10n;
use crate::models::Connection;
use crate::ui::widgets::{self, Validation};

pub struct ConnectionForm {
    /// The fields with the validation message below them.
    pub widget: gtk::Box,
    pub name: adw::EntryRow,
    pub address: adw::EntryRow,
    validation: Validation,
    /// The connection being edited, or an empty one for a new connection.
    connection: Connection,
}

impl ConnectionForm {
    pub fn new(connection: Connection) -> ConnectionForm {
        let name = widgets::entry_row(l10n::field_name(), &connection.name);
        let address = widgets::entry_row(l10n::field_address(), &connection.address);
        let group = adw::PreferencesGroup::new();
        group.add(&name);
        group.add(&address);
        // The mDNS name identifies a paired device; it comes from pairing
        // and cannot be changed here.
        if !connection.mdns_name.is_empty() {
            group.add(
                &adw::ActionRow::builder()
                    .title(l10n::field_mdns_name())
                    .subtitle(&connection.mdns_name)
                    .use_markup(false)
                    .subtitle_selectable(true)
                    .css_classes(["property"])
                    .build(),
            );
        }
        let validation = Validation::new();
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);
        widget.append(&group);
        widget.append(&validation.label);
        ConnectionForm {
            widget,
            name,
            address,
            validation,
            connection,
        }
    }

    /// The connection with the entered name and address, or `None` after
    /// showing why the input is not valid.
    pub fn connection(&self) -> Option<Connection> {
        let validation = &self.validation;
        let name = validation.require(&self.name, l10n::error_missing_name())?;
        // A wireless debugging device is found by its mDNS name, so its
        // address may be left empty.
        let address = if self.connection.mdns_name.is_empty() {
            validation.require(&self.address, l10n::error_missing_address())?
        } else {
            self.address.text().trim().to_string()
        };
        Some(Connection {
            name,
            address,
            ..self.connection.clone()
        })
    }
}
