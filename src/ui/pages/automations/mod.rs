mod commands_editor;
mod edit_dialog;

use std::rc::Rc;

use adw::prelude::*;

use super::{Callback, Page};
use crate::l10n;
use crate::models::Automation;
use crate::state::AppState;
use crate::ui::widgets;

use edit_dialog::edit_dialog;

pub fn page(state: &Rc<AppState>) -> Page {
    let widget = widgets::list_page(
        state,
        widgets::EmptyState {
            icon: "media-playlist-repeat-symbolic",
            title: l10n::empty_automations_title(),
            description: l10n::empty_automations_description(),
        },
        |state| state.db.borrow().automations.clone(),
        row,
    );

    let on_add: Callback = {
        let state = state.clone();
        let parent = widget.clone();
        Rc::new(move || edit_dialog(&state, &parent, None))
    };

    Page {
        id: "automations",
        title: l10n::page_title_automations(),
        icon: "media-playlist-repeat-symbolic",
        widget,
        on_add: Some(on_add),
        on_show: None,
    }
}

fn row(state: &Rc<AppState>, list: &gtk::ListBox, automation: Automation) -> adw::ActionRow {
    let row = widgets::action_row(&automation.name, None);

    let run = widgets::icon_button("media-playback-start-symbolic", l10n::button_run(), {
        let state = state.clone();
        let automation = automation.clone();
        move || {
            let Some(device) = state.active_device() else {
                return;
            };
            let automation = automation.clone();
            state.run_action(
                move |tools| tools.run_automation(&device, &automation),
                |_| {},
            );
        }
    });
    // Rows are rebuilt on every state change, so this needs no binding.
    run.set_sensitive(state.active_device().is_some());
    row.add_suffix(&run);

    row.add_suffix(&widgets::edit_button({
        let state = state.clone();
        let list = list.clone();
        let automation = automation.clone();
        move || edit_dialog(&state, &list, Some(automation.clone()))
    }));
    row.add_suffix(&widgets::icon_button(
        "edit-copy-symbolic",
        l10n::button_duplicate(),
        {
            let state = state.clone();
            let automation = automation.clone();
            move || {
                let copy = Automation {
                    id: 0,
                    name: format!("{} - {}", automation.name, l10n::copy_name_postfix()),
                    commands: automation.commands.clone(),
                };
                state.update_db(|db| db.put_automation(copy));
            }
        },
    ));
    row.add_suffix(&widgets::delete_button(
        list,
        l10n::dialog_title_automation_delete(),
        automation.name.clone(),
        {
            let state = state.clone();
            let id = automation.id;
            move || state.update_db(|db| db.automations.retain(|a| a.id != id))
        },
    ));
    row
}
