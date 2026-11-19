use std::rc::Rc;

use adw::prelude::*;
use gtk::gdk;

use super::Page;
use crate::l10n;
use crate::models::InstallMode;
use crate::state::AppState;
use crate::ui::widgets::{self, PathKind};

const EXTENSION: &str = "apk";

pub fn page(state: &Rc<AppState>) -> Page {
    let path = widgets::path_row(
        l10n::field_package_path(),
        PathKind::File {
            extension: Some(EXTENSION),
        },
    );

    let labels: Vec<&str> = InstallMode::ALL.iter().map(|m| m.label()).collect();
    let mode = adw::ComboRow::builder()
        .title(l10n::field_mode())
        .model(&widgets::string_list(&labels))
        .build();

    let group = adw::PreferencesGroup::new();
    group.add(&path);
    group.add(&mode);

    let warning = gtk::Label::builder()
        .label(l10n::warning_priv_app())
        .css_classes(["error"])
        .xalign(0.0)
        .wrap(true)
        .visible(false)
        .build();

    let install = gtk::Button::builder()
        .label(l10n::button_install())
        .css_classes(["suggested-action", "pill"])
        .halign(gtk::Align::Center)
        .build();
    let spinner = gtk::Spinner::builder()
        .halign(gtk::Align::Center)
        .visible(false)
        .build();

    let content = gtk::Box::new(gtk::Orientation::Vertical, 18);
    content.append(&group);
    content.append(&warning);
    content.append(&install);
    content.append(&spinner);

    mode.connect_selected_notify({
        let warning = warning.clone();
        move |mode| warning.set_visible(selected_mode(mode) == InstallMode::PrivApp)
    });

    let update = widgets::bind_sensitive(state, &install, {
        let path = path.clone();
        move || !path.text().is_empty()
    });
    path.connect_changed(move |_| update());

    install.connect_clicked({
        let state = state.clone();
        let (path, mode, spinner) = (path.clone(), mode.clone(), spinner.clone());
        move |button| {
            let Some(device) = state.active_device() else {
                return;
            };
            let file = path.text().to_string();
            let mode = selected_mode(&mode);
            button.set_visible(false);
            spinner.set_visible(true);
            spinner.start();
            let (button, spinner) = (button.clone(), spinner.clone());
            state.run_action(
                move |tools| tools.install(&device, &file, mode),
                move |_| {
                    spinner.stop();
                    spinner.set_visible(false);
                    button.set_visible(true);
                },
            );
        }
    });

    let widget = widgets::page_body(&content);

    // Accept a dropped .apk file anywhere on the page.
    let drop = gtk::DropTarget::new(gdk::FileList::static_type(), gdk::DragAction::COPY);
    drop.connect_drop({
        let path = path.clone();
        move |_, value, _, _| {
            let Ok(files) = value.get::<gdk::FileList>() else {
                return false;
            };
            let apk = files
                .files()
                .into_iter()
                .filter_map(|file| file.path())
                .find(|p| p.extension().is_some_and(|e| e == EXTENSION));
            match apk {
                Some(apk) => {
                    path.set_text(&apk.to_string_lossy());
                    true
                }
                None => false,
            }
        }
    });
    widget.add_controller(drop);

    Page {
        id: "install",
        title: l10n::page_title_install(),
        icon: "system-software-install-symbolic",
        widget,
        on_add: None,
        on_show: None,
    }
}

fn selected_mode(row: &adw::ComboRow) -> InstallMode {
    InstallMode::ALL
        .get(row.selected() as usize)
        .copied()
        .unwrap_or(InstallMode::Normal)
}
