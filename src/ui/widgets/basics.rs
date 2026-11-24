//! Small building blocks: buttons, rows, lists and page layout.

use adw::prelude::*;

use super::confirm_delete;
use crate::l10n;

pub fn icon_button(icon: &str, tooltip: &str, on_click: impl Fn() + 'static) -> gtk::Button {
    let button = gtk::Button::builder()
        .icon_name(icon)
        .tooltip_text(tooltip)
        .valign(gtk::Align::Center)
        .css_classes(["flat"])
        .build();
    button.update_property(&[gtk::accessible::Property::Label(tooltip)]);
    button.connect_clicked(move |_| on_click());
    button
}

/// The "Edit" button of a list row.
pub fn edit_button(on_click: impl Fn() + 'static) -> gtk::Button {
    icon_button("document-edit-symbolic", l10n::button_edit(), on_click)
}

/// The "Delete" button of a list row. It asks for confirmation before
/// calling `on_delete`.
pub fn delete_button(
    parent: &impl IsA<gtk::Widget>,
    title: &'static str,
    name: String,
    on_delete: impl Fn() + Clone + 'static,
) -> gtk::Button {
    let parent = parent.clone().upcast::<gtk::Widget>();
    icon_button("user-trash-symbolic", l10n::button_delete(), move || {
        confirm_delete(&parent, title, &name, on_delete.clone())
    })
}

/// An action row that shows its title and subtitle as plain text.
pub fn action_row(title: &str, subtitle: Option<&str>) -> adw::ActionRow {
    let row = adw::ActionRow::builder()
        .use_markup(false)
        .title(title)
        .build();
    if let Some(subtitle) = subtitle {
        row.set_subtitle(subtitle);
    }
    row
}

/// A status text for a row suffix, green when `positive`.
pub fn status_label(text: &str, positive: bool) -> gtk::Label {
    gtk::Label::builder()
        .label(text)
        .valign(gtk::Align::Center)
        .css_classes([if positive { "success" } else { "dim-label" }])
        .build()
}

/// A boxed list without selection.
pub fn boxed_list() -> gtk::ListBox {
    gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .css_classes(["boxed-list"])
        .build()
}

/// A scrollable, width-clamped page body.
pub fn page_body(child: &impl IsA<gtk::Widget>) -> gtk::Widget {
    let clamp = adw::Clamp::builder()
        .maximum_size(900)
        .child(child)
        .valign(gtk::Align::Start)
        .margin_top(18)
        .margin_bottom(18)
        .margin_start(12)
        .margin_end(12)
        .build();
    gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&clamp)
        .vexpand(true)
        .build()
        .upcast()
}

pub fn string_list(items: &[&str]) -> gtk::StringList {
    gtk::StringList::new(items)
}

/// A combo row offering `labels`, with `selected` preselected.
pub fn combo_row(title: &str, labels: &[&str], selected: usize) -> adw::ComboRow {
    adw::ComboRow::builder()
        .title(title)
        .model(&string_list(labels))
        .selected(selected as u32)
        .build()
}

/// Makes a combo row searchable by its item strings.
pub fn enable_search(row: &adw::ComboRow) {
    row.set_enable_search(true);
    row.set_expression(Some(gtk::PropertyExpression::new(
        gtk::StringObject::static_type(),
        None::<gtk::Expression>,
        "string",
    )));
}

/// Shows the whole label of each combo row item instead of cutting it off:
/// the selected value and the popover entries wrap onto more lines.
pub fn wrap_labels(row: &adw::ComboRow) {
    row.set_factory(Some(&label_factory(None)));
    row.set_list_factory(Some(&label_factory(Some(row))));
}

/// Builds labels for `gtk::StringObject` items. With `row`, it is the popover
/// list and each entry gets the check mark that shows the current selection.
fn label_factory(row: Option<&adw::ComboRow>) -> gtk::SignalListItemFactory {
    let factory = gtk::SignalListItemFactory::new();
    let in_popover = row.is_some();
    let row = row.map(|row| row.downgrade());
    let row_for_bind = row.clone();
    factory.connect_setup(move |_, item| {
        let Some(item) = item.downcast_ref::<gtk::ListItem>() else {
            return;
        };
        let label = gtk::Label::builder()
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .xalign(if in_popover { 0.0 } else { 1.0 })
            .justify(if in_popover {
                gtk::Justification::Left
            } else {
                gtk::Justification::Right
            })
            .max_width_chars(if in_popover { 36 } else { 24 })
            .build();
        if !in_popover {
            item.set_child(Some(&label));
            return;
        }
        let check = gtk::Image::from_icon_name("object-select-symbolic");
        check.set_valign(gtk::Align::Center);
        let content = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        label.set_hexpand(true);
        content.append(&label);
        content.append(&check);
        item.set_child(Some(&content));
        if let Some(row) = row.as_ref().and_then(|row| row.upgrade()) {
            let item = item.downgrade();
            let check = check.downgrade();
            row.connect_selected_item_notify(move |row| {
                if let (Some(item), Some(check)) = (item.upgrade(), check.upgrade()) {
                    update_check(row, &item, &check);
                }
            });
        }
    });
    factory.connect_bind(move |_, item| {
        let Some(item) = item.downcast_ref::<gtk::ListItem>() else {
            return;
        };
        let text = item
            .item()
            .and_downcast::<gtk::StringObject>()
            .map(|s| s.string())
            .unwrap_or_default();
        let child = item.child();
        let label = match child.as_ref().and_then(|c| c.downcast_ref::<gtk::Box>()) {
            Some(content) => content.first_child().and_downcast::<gtk::Label>(),
            None => child.and_downcast::<gtk::Label>(),
        };
        if let Some(label) = label {
            label.set_label(&text);
        }
        let check = item
            .child()
            .and_downcast::<gtk::Box>()
            .and_then(|content| content.last_child())
            .and_downcast::<gtk::Image>();
        if let (Some(row), Some(check)) = (row_for_bind.as_ref().and_then(|r| r.upgrade()), check) {
            update_check(&row, item, &check);
        }
    });
    factory
}

fn update_check(row: &adw::ComboRow, item: &gtk::ListItem, check: &gtk::Image) {
    let selected = row.selected_item().is_some() && row.selected_item() == item.item();
    check.set_opacity(if selected { 1.0 } else { 0.0 });
}
