//! The console panel: every command the app sends, with its output.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use adw::prelude::*;
use gtk::glib;

use crate::l10n;
use crate::tools::{CommandLog, Record, RecordState, format_duration};
use crate::ui::widgets::icon_button;

/// Styles for the console, added to the display once by the window.
pub const CSS: &str = "
.console { background-color: @view_bg_color; }
.console-toolbar { padding: 4px 6px 4px 12px; border-bottom: 1px solid alpha(currentColor, 0.12); }
.console-row { border-bottom: 1px solid alpha(currentColor, 0.08); }
.console-row-failed { background-color: alpha(@error_bg_color, 0.08); }
.console-output { border-radius: 6px; padding: 6px 8px; background-color: alpha(currentColor, 0.05); }
.console-dot { min-width: 7px; min-height: 7px; border-radius: 4px; background-color: @error_bg_color; }
";

const FAILED_CLASS: &str = "console-row-failed";

struct Row {
    row: gtk::ListBoxRow,
    revision: u64,
}

/// Builds the console panel. `on_hide` is called by its Hide button.
pub fn console(log: &CommandLog, on_hide: impl Fn() + 'static) -> gtk::Box {
    let panel = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["console"])
        .build();

    // Toolbar: title, counts, filter, Copy all, Clear, Hide.
    let toolbar = gtk::Box::builder()
        .spacing(6)
        .css_classes(["console-toolbar"])
        .build();
    toolbar.append(
        &gtk::Label::builder()
            .label(l10n::console_title())
            .css_classes(["heading"])
            .build(),
    );
    let count = gtk::Label::builder()
        .css_classes(["dim-label", "caption", "numeric"])
        .hexpand(true)
        .xalign(0.0)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .build();
    toolbar.append(&count);
    let show_all = gtk::ToggleButton::builder()
        .label(l10n::console_filter_all())
        .active(true)
        .build();
    let show_failed = gtk::ToggleButton::builder()
        .label(l10n::console_filter_failed())
        .group(&show_all)
        .build();
    let filter = gtk::Box::builder()
        .css_classes(["linked"])
        .valign(gtk::Align::Center)
        .build();
    filter.append(&show_all);
    filter.append(&show_failed);
    toolbar.append(&filter);
    let copy_button = icon_button("edit-copy-symbolic", l10n::button_copy_all(), {
        let log = log.clone();
        let toolbar = toolbar.clone();
        move || {
            let text: Vec<String> = log.snapshot().iter().map(Record::to_text).collect();
            toolbar.clipboard().set_text(&text.join("\n"));
        }
    });
    toolbar.append(&copy_button);
    toolbar.append(&icon_button(
        "edit-clear-all-symbolic",
        l10n::button_clear(),
        {
            let log = log.clone();
            move || log.clear()
        },
    ));
    toolbar.append(&icon_button(
        "go-down-symbolic",
        l10n::button_hide_console(),
        on_hide,
    ));
    panel.append(&toolbar);

    // The list, with a button over it that jumps back to the newest command.
    let placeholder = gtk::Label::builder()
        .label(l10n::console_empty())
        .css_classes(["dim-label"])
        .margin_top(24)
        .margin_bottom(24)
        .build();
    let list = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    list.set_placeholder(Some(&placeholder));
    let failed_only = Rc::new(Cell::new(false));
    list.set_filter_func({
        let failed_only = failed_only.clone();
        move |row| !failed_only.get() || row.has_css_class(FAILED_CLASS)
    });
    show_failed.connect_toggled({
        let list = list.clone();
        let failed_only = failed_only.clone();
        move |button| {
            failed_only.set(button.is_active());
            placeholder.set_label(if button.is_active() {
                l10n::console_no_failed()
            } else {
                l10n::console_empty()
            });
            list.invalidate_filter();
        }
    });
    let scrolled = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .child(&list)
        .build();
    let jump = gtk::Button::builder()
        .icon_name("go-bottom-symbolic")
        .tooltip_text(l10n::button_scroll_to_newest())
        .css_classes(["osd", "circular"])
        .halign(gtk::Align::End)
        .valign(gtk::Align::End)
        .margin_end(12)
        .margin_bottom(12)
        .visible(false)
        .build();
    jump.update_property(&[gtk::accessible::Property::Label(
        l10n::button_scroll_to_newest(),
    )]);
    let overlay = gtk::Overlay::builder().child(&scrolled).build();
    overlay.add_overlay(&jump);
    panel.append(&overlay);

    // Follow new commands while the list is scrolled to the bottom. Scrolling
    // up stops that; the jump button scrolls down and follows again.
    let following = Rc::new(Cell::new(true));
    let adjustment = scrolled.vadjustment();
    let to_bottom = |adjustment: &gtk::Adjustment| {
        adjustment.set_value(adjustment.upper() - adjustment.page_size());
    };
    adjustment.connect_value_changed({
        let following = following.clone();
        let jump = jump.clone();
        let last_value = Cell::new(0.0);
        move |adjustment| {
            // Only a move up stops following. New rows and a panel that grows
            // also change the value or the bottom, and must not stop it.
            let value = adjustment.value();
            if value >= adjustment.upper() - adjustment.page_size() - 2.0 {
                following.set(true);
            } else if value < last_value.get() {
                following.set(false);
            }
            last_value.set(value);
            jump.set_visible(!following.get());
        }
    });
    // Set when rows were added; growth without new rows comes from an expanded row.
    let rows_added = Rc::new(Cell::new(false));
    let last_page_size = Cell::new(0.0);
    adjustment.connect_changed({
        let following = following.clone();
        let rows_added = rows_added.clone();
        let jump = jump.clone();
        move |adjustment| {
            let resized = adjustment.page_size() != last_page_size.replace(adjustment.page_size());
            let added = rows_added.replace(false);
            if !following.get() {
                return;
            }
            if added || resized {
                // This runs while the list is being laid out, when a new scroll
                // position would not be drawn; scroll once that is done.
                let adjustment = adjustment.downgrade();
                glib::idle_add_local_once(move || {
                    if let Some(adjustment) = adjustment.upgrade() {
                        to_bottom(&adjustment);
                    }
                });
            } else if adjustment.value() < adjustment.upper() - adjustment.page_size() - 2.0 {
                // A row was expanded: keep it in view rather than jumping away.
                following.set(false);
                jump.set_visible(true);
            }
        }
    });
    jump.connect_clicked({
        let following = following.clone();
        let adjustment = adjustment.clone();
        move |_| {
            following.set(true);
            to_bottom(&adjustment);
        }
    });

    let rows: Rc<RefCell<HashMap<u64, Row>>> = Rc::default();
    let refresh = {
        let log = log.clone();
        let list = list.clone();
        move || {
            let records = log.snapshot();
            let mut rows = rows.borrow_mut();
            rows.retain(|id, row| {
                let keep = records.iter().any(|r| r.id == *id);
                if !keep {
                    list.remove(&row.row);
                }
                keep
            });
            for record in &records {
                match rows.get_mut(&record.id) {
                    Some(row) if row.revision == record.revision => {}
                    Some(row) => {
                        let expanded = row
                            .row
                            .child()
                            .and_downcast::<gtk::Expander>()
                            .is_some_and(|e| e.is_expanded());
                        fill_row(&row.row, record, expanded);
                        row.revision = record.revision;
                    }
                    None => {
                        let row = gtk::ListBoxRow::builder()
                            .activatable(false)
                            .css_classes(["console-row"])
                            .build();
                        fill_row(&row, record, false);
                        list.append(&row);
                        rows_added.set(true);
                        rows.insert(
                            record.id,
                            Row {
                                row,
                                revision: record.revision,
                            },
                        );
                    }
                }
            }
            let failed = records.iter().filter(|r| r.is_failed()).count();
            count.set_label(&l10n::console_count(records.len(), failed));
            list.invalidate_filter();
        }
    };
    refresh();
    log.subscribe(refresh);
    panel
}

/// Shows `record` in `row`, keeping the row's place in the list.
fn fill_row(row: &gtk::ListBoxRow, record: &Record, expanded: bool) {
    if record.is_failed() {
        row.add_css_class(FAILED_CLASS);
    } else {
        row.remove_css_class(FAILED_CLASS);
    }
    let expander = gtk::Expander::builder()
        .label_widget(&summary(record))
        .child(&details(record))
        .expanded(expanded)
        .margin_start(8)
        .margin_end(8)
        .margin_top(4)
        .margin_bottom(4)
        .build();
    row.set_child(Some(&expander));
}

/// The always-visible line: status, send time, command, duration and exit code.
fn summary(record: &Record) -> gtk::Box {
    let line = gtk::Box::builder().spacing(8).hexpand(true).build();
    let status: gtk::Widget = match &record.state {
        RecordState::Running => gtk::Spinner::builder().spinning(true).build().upcast(),
        _ if record.is_failed() => icon("process-stop-symbolic", "error"),
        RecordState::Launched => icon("media-playback-start-symbolic", "dim-label"),
        _ => icon("object-select-symbolic", "success"),
    };
    line.append(&status);
    let time = gtk::Label::builder()
        .label(
            record
                .sent
                .format("%H:%M:%S")
                .map(|t| t.to_string())
                .unwrap_or_default(),
        )
        .tooltip_text(l10n::console_sent(&record.sent_text()))
        .css_classes(["dim-label", "monospace", "numeric"])
        .build();
    line.append(&time);
    line.append(
        &gtk::Label::builder()
            .label(record.short_command())
            .css_classes(["monospace"])
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .hexpand(true)
            .xalign(0.0)
            .build(),
    );
    let (meta, failed_meta) = match &record.state {
        RecordState::Running => (l10n::console_running().to_string(), false),
        RecordState::Launched => (l10n::console_launched().to_string(), false),
        RecordState::NotStarted(_) => (l10n::console_not_started().to_string(), true),
        RecordState::Finished(finished) => {
            let exit = finished
                .exit_code
                .map_or(l10n::console_signal().to_string(), |code| {
                    format!("exit {code}")
                });
            (
                format!("{} · {exit}", format_duration(finished.duration)),
                finished.exit_code != Some(0),
            )
        }
    };
    line.append(
        &gtk::Label::builder()
            .label(meta)
            .css_classes(if failed_meta {
                ["error", "numeric"]
            } else {
                ["dim-label", "numeric"]
            })
            .build(),
    );
    line
}

/// The expanded part: full send time and command, stdout, stderr and notes.
fn details(record: &Record) -> gtk::Box {
    let details = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_start(22)
        .margin_top(6)
        .margin_bottom(4)
        .build();
    details.append(&note(&l10n::console_sent(&record.sent_text())));
    details.append(&text_label(&format!("$ {}", record.full_command()), &[]));
    match &record.state {
        RecordState::Running => details.append(&note(l10n::console_running_note())),
        RecordState::Launched => details.append(&note(l10n::console_launched_note())),
        RecordState::NotStarted(error) => details.append(&output("stderr", error, true)),
        RecordState::Finished(finished) => {
            for (name, stream, is_error) in [
                ("stdout", &finished.stdout, false),
                ("stderr", &finished.stderr, true),
            ] {
                if stream.is_empty() {
                    continue;
                }
                details.append(&output(name, stream.text.trim_end(), is_error));
                if stream.dropped > 0 {
                    details.append(&note(&l10n::console_dropped(stream.dropped)));
                }
            }
            if finished.stdout.is_empty() && finished.stderr.is_empty() {
                details.append(&note(l10n::console_no_output()));
            }
            if finished.rejected {
                details.append(&note(l10n::console_rejected_note()));
            }
            if finished.exit_code.is_none() {
                details.append(&note(l10n::console_signal_note()));
            }
        }
    }
    details
}

fn icon(name: &str, class: &str) -> gtk::Widget {
    gtk::Image::builder()
        .icon_name(name)
        .css_classes([class])
        .build()
        .upcast()
}

fn note(text: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(text)
        .css_classes(["dim-label", "caption"])
        .wrap(true)
        .xalign(0.0)
        .build()
}

/// Selectable monospace text that wraps anywhere, so long lines stay visible.
fn text_label(text: &str, classes: &[&str]) -> gtk::Label {
    let label = gtk::Label::builder()
        .label(text)
        .css_classes(["monospace"])
        .selectable(true)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .xalign(0.0)
        .build();
    for class in classes {
        label.add_css_class(class);
    }
    label
}

fn output(name: &str, text: &str, is_error: bool) -> gtk::Box {
    let block = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(2)
        .css_classes(["console-output"])
        .build();
    let title = gtk::Label::builder()
        .label(name)
        .css_classes(["caption-heading"])
        .xalign(0.0)
        .build();
    let classes: &[&str] = if is_error { &["error"] } else { &[] };
    if is_error {
        title.add_css_class("error");
    } else {
        title.add_css_class("dim-label");
    }
    block.append(&title);
    block.append(&text_label(text, classes));
    block
}
