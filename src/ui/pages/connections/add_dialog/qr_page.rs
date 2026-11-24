//! The "Pair with QR code" page: a code that a phone or tablet scans to pair.

use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

use adw::prelude::*;

use super::page::{page_content, scrolled, subpage};
use super::status::Status;
use crate::l10n;

pub const TAG: &str = "qr";

pub struct QrPage {
    pub page: adw::NavigationPage,
    pub status: Status,
    /// The service name and password in the QR code. The device advertises
    /// a pairing service under this name once it has scanned the code.
    /// Both are replaced after a failed pairing, so the device must scan
    /// again instead of the dialog retrying the same service.
    name: RefCell<String>,
    password: RefCell<String>,
    /// Holds the QR code, so it can be redrawn with a new name and password.
    frame: gtk::Box,
}

impl QrPage {
    pub fn new() -> QrPage {
        let frame = gtk::Box::builder().halign(gtk::Align::Center).build();
        let status = Status::new();
        status.widget.set_halign(gtk::Align::Center);
        status.progress(l10n::pair_qr_waiting());
        let content = page_content(l10n::pair_qr_help());
        content.append(&frame);
        content.append(&status.widget);
        let page = QrPage {
            page: subpage(TAG, l10n::add_by_qr(), &scrolled(&content), None),
            status,
            name: RefCell::new(new_name()),
            password: RefCell::new(new_password()),
            frame,
        };
        page.draw();
        page
    }

    pub fn name(&self) -> String {
        self.name.borrow().clone()
    }

    pub fn password(&self) -> String {
        self.password.borrow().clone()
    }

    /// Shows a new QR code after a failed pairing. The device has to scan
    /// it, which starts a fresh pairing service under the new name.
    pub fn renew(&self) {
        self.name.replace(new_name());
        self.password.replace(new_password());
        self.draw();
    }

    fn draw(&self) {
        while let Some(child) = self.frame.first_child() {
            self.frame.remove(&child);
        }
        let text = qr_text(&self.name.borrow(), &self.password.borrow());
        self.frame.append(&qr_widget(&text));
    }
}

/// The text of a QR code that Android's "Pair device with QR code" accepts.
fn qr_text(name: &str, password: &str) -> String {
    format!("WIFI:T:ADB;S:{name};P:{password};;")
}

fn new_name() -> String {
    format!("adb-gui-{}", random_text(8))
}

fn new_password() -> String {
    random_text(12)
}

/// Random letters and digits, leaving out ones that look alike.
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
    area.update_property(&[gtk::accessible::Property::Label(l10n::qr_code_label())]);
    area.upcast()
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
