use std::rc::Rc;

use adw::prelude::*;

use crate::l10n;
use crate::models::{Emulator, EmulatorParams};
use crate::state::AppState;
use crate::ui::{self, Page};

pub fn page(state: &Rc<AppState>) -> Page {
    let list = ui::boxed_list();
    let widget = ui::page_body(&list);

    let refresh = {
        let state = state.clone();
        let list = list.clone();
        move || {
            list.remove_all();
            for emulator in state.emulators() {
                list.append(&row(&state, &list, emulator));
            }
        }
    };
    refresh();
    state.subscribe(refresh);

    let on_show: Rc<dyn Fn()> = {
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
    let row = ui::action_row(&emulator.name, None);
    row.add_suffix(&ui::icon_button(
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
    row.add_suffix(&ui::icon_button(
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

fn params_dialog(state: &Rc<AppState>, parent: &gtk::ListBox, emulator: Emulator) {
    let no_snapshot_load = adw::SwitchRow::builder()
        .title(l10n::field_no_snapshot_load())
        .active(emulator.params.no_snapshot_load)
        .build();
    let writeable_system = adw::SwitchRow::builder()
        .title(l10n::field_writeable_system())
        .active(emulator.params.writeable_system)
        .build();
    let group = adw::PreferencesGroup::new();
    group.add(&no_snapshot_load);
    group.add(&writeable_system);

    let state = state.clone();
    ui::form_dialog(
        parent,
        l10n::dialog_title_emulator_params_edit(),
        l10n::button_save(),
        &group,
        move || {
            let updated = Emulator {
                params: EmulatorParams {
                    no_snapshot_load: no_snapshot_load.is_active(),
                    writeable_system: writeable_system.is_active(),
                },
                ..emulator.clone()
            };
            state.update_db(|db| db.put_emulator(updated));
            true
        },
    );
}
