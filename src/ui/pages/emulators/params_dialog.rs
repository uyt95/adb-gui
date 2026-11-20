use std::rc::Rc;

use adw::prelude::*;

use crate::l10n;
use crate::models::{Emulator, EmulatorParams};
use crate::state::AppState;
use crate::ui::widgets;

/// Edits the launch parameters of an emulator.
pub fn params_dialog(state: &Rc<AppState>, parent: &impl IsA<gtk::Widget>, emulator: Emulator) {
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
    widgets::form_dialog(
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
