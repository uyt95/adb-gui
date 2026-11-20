use std::rc::Rc;

use adw::prelude::*;

use super::commands_editor::CommandsEditor;
use crate::l10n;
use crate::models::{Automation, Command, CommandType};
use crate::state::AppState;
use crate::ui::widgets::{self, FormTitles, Validation};

/// Adds an automation (`None`) or edits an existing one.
pub fn edit_dialog(
    state: &Rc<AppState>,
    parent: &impl IsA<gtk::Widget>,
    automation: Option<Automation>,
) {
    let is_new = automation.is_none();
    let automation = automation.unwrap_or_else(|| Automation {
        id: 0,
        name: String::new(),
        commands: vec![Command::new(CommandType::ALL[0])],
    });

    let name = widgets::entry_row(l10n::field_name(), &automation.name);
    let name_group = adw::PreferencesGroup::new();
    name_group.add(&name);
    let validation = Validation::new();

    let editor = CommandsEditor::new(automation.commands.clone());

    let content = gtk::Box::new(gtk::Orientation::Vertical, 12);
    content.append(&name_group);
    content.append(&validation.label);
    content.append(&editor.container);

    let titles = FormTitles {
        add: l10n::dialog_title_automation_add(),
        edit: l10n::dialog_title_automation_edit(),
    };
    let state = state.clone();
    let name_row = name.clone();
    widgets::form_dialog(
        parent,
        titles.title(is_new),
        FormTitles::confirm_label(is_new),
        &content,
        move || {
            let Some(name) = validation.require(&name_row, l10n::error_missing_name()) else {
                return false;
            };
            let updated = Automation {
                id: automation.id,
                name,
                commands: editor.commands.borrow().clone(),
            };
            state.update_db(|db| db.put_automation(updated));
            true
        },
    );
    name.grab_focus();
}
