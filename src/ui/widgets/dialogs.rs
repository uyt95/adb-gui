//! Form and confirmation dialogs.

use adw::prelude::*;

use crate::l10n;

/// Titles and confirm label of a dialog that either adds or edits a record.
pub struct FormTitles {
    pub add: &'static str,
    pub edit: &'static str,
}

impl FormTitles {
    pub fn title(&self, is_new: bool) -> &'static str {
        if is_new { self.add } else { self.edit }
    }

    pub fn confirm_label(is_new: bool) -> &'static str {
        if is_new {
            l10n::button_add()
        } else {
            l10n::button_save()
        }
    }
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
            && let Some(dialog) = weak.upgrade()
        {
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
