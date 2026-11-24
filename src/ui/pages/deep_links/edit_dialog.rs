use std::rc::Rc;

use adw::prelude::*;

use crate::l10n;
use crate::models::DeepLink;
use crate::state::AppState;
use crate::ui::widgets::{self, FormTitles, Validation};

/// Adds a link (id 0, possibly with the link already filled in) or edits a
/// saved one.
pub fn edit_dialog(state: &Rc<AppState>, parent: &impl IsA<gtk::Widget>, link: DeepLink) {
    let is_new = link.id == 0;

    let name = widgets::entry_row(l10n::field_name(), &link.name);
    let uri = widgets::entry_row(l10n::field_link(), &link.link);
    let package = widgets::entry_row(l10n::field_app_package(), &link.package);
    let group = adw::PreferencesGroup::new();
    group.add(&name);
    group.add(&uri);
    group.add(&package);
    let hint = gtk::Label::builder()
        .label(l10n::field_app_package_hint())
        .wrap(true)
        .xalign(0.0)
        .margin_top(6)
        .css_classes(["dim-label", "caption"])
        .build();
    let validation = Validation::new();
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.append(&group);
    content.append(&hint);
    content.append(&validation.label);

    let save = Rc::new({
        let state = state.clone();
        let (name, uri, package) = (name.clone(), uri.clone(), package.clone());
        move || {
            let Some(name) = validation.require(&name, l10n::error_missing_name()) else {
                return false;
            };
            let Some(uri) = validation.require(&uri, l10n::error_missing_link()) else {
                return false;
            };
            let updated = DeepLink {
                id: link.id,
                name,
                link: uri,
                package: package.text().trim().to_string(),
            };
            state.update_db(|db| db.put_deep_link(updated));
            true
        }
    });

    let titles = FormTitles {
        add: l10n::dialog_title_deep_link_add(),
        edit: l10n::dialog_title_deep_link_edit(),
    };
    let dialog = widgets::form_dialog(
        parent,
        titles.title(is_new),
        FormTitles::confirm_label(is_new),
        &content,
        {
            let save = save.clone();
            move || save()
        },
    );
    // Enter in any field saves, like the connection dialog.
    for row in [&name, &uri, &package] {
        let save = save.clone();
        let weak = dialog.downgrade();
        row.connect_entry_activated(move |_| {
            if save()
                && let Some(dialog) = weak.upgrade()
            {
                dialog.close();
            }
        });
    }
    name.grab_focus();
}
