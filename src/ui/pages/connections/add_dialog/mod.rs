//! Adding a connection: pairing for wireless debugging (Android 11+), with a
//! pairing code or a QR code, or a plain address for classic network debugging.
//! Each tab is its own widget; this dialog holds them and runs the pairing.

mod code_tab;
mod qr_tab;
mod status;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use gtk::glib;

use super::connection_form::ConnectionForm;
use crate::l10n;
use crate::models::Connection;
use crate::state::AppState;
use crate::tools::{MdnsService, MdnsServiceType, Paired};
use code_tab::CodeTab;
use qr_tab::QrTab;

/// How often the dialog looks for devices that are ready to pair.
const POLL_INTERVAL: Duration = Duration::from_secs(2);

const PAGE_CODE: &str = "code";
const PAGE_QR: &str = "qr";
const PAGE_ADDRESS: &str = "address";

/// One way to add a connection: a row on the first page and its own page.
type Subpage<'a> = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    gtk::Widget,
    Option<&'a gtk::Button>,
);

struct AddDialog {
    state: Rc<AppState>,
    dialog: adw::Dialog,
    navigation: adw::NavigationView,
    /// Pair on the pairing code page; greyed out while a pairing runs.
    pair_button: gtk::Button,
    closed: Cell<bool>,
    /// A pairing runs; no second one starts meanwhile.
    pairing: Cell<bool>,
    /// An mDNS lookup runs; the next poll skips its turn.
    polling: Cell<bool>,
    code_tab: Rc<CodeTab>,
    qr_tab: QrTab,
    address_form: ConnectionForm,
}

/// Shows the dialog for adding a connection. It opens on a list of the
/// three ways to add one; each leads to its own page with a back button,
/// so the header bar only ever holds that page's title and action.
pub fn add_dialog(state: &Rc<AppState>, parent: &impl IsA<gtk::Widget>) {
    let pair_button = suggested_button(l10n::button_pair());
    let add_button = suggested_button(l10n::button_add());
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
        pair_button: pair_button.clone(),
        closed: Cell::new(false),
        pairing: Cell::new(false),
        polling: Cell::new(false),
        code_tab: CodeTab::new(),
        qr_tab: QrTab::new(),
        address_form: ConnectionForm::new(Connection::default()),
    });

    let choices = adw::PreferencesGroup::new();
    let content = page_content(l10n::add_connection_help());
    content.append(&choices);
    navigation.add(&page(
        "choose",
        l10n::dialog_title_connection_add(),
        &scrolled(&content),
        None,
    ));

    let address_content = page_content(l10n::pair_address_help());
    address_content.append(&this.address_form.widget);
    let subpages: [Subpage; 3] = [
        (
            PAGE_CODE,
            "input-dialpad-symbolic",
            l10n::add_by_code(),
            l10n::add_by_code_description(),
            this.code_tab.widget.clone(),
            Some(&pair_button),
        ),
        (
            PAGE_QR,
            "camera-photo-symbolic",
            l10n::add_by_qr(),
            l10n::add_by_qr_description(),
            this.qr_tab.widget.clone(),
            None,
        ),
        (
            PAGE_ADDRESS,
            "network-wired-symbolic",
            l10n::add_by_address(),
            l10n::add_by_address_description(),
            scrolled(&address_content),
            Some(&add_button),
        ),
    ];
    for (tag, icon, title, description, content, action) in subpages {
        navigation.add(&page(tag, title, &content, action));
        let row = adw::ActionRow::builder()
            .title(title)
            .subtitle(description)
            .activatable(true)
            .build();
        row.add_prefix(&gtk::Image::from_icon_name(icon));
        row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));
        let navigation = navigation.clone();
        row.connect_activated(move |_| navigation.push_by_tag(tag));
        choices.add(&row);
    }

    let weak = Rc::downgrade(&this);
    navigation.connect_visible_page_notify(move |_| {
        if let Some(this) = weak.upgrade() {
            this.page_changed();
        }
    });
    for button in [&pair_button, &add_button] {
        let weak = Rc::downgrade(&this);
        button.connect_clicked(move |_| {
            if let Some(this) = weak.upgrade() {
                this.confirm();
            }
        });
    }
    for row in [&this.code_tab.code, &this.address_form.address] {
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
            PAGE_CODE => self.code_tab.address.grab_focus(),
            PAGE_ADDRESS => self.address_form.name.grab_focus(),
            _ => false,
        };
        // Look right away instead of after the next interval.
        self.poll();
    }

    /// Pair is greyed out while a pairing runs. Add on the address page
    /// stays usable: adding a connection does not affect the pairing.
    fn update_pair_button(&self) {
        self.pair_button.set_sensitive(!self.pairing.get());
    }

    fn confirm(self: &Rc<Self>) {
        match self.page().as_str() {
            PAGE_ADDRESS => {
                // Saves a connection for classic network debugging.
                if let Some(connection) = self.address_form.connection() {
                    self.state.update_db(|db| db.put_connection(connection));
                    self.dialog.close();
                }
            }
            PAGE_CODE => {
                if let Some((address, code)) = self.code_tab.input() {
                    self.pair(address, code, false);
                }
            }
            _ => {}
        }
    }

    /// Looks for pairing services: those to list on the code tab, and the one
    /// of a device that scanned the QR code.
    fn poll(self: &Rc<Self>) {
        if self.closed.get()
            || self.pairing.get()
            || self.polling.get()
            || !matches!(self.page().as_str(), PAGE_CODE | PAGE_QR)
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
                let qr_name = this.qr_tab.name();
                let (scanned, waiting): (Vec<MdnsService>, Vec<MdnsService>) = services
                    .into_iter()
                    .filter(|s| s.service_type == MdnsServiceType::Pairing)
                    .partition(|s| s.instance == qr_name);
                this.code_tab.show_found(waiting);
                if let Some(service) = scanned.into_iter().next()
                    && this.page() == PAGE_QR
                {
                    this.pair(service.address, this.qr_tab.password(), true);
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
            &self.qr_tab.status
        } else {
            &self.code_tab.status
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
                            this.qr_tab.renew();
                            this.qr_tab
                                .status
                                .error(&l10n::error_pair_qr_failed(&error));
                        } else {
                            this.code_tab.status.error(&l10n::error_message(&error));
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

/// A subpage of the dialog: a header bar with a back button, the title and
/// an optional action button, above `content`.
fn page(
    tag: &str,
    title: &str,
    content: &gtk::Widget,
    action: Option<&gtk::Button>,
) -> adw::NavigationPage {
    let header = adw::HeaderBar::new();
    if let Some(action) = action {
        header.pack_end(action);
    }
    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&header);
    toolbar.set_content(Some(content));
    adw::NavigationPage::builder()
        .tag(tag)
        .title(title)
        .child(&toolbar)
        .build()
}

fn suggested_button(label: &str) -> gtk::Button {
    gtk::Button::builder()
        .label(label)
        .css_classes(["suggested-action"])
        .build()
}

/// A page's content box, starting with its help text.
fn page_content(help: &str) -> gtk::Box {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(18)
        .margin_start(18)
        .margin_end(18)
        .build();
    content.append(
        &gtk::Label::builder()
            .label(help)
            .wrap(true)
            .xalign(0.0)
            .css_classes(["dim-label"])
            .build(),
    );
    content
}

fn scrolled(content: &gtk::Box) -> gtk::Widget {
    gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_height(true)
        .max_content_height(640)
        .child(content)
        .build()
        .upcast()
}
