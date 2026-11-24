//! The "Pairing code" tab: the address and code a device shows, and a list
//! of devices on the network that are showing one.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use adw::prelude::*;

use super::status::Status;
use super::{page_content, scrolled};
use crate::l10n;
use crate::tools::MdnsService;
use crate::ui::widgets::{self, Validation};

pub struct CodeTab {
    pub widget: gtk::Widget,
    pub address: adw::EntryRow,
    pub code: adw::EntryRow,
    validation: Validation,
    pub status: Status,
    found_group: adw::PreferencesGroup,
    found_rows: RefCell<Vec<adw::ActionRow>>,
    /// The pairing services in `found_group`, to rebuild it only on a change.
    found: RefCell<Vec<MdnsService>>,
}

impl CodeTab {
    pub fn new() -> Rc<CodeTab> {
        let address = widgets::entry_row(l10n::field_pairing_address(), "");
        let code = widgets::entry_row(l10n::field_pairing_code(), "");
        let group = adw::PreferencesGroup::new();
        group.add(&address);
        group.add(&code);
        let validation = Validation::new();
        let status = Status::new();
        let found_group = adw::PreferencesGroup::builder()
            .title(l10n::pair_code_found())
            .margin_top(18)
            .visible(false)
            .build();
        let content = page_content(l10n::pair_code_help());
        content.append(&group);
        content.append(&validation.label);
        content.append(&status.widget);
        content.append(&found_group);
        Rc::new(CodeTab {
            widget: scrolled(&content),
            address,
            code,
            validation,
            status,
            found_group,
            found_rows: RefCell::default(),
            found: RefCell::default(),
        })
    }

    /// The entered address and code, or `None` after showing what is missing.
    pub fn input(&self) -> Option<(String, String)> {
        let validation = &self.validation;
        let address = validation.require(&self.address, l10n::error_missing_address())?;
        let code = validation.require(&self.code, l10n::error_missing_pairing_code())?;
        Some((address, code))
    }

    /// Lists devices that show their pairing code, so their address can be
    /// picked instead of typed.
    pub fn show_found(self: &Rc<Self>, services: Vec<MdnsService>) {
        if *self.found.borrow() == services {
            return;
        }
        for row in self.found_rows.take() {
            self.found_group.remove(&row);
        }
        for service in &services {
            let row = widgets::action_row(&service.address, Some(&service.instance));
            row.set_activatable(true);
            row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));
            let weak: Weak<CodeTab> = Rc::downgrade(self);
            let address = service.address.clone();
            row.connect_activated(move |_| {
                if let Some(this) = weak.upgrade() {
                    this.address.set_text(&address);
                    this.code.grab_focus();
                }
            });
            self.found_group.add(&row);
            self.found_rows.borrow_mut().push(row);
        }
        self.found_group.set_visible(!services.is_empty());
        self.found.replace(services);
    }
}
