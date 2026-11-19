use std::cell::RefCell;
use std::rc::{Rc, Weak};

use adw::prelude::*;

use crate::l10n;
use crate::models::KEY_CODES;
use crate::models::{ArgumentType, Command, CommandType, Operation};
use crate::ui::widgets;

/// Edits a working copy of an automation's commands. The command cards are
/// rebuilt after each structural change (move, duplicate, delete, operation).
pub struct CommandsEditor {
    pub commands: RefCell<Vec<Command>>,
    pub container: gtk::Box,
    this: Weak<CommandsEditor>,
}

impl CommandsEditor {
    pub fn new(commands: Vec<Command>) -> Rc<CommandsEditor> {
        let editor = Rc::new_cyclic(|this| CommandsEditor {
            commands: RefCell::new(commands),
            container: gtk::Box::new(gtk::Orientation::Vertical, 18),
            this: this.clone(),
        });
        editor.rebuild();
        editor
    }

    /// Runs `change` on the commands from a signal handler, then rebuilds the cards.
    fn change(&self, change: impl FnOnce(&mut Vec<Command>) + 'static) -> impl Fn() + 'static {
        let this = self.this.clone();
        let change = RefCell::new(Some(change));
        move || {
            let Some(editor) = this.upgrade() else { return };
            if let Some(change) = change.take() {
                change(&mut editor.commands.borrow_mut());
            }
            // Rebuild after the signal handler returns, since it destroys the emitting widget.
            gtk::glib::idle_add_local_once(move || editor.rebuild());
        }
    }

    fn rebuild(&self) {
        while let Some(child) = self.container.first_child() {
            self.container.remove(&child);
        }
        let commands = self.commands.borrow().clone();
        let count = commands.len();
        for (index, command) in commands.iter().enumerate() {
            self.container.append(&self.card(index, count, command));
        }
        let add = widgets::icon_button(
            "list-add-symbolic",
            l10n::button_add(),
            self.change(|commands| commands.push(Command::new(CommandType::ALL[0]))),
        );
        add.set_halign(gtk::Align::Center);
        self.container.append(&add);
    }

    fn card(&self, index: usize, count: usize, command: &Command) -> adw::PreferencesGroup {
        let group = adw::PreferencesGroup::builder()
            .title(l10n::command_number(index + 1))
            .build();

        let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let up = widgets::icon_button(
            "go-up-symbolic",
            l10n::button_move_up(),
            self.change(move |c| c.swap(index, index - 1)),
        );
        up.set_sensitive(index > 0);
        let down = widgets::icon_button(
            "go-down-symbolic",
            l10n::button_move_down(),
            self.change(move |c| c.swap(index, index + 1)),
        );
        down.set_sensitive(index + 1 < count);
        let duplicate = widgets::icon_button(
            "edit-copy-symbolic",
            l10n::button_duplicate(),
            self.change(move |c| {
                let copy = c[index].clone();
                c.insert(index + 1, copy);
            }),
        );
        let delete = widgets::icon_button(
            "user-trash-symbolic",
            l10n::button_delete(),
            self.change(move |c| {
                c.remove(index);
            }),
        );
        delete.set_sensitive(count > 1);
        for button in [&up, &down, &duplicate, &delete] {
            buttons.append(button);
        }
        group.set_header_suffix(Some(&buttons));

        // Command type. Only ADB exists today, so selecting it changes nothing.
        let command_labels: Vec<&str> = CommandType::ALL.iter().map(|t| t.label()).collect();
        let command_row = adw::ComboRow::builder()
            .title("Command")
            .model(&widgets::string_list(&command_labels))
            .selected(
                CommandType::ALL
                    .iter()
                    .position(|t| *t == command.command_type)
                    .unwrap_or(0) as u32,
            )
            .build();
        group.add(&command_row);

        let operations = command.command_type.operations();
        let operation_labels: Vec<&str> = operations.iter().map(|o| o.label()).collect();
        let operation_row = adw::ComboRow::builder()
            .title("Operation")
            .model(&widgets::string_list(&operation_labels))
            .selected(
                operations
                    .iter()
                    .position(|o| *o == command.operation.operation_type)
                    .unwrap_or(0) as u32,
            )
            .build();
        let current = command.operation.operation_type;
        let this = self.this.clone();
        operation_row.connect_selected_notify(move |row| {
            let Some(&operation_type) = operations.get(row.selected() as usize) else {
                return;
            };
            if operation_type == current {
                return;
            }
            if let Some(editor) = this.upgrade() {
                editor.change(move |c| c[index].operation = Operation::new(operation_type))();
            }
        });
        group.add(&operation_row);

        for (argument_index, argument) in command.operation.arguments.iter().enumerate() {
            let set_value = {
                let this = self.this.clone();
                move |value: String| {
                    if let Some(editor) = this.upgrade() {
                        editor.commands.borrow_mut()[index].operation.arguments[argument_index]
                            .value = value;
                    }
                }
            };
            match argument.argument_type {
                ArgumentType::Static => {}
                ArgumentType::Text => {
                    let row = adw::EntryRow::builder()
                        .title(argument.argument_type.label())
                        .text(&argument.value)
                        .build();
                    row.connect_changed(move |row| set_value(row.text().to_string()));
                    group.add(&row);
                }
                ArgumentType::KeyCode => {
                    let labels: Vec<&str> = KEY_CODES.iter().map(|k| k.label).collect();
                    let row = adw::ComboRow::builder()
                        .title(argument.argument_type.label())
                        .model(&widgets::string_list(&labels))
                        .selected(
                            KEY_CODES
                                .iter()
                                .position(|k| k.code == argument.value)
                                .unwrap_or(0) as u32,
                        )
                        .build();
                    widgets::enable_search(&row);
                    row.connect_selected_notify(move |row| {
                        if let Some(key) = KEY_CODES.get(row.selected() as usize) {
                            set_value(key.code.to_string());
                        }
                    });
                    group.add(&row);
                }
            }
        }
        group
    }
}
