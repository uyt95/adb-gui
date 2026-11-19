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

/// A boxed list that shows "No data" when empty.
pub fn boxed_list() -> gtk::ListBox {
    let list = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();
    let placeholder = gtk::Label::builder()
        .label(l10n::table_empty())
        .css_classes(["dim-label"])
        .margin_top(18)
        .margin_bottom(18)
        .build();
    list.set_placeholder(Some(&placeholder));
    list
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
