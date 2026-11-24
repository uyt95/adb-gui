//! Adding a connection: pairing for wireless debugging (Android 11+), with a
//! pairing code or a QR code, or a plain address for classic network debugging.

use std::cell::{Cell, RefCell};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::rc::{Rc, Weak};
use std::time::Duration;

use adw::prelude::*;
use gtk::glib;

use crate::l10n;
use crate::models::Connection;
use crate::state::AppState;
use crate::tools::{MdnsService, MdnsServiceType, Paired};
use crate::ui::widgets::{self, Validation};

/// How often the dialog looks for devices that are ready to pair.
const POLL_INTERVAL: Duration = Duration::from_secs(2);

const TAB_CODE: &str = "code";
const TAB_QR: &str = "qr";
const TAB_ADDRESS: &str = "address";

/// A status line with a spinner, for progress and errors.
struct Status {
    widget: gtk::Box,
    spinner: gtk::Spinner,
    label: gtk::Label,
}

impl Status {
    fn new() -> Status {
        let spinner = gtk::Spinner::new();
        let label = gtk::Label::builder().wrap(true).xalign(0.0).build();
        let widget = gtk::Box::builder()
            .spacing(8)
            .margin_top(12)
            .visible(false)
            .build();
        widget.append(&spinner);
        widget.append(&label);
        Status {
            widget,
            spinner,
            label,
        }
    }

    fn progress(&self, text: &str) {
        self.show(text, true);
    }

    fn error(&self, text: &str) {
        self.show(text, false);
    }

    fn show(&self, text: &str, busy: bool) {
        self.label.set_label(text);
        self.spinner.set_visible(busy);
        self.spinner.set_spinning(busy);
        if busy {
            self.label.remove_css_class("error");
        } else {
            self.label.add_css_class("error");
        }
        self.widget.set_visible(true);
    }
}

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

    pair_address: adw::EntryRow,
    pair_code: adw::EntryRow,
    found_group: adw::PreferencesGroup,
    found_rows: RefCell<Vec<adw::ActionRow>>,
    /// The pairing services in `found_group`, to rebuild it only on a change.
    found: RefCell<Vec<MdnsService>>,
    code_validation: Validation,
    code_status: Status,

    /// The service name and password in the QR code. The device advertises
    /// a pairing service under this name once it has scanned the code.
    /// Both are replaced after a failed pairing, so the device must scan
    /// again instead of the dialog retrying the same service.
    qr_name: RefCell<String>,
    qr_password: RefCell<String>,
    /// Holds the QR code, so it can be redrawn with a new name and password.
    qr_frame: gtk::Box,
    qr_status: Status,

    name: adw::EntryRow,
    address: adw::EntryRow,
    address_validation: Validation,
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
        pair_address: widgets::entry_row(l10n::field_pairing_address(), ""),
        pair_code: widgets::entry_row(l10n::field_pairing_code(), ""),
        found_group: adw::PreferencesGroup::builder()
            .title(l10n::pair_code_found())
            .margin_top(18)
            .visible(false)
            .build(),
        found_rows: RefCell::default(),
        found: RefCell::default(),
        code_validation: Validation::new(),
        code_status: Status::new(),
        qr_name: RefCell::new(new_qr_name()),
        qr_password: RefCell::new(new_qr_password()),
        qr_frame: gtk::Box::builder().halign(gtk::Align::Center).build(),
        qr_status: Status::new(),
        name: widgets::entry_row(l10n::field_name(), ""),
        address: widgets::entry_row(l10n::field_address(), ""),
        address_validation: Validation::new(),
    });

    stack.add_titled_with_icon(
        &this.code_page(),
        Some(TAB_CODE),
        l10n::pair_tab_code(),
        "input-dialpad-symbolic",
    );
    stack.add_titled_with_icon(
        &this.qr_page(),
        Some(TAB_QR),
        l10n::pair_tab_qr(),
        "camera-photo-symbolic",
    );
    stack.add_titled_with_icon(
        &this.address_page(),
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
    for row in [&this.pair_code, &this.address] {
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
    this.pair_address.grab_focus();
}

impl AddDialog {
    fn code_page(self: &Rc<Self>) -> gtk::Widget {
        let group = adw::PreferencesGroup::new();
        group.add(&self.pair_address);
        group.add(&self.pair_code);
        let content = page_content(l10n::pair_code_help());
        content.append(&group);
        content.append(&self.code_validation.label);
        content.append(&self.code_status.widget);
        content.append(&self.found_group);
        scrolled(&content)
    }

    fn qr_page(self: &Rc<Self>) -> gtk::Widget {
        let content = page_content(l10n::pair_qr_help());
        self.draw_qr();
        content.append(&self.qr_frame);
        self.qr_status.widget.set_halign(gtk::Align::Center);
        self.qr_status.progress(l10n::pair_qr_waiting());
        content.append(&self.qr_status.widget);
        scrolled(&content)
    }

    fn draw_qr(&self) {
        while let Some(child) = self.qr_frame.first_child() {
            self.qr_frame.remove(&child);
        }
        let text = qr_text(&self.qr_name.borrow(), &self.qr_password.borrow());
        self.qr_frame.append(&qr_widget(&text));
    }

    /// Shows a new QR code after a failed pairing. The device has to scan
    /// it, which starts a fresh pairing service under the new name.
    fn renew_qr(&self) {
        self.qr_name.replace(new_qr_name());
        self.qr_password.replace(new_qr_password());
        self.draw_qr();
    }

    fn address_page(self: &Rc<Self>) -> gtk::Widget {
        let group = adw::PreferencesGroup::new();
        group.add(&self.name);
        group.add(&self.address);
        let content = page_content(l10n::pair_address_help());
        content.append(&group);
        content.append(&self.address_validation.label);
        scrolled(&content)
    }

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
            self.name.grab_focus();
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
            self.add_address();
            return;
        }
        let validation = &self.code_validation;
        let Some(address) = validation.require(&self.pair_address, l10n::error_missing_address())
        else {
            return;
        };
        let Some(code) = validation.require(&self.pair_code, l10n::error_missing_pairing_code())
        else {
            return;
        };
        self.pair(address, code, false);
    }

    /// Saves a connection for classic network debugging.
    fn add_address(&self) {
        let validation = &self.address_validation;
        let Some(name) = validation.require(&self.name, l10n::error_missing_name()) else {
            return;
        };
        let Some(address) = validation.require(&self.address, l10n::error_missing_address()) else {
            return;
        };
        self.state.update_db(|db| {
            db.put_connection(Connection {
                name,
                address,
                ..Connection::default()
            })
        });
        self.dialog.close();
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
                let (scanned, waiting): (Vec<MdnsService>, Vec<MdnsService>) = services
                    .into_iter()
                    .filter(|s| s.service_type == MdnsServiceType::Pairing)
                    .partition(|s| s.instance == *this.qr_name.borrow());
                this.show_found(waiting);
                if let Some(service) = scanned.into_iter().next()
                    && this.tab() == TAB_QR
                {
                    let password = this.qr_password.borrow().clone();
                    this.pair(service.address, password, true);
                }
            },
        );
    }

    /// Lists devices that show their pairing code, so their address can be
    /// picked instead of typed.
    fn show_found(self: &Rc<Self>, services: Vec<MdnsService>) {
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
            let weak: Weak<AddDialog> = Rc::downgrade(self);
            let address = service.address.clone();
            row.connect_activated(move |_| {
                if let Some(this) = weak.upgrade() {
                    this.pair_address.set_text(&address);
                    this.pair_code.grab_focus();
                }
            });
            self.found_group.add(&row);
            self.found_rows.borrow_mut().push(row);
        }
        self.found_group.set_visible(!services.is_empty());
        self.found.replace(services);
    }

    /// Pairs with the device, then saves and connects it and closes the dialog.
    fn pair(self: &Rc<Self>, address: String, code: String, from_qr: bool) {
        if self.pairing.replace(true) {
            return;
        }
        let status = if from_qr {
            &self.qr_status
        } else {
            &self.code_status
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
                            this.renew_qr();
                            this.qr_status.error(&l10n::error_pair_qr_failed(&error));
                        } else {
                            this.code_status.error(&l10n::error_message(&error));
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

/// The text of a QR code that Android's "Pair device with QR code" accepts.
fn qr_text(name: &str, password: &str) -> String {
    format!("WIFI:T:ADB;S:{name};P:{password};;")
}

/// Random letters and digits, leaving out ones that look alike.
fn new_qr_name() -> String {
    format!("adb-gui-{}", random_text(8))
}

fn new_qr_password() -> String {
    random_text(12)
}

fn random_text(length: usize) -> String {
    const ALPHABET: &[u8] = b"abcdefghijkmnpqrstuvwxyz23456789";
    // RandomState is seeded from the operating system's random source.
    let mut hasher = RandomState::new().build_hasher();
    (0..length)
        .map(|i| {
            hasher.write_usize(i);
            ALPHABET[(hasher.finish() % ALPHABET.len() as u64) as usize] as char
        })
        .collect()
}

/// Draws a QR code, black on white whatever the theme, so cameras read it.
fn qr_widget(text: &str) -> gtk::Widget {
    const SIZE: i32 = 220;
    let Ok(code) = qrcode::QrCode::new(text.as_bytes()) else {
        return gtk::Label::new(Some(text)).upcast();
    };
    let width = code.width();
    let dark: Vec<bool> = code
        .to_colors()
        .into_iter()
        .map(|color| color == qrcode::Color::Dark)
        .collect();
    let area = gtk::DrawingArea::builder()
        .content_width(SIZE)
        .content_height(SIZE)
        .halign(gtk::Align::Center)
        .margin_top(12)
        .build();
    area.set_draw_func(move |_, cr, w, h| {
        // A quiet zone of 2 modules on each side.
        let modules = (width + 4) as f64;
        let module = (w.min(h) as f64 / modules).floor().max(1.0);
        let offset_x = (w as f64 - module * modules) / 2.0 + 2.0 * module;
        let offset_y = (h as f64 - module * modules) / 2.0 + 2.0 * module;
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.rectangle(
            offset_x - 2.0 * module,
            offset_y - 2.0 * module,
            module * modules,
            module * modules,
        );
        let _ = cr.fill();
        cr.set_source_rgb(0.0, 0.0, 0.0);
        for (index, _) in dark.iter().enumerate().filter(|(_, dark)| **dark) {
            let (x, y) = ((index % width) as f64, (index / width) as f64);
            cr.rectangle(offset_x + x * module, offset_y + y * module, module, module);
        }
        let _ = cr.fill();
    });
    area.update_property(&[gtk::accessible::Property::Label(l10n::pair_tab_qr())]);
    area.upcast()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qr_text_uses_the_android_studio_format() {
        assert_eq!(
            qr_text("adb-gui-abc", "secret"),
            "WIFI:T:ADB;S:adb-gui-abc;P:secret;;"
        );
    }

    #[test]
    fn random_text_has_the_length_and_alphabet() {
        let text = random_text(12);
        assert_eq!(text.len(), 12);
        assert!(
            text.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        );
        assert_ne!(random_text(12), random_text(12));
    }
}
