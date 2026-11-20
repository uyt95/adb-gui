//! The header bar's drop-down for choosing the active device.

use std::cell::Cell;
use std::rc::Rc;

use adw::prelude::*;

use crate::l10n;
use crate::state::AppState;

/// Drop-down of connected devices, labelled with the saved connection name when known.
pub fn device_selector(state: &Rc<AppState>) -> gtk::DropDown {
    let model = gtk::StringList::new(&[]);
    let dropdown = gtk::DropDown::builder()
        .model(&model)
        .tooltip_text(l10n::field_device())
        .build();
    dropdown.update_property(&[gtk::accessible::Property::Label(l10n::field_device())]);
    // Ellipsize long device names in the button so the header bar fits
    // narrow windows; the popover list keeps the full names.
    dropdown.set_factory(Some(&label_factory(Some(12))));
    dropdown.set_list_factory(Some(&label_factory(None)));
    let updating = Rc::new(Cell::new(false));

    let refresh = {
        let state = state.clone();
        let dropdown = dropdown.clone();
        let updating = updating.clone();
        move || {
            let devices = state.devices.borrow().clone();
            let connections = state.db.borrow().connections.clone();
            let labels: Vec<String> = devices
                .iter()
                .map(|device| {
                    let name = connections
                        .iter()
                        .find(|c| device.matches_address(&c.address))
                        .map(|c| c.name.clone())
                        .unwrap_or_else(|| device.serial.clone());
                    // Show why a device cannot be used, e.g. "Phone (unauthorized)".
                    if device.is_ready() {
                        name
                    } else {
                        format!("{name} ({})", device.state)
                    }
                })
                .collect();
            let labels: Vec<&str> = if labels.is_empty() {
                vec![l10n::value_none()]
            } else {
                labels.iter().map(String::as_str).collect()
            };
            let active = state.active_device();
            let selected = active
                .and_then(|a| devices.iter().position(|d| *d == a))
                .unwrap_or(0);

            updating.set(true);
            model.splice(0, model.n_items(), &labels);
            dropdown.set_selected(selected as u32);
            dropdown.set_sensitive(!devices.is_empty());
            updating.set(false);
        }
    };
    refresh();
    state.subscribe(refresh);

    dropdown.connect_selected_notify({
        let state = state.clone();
        move |dropdown| {
            if updating.get() {
                return;
            }
            let device = state
                .devices
                .borrow()
                .get(dropdown.selected() as usize)
                .cloned();
            state.set_active_device(device);
        }
    });
    dropdown
}

/// Item factory showing a `gtk::StringObject` as a label, optionally ellipsized.
fn label_factory(max_width_chars: Option<i32>) -> gtk::SignalListItemFactory {
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_, item| {
        let label = gtk::Label::builder().xalign(0.0).build();
        if let Some(chars) = max_width_chars {
            label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            label.set_max_width_chars(chars);
            label.set_width_chars(chars.min(6));
        }
        item.downcast_ref::<gtk::ListItem>()
            .expect("list item")
            .set_child(Some(&label));
    });
    factory.connect_bind(|_, item| {
        let item = item.downcast_ref::<gtk::ListItem>().expect("list item");
        let text = item
            .item()
            .and_downcast::<gtk::StringObject>()
            .map(|s| s.string())
            .unwrap_or_default();
        if let Some(label) = item.child().and_downcast::<gtk::Label>() {
            label.set_label(&text);
        }
    });
    factory
}
