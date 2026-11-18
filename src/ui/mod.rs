//! Widgets and helpers shared by the pages.

pub mod automations;
pub mod connections;
pub mod emulators;
pub mod install;
pub mod keyboard;
pub mod proxies;
pub mod remote_control;
pub mod screenshot;
pub mod settings;
pub mod window;

use std::rc::Rc;

use adw::prelude::*;

use crate::l10n;

pub type Callback = Rc<dyn Fn()>;

/// A page in the sidebar.
pub struct Page {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub widget: gtk::Widget,
    /// Shown as an "Add" button in the header bar.
    pub on_add: Option<Rc<dyn Fn()>>,
    /// Called each time the page becomes visible.
    pub on_show: Option<Rc<dyn Fn()>>,
}

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

pub fn status_label(text: &str, positive: bool) -> gtk::Label {
    gtk::Label::builder()
        .label(text)
        .valign(gtk::Align::Center)
        .css_classes([if positive { "success" } else { "dim-label" }])
        .build()
}

pub fn string_list(items: &[&str]) -> gtk::StringList {
    gtk::StringList::new(items)
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

/// A dialog with Cancel and a confirm button in its header bar. The dialog
/// closes when `on_confirm` returns true.
pub fn form_dialog(
    parent: &impl IsA<gtk::Widget>,
    title: &str,
    confirm_label: &str,
    content: &impl IsA<gtk::Widget>,
    on_confirm: impl Fn() -> bool + 'static,
) -> adw::Dialog {
    let dialog = adw::Dialog::builder()
        .title(title)
        .content_width(520)
        .build();

    let cancel = gtk::Button::with_label(l10n::button_cancel());
    let confirm = gtk::Button::builder()
        .label(confirm_label)
        .css_classes(["suggested-action"])
        .build();
    let header = adw::HeaderBar::builder()
        .show_start_title_buttons(false)
        .show_end_title_buttons(false)
        .build();
    header.pack_start(&cancel);
    header.pack_end(&confirm);

    let inner = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(18)
        .margin_start(18)
        .margin_end(18)
        .build();
    inner.append(content);
    let body = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_height(true)
        .max_content_height(640)
        .child(&inner)
        .build();

    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&header);
    toolbar.set_content(Some(&body));
    dialog.set_child(Some(&toolbar));
    dialog.set_default_widget(Some(&confirm));

    let weak = dialog.downgrade();
    cancel.connect_clicked(move |_| {
        if let Some(dialog) = weak.upgrade() {
            dialog.close();
        }
    });
    let weak = dialog.downgrade();
    confirm.connect_clicked(move |_| {
        if on_confirm()
            && let Some(dialog) = weak.upgrade() {
                dialog.close();
            }
    });

    dialog.present(Some(parent));
    dialog
}

/// Asks "Are you sure you want to delete {name}?" and runs `on_delete` on confirmation.
pub fn confirm_delete(
    parent: &impl IsA<gtk::Widget>,
    title: &str,
    name: &str,
    on_delete: impl Fn() + 'static,
) {
    let dialog = adw::AlertDialog::new(
        Some(title),
        Some(&l10n::dialog_message_confirm_delete(name)),
    );
    dialog.add_responses(&[
        ("cancel", l10n::button_cancel()),
        ("delete", l10n::button_delete()),
    ]);
    dialog.set_response_appearance("delete", adw::ResponseAppearance::Destructive);
    dialog.set_default_response(Some("cancel"));
    dialog.set_close_response("cancel");
    dialog.connect_response(Some("delete"), move |_, _| on_delete());
    dialog.present(Some(parent));
}

/// Marks an entry row as invalid until its text changes.
pub fn mark_invalid(row: &adw::EntryRow) {
    row.add_css_class("error");
    row.connect_changed(|row| row.remove_css_class("error"));
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
