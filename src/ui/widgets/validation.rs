//! Required-field validation for form dialogs.

use adw::prelude::*;

/// An entry row that drops its error highlight as soon as it is edited.
pub fn entry_row(title: &str, text: &str) -> adw::EntryRow {
    let row = adw::EntryRow::builder().title(title).text(text).build();
    row.connect_changed(|row| row.remove_css_class("error"));
    row
}

/// An error message shown under a form, and checks for required fields.
pub struct Validation {
    pub label: gtk::Label,
}

impl Validation {
    pub fn new() -> Validation {
        Validation {
            label: gtk::Label::builder()
                .css_classes(["error"])
                .xalign(0.0)
                .margin_top(6)
                .visible(false)
                .build(),
        }
    }

    /// The trimmed text of `row`, or `None` after highlighting the row and
    /// showing `message` when it is empty.
    pub fn require(&self, row: &adw::EntryRow, message: &str) -> Option<String> {
        let text = row.text().trim().to_string();
        if text.is_empty() {
            row.add_css_class("error");
            self.label.set_label(message);
            self.label.set_visible(true);
            return None;
        }
        self.label.set_visible(false);
        Some(text)
    }
}
