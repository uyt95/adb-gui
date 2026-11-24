use std::rc::Rc;

use adw::prelude::*;

use super::connection_form::ConnectionForm;
use crate::l10n;
use crate::models::Connection;
use crate::state::AppState;
use crate::ui::widgets::{self, FormTitles};

/// Edits a saved connection.
pub fn edit_dialog(state: &Rc<AppState>, parent: &impl IsA<gtk::Widget>, connection: Connection) {
    let form = Rc::new(ConnectionForm::new(connection));

    let save = Rc::new({
        let state = state.clone();
        let form = form.clone();
        move || {
            let Some(updated) = form.connection() else {
                return false;
            };
            state.update_db(|db| db.put_connection(updated));
            true
        }
    });

    let dialog = widgets::form_dialog(
        parent,
        l10n::dialog_title_connection_edit(),
        FormTitles::confirm_label(false),
        &form.widget,
        {
            let save = save.clone();
            move || save()
        },
    );
    // Enter in the address field saves, like the Flutter app.
    let weak = dialog.downgrade();
    form.address.connect_entry_activated(move |_| {
        if save()
            && let Some(dialog) = weak.upgrade()
        {
            dialog.close();
        }
    });
    form.name.grab_focus();
}
