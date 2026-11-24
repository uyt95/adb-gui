//! A status line with a spinner, for progress and errors.

use adw::prelude::*;

pub struct Status {
    pub widget: gtk::Box,
    spinner: gtk::Spinner,
    label: gtk::Label,
}

impl Status {
    pub fn new() -> Status {
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

    pub fn progress(&self, text: &str) {
        self.show(text, true);
    }

    pub fn error(&self, text: &str) {
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
