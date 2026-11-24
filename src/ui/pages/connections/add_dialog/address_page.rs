//! The "Connect by address" page, for classic network debugging.

use adw::prelude::*;

use super::super::connection_form::ConnectionForm;
use super::page::{page_content, scrolled, subpage, suggested_button};
use crate::l10n;
use crate::models::Connection;

pub const TAG: &str = "address";

pub struct AddressPage {
    pub page: adw::NavigationPage,
    pub add_button: gtk::Button,
    pub form: ConnectionForm,
}

impl AddressPage {
    pub fn new() -> AddressPage {
        let form = ConnectionForm::new(Connection::default());
        let content = page_content(l10n::pair_address_help());
        content.append(&form.widget);
        let add_button = suggested_button(l10n::button_add());
        AddressPage {
            page: subpage(
                TAG,
                l10n::add_by_address(),
                &scrolled(&content),
                Some(&add_button),
            ),
            add_button,
            form,
        }
    }
}
