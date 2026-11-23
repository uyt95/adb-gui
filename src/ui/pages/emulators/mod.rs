mod params_dialog;

use std::rc::Rc;

use adw::prelude::*;

use super::{Callback, Page};
use crate::l10n;
use crate::models::Emulator;
use crate::state::AppState;
use crate::ui::widgets;

use params_dialog::params_dialog;

pub fn page(state: &Rc<AppState>) -> Page {
    let widget = widgets::list_page(
        state,
        widgets::EmptyState {
            icon: "phone-symbolic",
            title: l10n::empty_emulators_title(),
            description: l10n::empty_emulators_description(),
        },
        |state| state.emulators(),
        row,
    );

    let on_show: Callback = {
        let state = state.clone();
        Rc::new(move || state.load_emulators())
    };

    Page {
        id: "emulators",
        title: l10n::page_title_emulators(),
        icon: "phone-symbolic",
        widget,
        on_add: None,
        on_show: Some(on_show),
    }
}

fn row(state: &Rc<AppState>, list: &gtk::ListBox, emulator: Emulator) -> adw::ActionRow {
    let row = widgets::action_row(&emulator.name, None);
    row.add_suffix(&widgets::icon_button(
        "media-playback-start-symbolic",
        l10n::button_run(),
        {
            let state = state.clone();
            let emulator = emulator.clone();
            move || {
                let emulator = emulator.clone();
                state.run_action(
                    move |tools| tools.start_emulator(&emulator.name, &emulator.params),
                    |_| {},
                );
            }
        },
    ));
    row.add_suffix(&widgets::icon_button(
        "emblem-system-symbolic",
        l10n::button_edit_parameters(),
        {
            let state = state.clone();
            let list = list.clone();
            move || params_dialog(&state, &list, emulator.clone())
        },
    ));
    row
}
