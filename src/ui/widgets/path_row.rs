//! An entry row for a file or folder path with a Browse button.

use adw::prelude::*;
use gtk::gio;

use crate::l10n;

#[derive(Clone, Copy)]
pub enum PathKind {
    /// A file, optionally limited to one extension (without the dot).
    File {
        extension: Option<&'static str>,
    },
    Folder,
}

pub fn path_row(title: &str, kind: PathKind) -> adw::EntryRow {
    let row = adw::EntryRow::builder().title(title).build();
    let browse = gtk::Button::builder()
        .label(l10n::button_browse())
        .valign(gtk::Align::Center)
        .css_classes(["flat"])
        .build();
    row.add_suffix(&browse);

    browse.connect_clicked({
        let row = row.clone();
        move |button| {
            let dialog = gtk::FileDialog::builder()
                .accept_label(l10n::button_select())
                .build();
            let current = row.text();
            if !current.is_empty() {
                let current = gio::File::for_path(current.as_str());
                match kind {
                    PathKind::Folder => dialog.set_initial_folder(Some(&current)),
                    PathKind::File { .. } => dialog.set_initial_file(Some(&current)),
                }
            }
            let window = button.root().and_downcast::<gtk::Window>();
            let row = row.clone();
            let on_chosen = move |result: Result<gio::File, gtk::glib::Error>| {
                if let Some(path) = result.ok().and_then(|file| file.path()) {
                    row.set_text(&path.to_string_lossy());
                }
            };
            match kind {
                PathKind::Folder => {
                    dialog.select_folder(window.as_ref(), gio::Cancellable::NONE, on_chosen)
                }
                PathKind::File { extension } => {
                    if let Some(extension) = extension {
                        let filter = gtk::FileFilter::new();
                        filter.set_name(Some(extension));
                        filter.add_suffix(extension);
                        let filters = gio::ListStore::new::<gtk::FileFilter>();
                        filters.append(&filter);
                        dialog.set_filters(Some(&filters));
                        dialog.set_default_filter(Some(&filter));
                    }
                    dialog.open(window.as_ref(), gio::Cancellable::NONE, on_chosen)
                }
            }
        }
    });
    row
}
