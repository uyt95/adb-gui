//! Adding a connection: pairing for wireless debugging (Android 11+), with a
//! pairing code or a QR code, or a plain address for classic network debugging.
//! Each page is its own module; this dialog holds them, lists them on its
//! first page, and looks for devices and runs the pairing.

mod address_page;
mod code_page;
mod page;
mod qr_page;
mod status;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use gtk::glib;

use crate::l10n;
use crate::state::AppState;
use crate::tools::{MdnsService, MdnsServiceType, Paired};
use address_page::AddressPage;
use code_page::CodePage;
use page::{page_content, scrolled, subpage};
use qr_page::QrPage;

/// How often the dialog looks for devices that are ready to pair.
const POLL_INTERVAL: Duration = Duration::from_secs(2);

struct AddDialog {
    state: Rc<AppState>,
    dialog: adw::Dialog,
    navigation: adw::NavigationView,
    closed: Cell<bool>,
    /// A pairing runs; no second one starts meanwhile.
    pairing: Cell<bool>,
    /// An mDNS lookup runs; the next poll skips its turn.
    polling: Cell<bool>,
    code_page: Rc<CodePage>,
    qr_page: QrPage,
    address_page: AddressPage,
}

/// Shows the dialog for adding a connection. It opens on a list of the
/// three ways to add one; each leads to its own page with a back button,
/// so the header bar only ever holds that page's title and action.
pub fn add_dialog(state: &Rc<AppState>, parent: &impl IsA<gtk::Widget>) {
    let navigation = adw::NavigationView::builder().width_request(560).build();
    let dialog = adw::Dialog::builder()
        .title(l10n::dialog_title_connection_add())
        .content_width(560)
        // Each page is as tall as it needs; the QR code page is taller than
        // the first one.
        .follows_content_size(true)
        .child(&navigation)
        .build();

    let this = Rc::new(AddDialog {
        state: state.clone(),
        dialog: dialog.clone(),
        navigation: navigation.clone(),
        closed: Cell::new(false),
        pairing: Cell::new(false),
        polling: Cell::new(false),
        code_page: CodePage::new(),
        qr_page: QrPage::new(),
        address_page: AddressPage::new(),
    });

    let choices = adw::PreferencesGroup::new();
    let content = page_content(l10n::add_connection_help());
    content.append(&choices);
    navigation.add(&subpage(
        "choose",
        l10n::dialog_title_connection_add(),
        &scrolled(&content),
        None,
    ));
    let subpages = [
        (
            &this.code_page.page,
            "input-dialpad-symbolic",
            l10n::add_by_code_description(),
        ),
        (
            &this.qr_page.page,
            "camera-photo-symbolic",
            l10n::add_by_qr_description(),
        ),
        (
            &this.address_page.page,
            "network-wired-symbolic",
            l10n::add_by_address_description(),
        ),
    ];
    for (page, icon, description) in subpages {
        navigation.add(page);
        let row = adw::ActionRow::builder()
            .title(page.title())
            .subtitle(description)
            .activatable(true)
            .build();
        row.add_prefix(&gtk::Image::from_icon_name(icon));
        row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));
        let (navigation, page) = (navigation.clone(), page.clone());
        row.connect_activated(move |_| navigation.push(&page));
        choices.add(&row);
    }

    let weak = Rc::downgrade(&this);
    navigation.connect_visible_page_notify(move |_| {
        if let Some(this) = weak.upgrade() {
            this.page_changed();
        }
    });
    for button in [&this.code_page.pair_button, &this.address_page.add_button] {
        let weak = Rc::downgrade(&this);
        button.connect_clicked(move |_| {
            if let Some(this) = weak.upgrade() {
                this.confirm();
            }
        });
    }
    for row in [&this.code_page.code, &this.address_page.form.address] {
        let weak = Rc::downgrade(&this);
        row.connect_entry_activated(move |_| {
            if let Some(this) = weak.upgrade() {
                this.confirm();
            }
        });
    }

    // The closures above only hold weak references. This one keeps the
    // dialog's state alive until the dialog closes.
    let keeper = RefCell::new(Some(this.clone()));
    dialog.connect_closed(move |_| {
        if let Some(this) = keeper.take() {
            this.closed.set(true);
        }
    });

    let weak = Rc::downgrade(&this);
    glib::timeout_add_local(POLL_INTERVAL, move || match weak.upgrade() {
        Some(this) if !this.closed.get() => {
            this.poll();
            glib::ControlFlow::Continue
        }
        _ => glib::ControlFlow::Break,
    });

    dialog.present(Some(parent));
}

impl AddDialog {
    /// The tag of the visible page.
    fn page(&self) -> String {
        self.navigation
            .visible_page()
            .and_then(|page| page.tag())
            .map(|tag| tag.to_string())
            .unwrap_or_default()
    }

    fn page_changed(self: &Rc<Self>) {
        match self.page().as_str() {
            code_page::TAG => self.code_page.address.grab_focus(),
            address_page::TAG => self.address_page.form.name.grab_focus(),
            _ => false,
        };
        // Look right away instead of after the next interval.
        self.poll();
    }

    /// Pair is greyed out while a pairing runs. Add on the address page
    /// stays usable: adding a connection does not affect the pairing.
    fn update_pair_button(&self) {
        self.code_page
            .pair_button
            .set_sensitive(!self.pairing.get());
    }

    fn confirm(self: &Rc<Self>) {
        match self.page().as_str() {
            address_page::TAG => {
                // Saves a connection for classic network debugging.
                if let Some(connection) = self.address_page.form.connection() {
                    self.state.update_db(|db| db.put_connection(connection));
                    self.dialog.close();
                }
            }
            code_page::TAG => {
                if let Some((address, code)) = self.code_page.input() {
                    self.pair(address, code, false);
                }
            }
            _ => {}
        }
    }

    /// Looks for pairing services: those to list on the pairing code page, and the one
    /// of a device that scanned the QR code.
    fn poll(self: &Rc<Self>) {
        if self.closed.get()
            || self.pairing.get()
            || self.polling.get()
            || !matches!(self.page().as_str(), code_page::TAG | qr_page::TAG)
        {
            return;
        }
        self.polling.set(true);
        let weak = Rc::downgrade(self);
        self.state.run(
            |tools| tools.unlogged().mdns_services(),
            move |_, services| {
                let Some(this) = weak.upgrade() else { return };
                this.polling.set(false);
                if this.closed.get() {
                    return;
                }
                let qr_name = this.qr_page.name();
                let (scanned, waiting): (Vec<MdnsService>, Vec<MdnsService>) = services
                    .into_iter()
                    .filter(|s| s.service_type == MdnsServiceType::Pairing)
                    .partition(|s| s.instance == qr_name);
                this.code_page.show_found(waiting);
                if let Some(service) = scanned.into_iter().next()
                    && this.page() == qr_page::TAG
                {
                    this.pair(service.address, this.qr_page.password(), true);
                }
            },
        );
    }

    /// Pairs with the device, then saves and connects it and closes the dialog.
    fn pair(self: &Rc<Self>, address: String, code: String, from_qr: bool) {
        if self.pairing.replace(true) {
            return;
        }
        let status = if from_qr {
            &self.qr_page.status
        } else {
            &self.code_page.status
        };
        status.progress(l10n::pair_in_progress());
        self.update_pair_button();
        let weak = Rc::downgrade(self);
        self.state.run(
            move |tools| tools.pair_and_connect(&address, &code),
            move |state, result| {
                // The dialog may have been closed while pairing ran. The
                // device is paired in adb either way, so save it first and
                // only touch the dialog if it still exists.
                let this = weak.upgrade();
                match result {
                    Ok(paired) => {
                        save_paired(state, paired);
                        state.load_devices();
                        if let Some(this) = this {
                            this.pairing.set(false);
                            this.dialog.close();
                        }
                    }
                    Err(error) => {
                        let Some(this) = this else {
                            state.show_error(&l10n::error_message(&error));
                            return;
                        };
                        this.pairing.set(false);
                        this.update_pair_button();
                        if from_qr {
                            this.qr_page.renew();
                            this.qr_page
                                .status
                                .error(&l10n::error_pair_qr_failed(&error));
                        } else {
                            this.code_page.status.error(&l10n::error_message(&error));
                        }
                    }
                }
            },
        );
    }
}

/// Saves a newly paired device. A saved connection for the same device, by
/// mDNS name, is updated and keeps its name, so pairing again does not add a
/// duplicate. Any other pairing is added as a new connection: an address
/// alone can belong to another device (a reused IP, a forwarded port), so it
/// never decides which saved connection is replaced.
fn save_paired(state: &AppState, paired: Paired) {
    let Paired {
        mut connection,
        connect_error,
    } = paired;
    if !connection.mdns_name.is_empty()
        && let Some(existing) = state
            .db
            .borrow()
            .connections
            .iter()
            .find(|c| c.mdns_name == connection.mdns_name)
    {
        connection.id = existing.id;
        connection.name = existing.name.clone();
    }
    let name = connection.name.clone();
    state.update_db(|db| db.put_connection(connection));
    match connect_error {
        None => state.show_message(&l10n::paired_with(&name)),
        Some(error) => state.show_error(&l10n::error_paired_not_connected(&name, &error)),
    }
}
