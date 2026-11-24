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

const TAB_CODE: &str = "code";
const TAB_QR: &str = "qr";
const TAB_ADDRESS: &str = "address";

struct AddDialog {
    state: Rc<AppState>,
    dialog: adw::Dialog,
    stack: adw::ViewStack,
    confirm: gtk::Button,
    closed: Cell<bool>,
    /// A pairing runs; no second one starts meanwhile.
    pairing: Cell<bool>,
    /// An mDNS lookup runs; the next poll skips its turn.
    polling: Cell<bool>,
    code_tab: Rc<CodeTab>,
    qr_tab: QrTab,
    address_form: ConnectionForm,
}

/// Shows the dialog for adding a connection.
pub fn add_dialog(state: &Rc<AppState>, parent: &impl IsA<gtk::Widget>) {
    let stack = adw::ViewStack::new();
    let switcher = adw::ViewSwitcher::builder()
        .stack(&stack)
        .policy(adw::ViewSwitcherPolicy::Wide)
        .build();
    let cancel = gtk::Button::with_label(l10n::button_cancel());
    let confirm = gtk::Button::builder()
        .label(l10n::button_pair())
        .css_classes(["suggested-action"])
        .build();
    let header = adw::HeaderBar::builder()
        .show_start_title_buttons(false)
        .show_end_title_buttons(false)
        .title_widget(&switcher)
        .build();
    header.pack_start(&cancel);
    header.pack_end(&confirm);

    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&header);
    toolbar.set_content(Some(&stack));
    let dialog = adw::Dialog::builder()
        .title(l10n::dialog_title_connection_add())
        .content_width(560)
        .child(&toolbar)
        .build();
    dialog.set_default_widget(Some(&confirm));

    let this = Rc::new(AddDialog {
        state: state.clone(),
        dialog: dialog.clone(),
        stack: stack.clone(),
        confirm: confirm.clone(),
        closed: Cell::new(false),
        pairing: Cell::new(false),
        polling: Cell::new(false),
        code_tab: CodeTab::new(),
        qr_tab: QrTab::new(),
        address_form: ConnectionForm::new(Connection::default()),
    });

    let address_page = page_content(l10n::pair_address_help());
    address_page.append(&this.address_form.widget);
    stack.add_titled_with_icon(
        &this.code_tab.widget,
        Some(TAB_CODE),
        l10n::pair_tab_code(),
        "input-dialpad-symbolic",
    );
    stack.add_titled_with_icon(
        &this.qr_tab.widget,
        Some(TAB_QR),
        l10n::pair_tab_qr(),
        "camera-photo-symbolic",
    );
    stack.add_titled_with_icon(
        &scrolled(&address_page),
        Some(TAB_ADDRESS),
        l10n::pair_tab_address(),
        "network-wired-symbolic",
    );

    let weak = Rc::downgrade(&this);
    stack.connect_visible_child_name_notify(move |_| {
        if let Some(this) = weak.upgrade() {
            this.tab_changed();
        }
    });
    let weak = Rc::downgrade(&this);
    cancel.connect_clicked(move |_| {
        if let Some(this) = weak.upgrade() {
            this.dialog.close();
        }
    });
    let weak = Rc::downgrade(&this);
    confirm.connect_clicked(move |_| {
        if let Some(this) = weak.upgrade() {
            this.confirm();
        }
    });
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

    this.tab_changed();
    dialog.present(Some(parent));
    this.code_tab.address.grab_focus();
}

impl AddDialog {
    fn tab(&self) -> String {
        self.stack
            .visible_child_name()
            .map(|name| name.to_string())
            .unwrap_or_default()
    }

    fn tab_changed(self: &Rc<Self>) {
        let tab = self.tab();
        self.confirm.set_visible(tab != TAB_QR);
        self.confirm.set_label(if tab == TAB_ADDRESS {
            l10n::button_add()
        } else {
            l10n::button_pair()
        });
        self.update_confirm();
        if tab == TAB_ADDRESS {
            self.address_form.name.grab_focus();
        } else {
            // Look right away instead of after the next interval.
            self.poll();
        }
    }

    /// While a pairing runs, Pair is greyed out, but Add on the address
    /// tab still works: adding a connection does not affect the pairing.
    fn update_confirm(&self) {
        self.confirm
            .set_sensitive(self.tab() == TAB_ADDRESS || !self.pairing.get());
    }

    fn confirm(self: &Rc<Self>) {
        if self.tab() == TAB_ADDRESS {
            // Saves a connection for classic network debugging.
            if let Some(connection) = self.address_form.connection() {
                self.state.update_db(|db| db.put_connection(connection));
                self.dialog.close();
            }
        } else if let Some((address, code)) = self.code_tab.input() {
            self.pair(address, code, false);
        }
    }

    /// Looks for pairing services: those to list on the code tab, and the one
    /// of a device that scanned the QR code.
    fn poll(self: &Rc<Self>) {
        if self.closed.get()
            || self.pairing.get()
            || self.polling.get()
            || self.tab() == TAB_ADDRESS
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
                    && this.tab() == TAB_QR
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
        self.update_confirm();
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
                        this.update_confirm();
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

/// A tab's content box, starting with its help text.
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
