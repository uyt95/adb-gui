mod edit_dialog;

use std::rc::Rc;

use adw::prelude::*;

use super::{Callback, Page};
use crate::l10n;
use crate::models::DeepLink;
use crate::state::AppState;
use crate::ui::widgets;

use edit_dialog::edit_dialog;

/// Opens deep links on the active device: a one-time link field above the
/// list of saved links.
pub fn page(state: &Rc<AppState>) -> Page {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 24);
    let widget = widgets::page_body(&content);

    // One-time link. Its text stays after Open, so it can be opened again or tweaked.
    let link = adw::EntryRow::builder().title(l10n::field_link()).build();
    let has_link = {
        let link = link.clone();
        move || !link.text().trim().is_empty()
    };
    let save = widgets::icon_button("bookmark-new-symbolic", l10n::button_save_link(), {
        let state = state.clone();
        let link = link.clone();
        let parent = widget.clone();
        move || {
            let text = link.text().trim().to_string();
            if text.is_empty() {
                return;
            }
            let new = DeepLink {
                link: text,
                ..DeepLink::default()
            };
            edit_dialog(&state, &parent, new);
        }
    });
    let open = gtk::Button::builder()
        .label(l10n::button_open())
        .valign(gtk::Align::Center)
        .css_classes(["suggested-action"])
        .build();
    link.add_suffix(&save);
    link.add_suffix(&open);

    let open_once = {
        let state = state.clone();
        let link = link.clone();
        move || {
            let text = link.text().trim().to_string();
            if text.is_empty() {
                return;
            }
            open_link(
                &state,
                DeepLink {
                    link: text,
                    ..DeepLink::default()
                },
            );
        }
    };
    open.connect_clicked({
        let open_once = open_once.clone();
        move |_| open_once()
    });
    link.connect_entry_activated(move |_| open_once());

    let update_open = widgets::bind_sensitive(state, &open, has_link.clone());
    link.connect_changed({
        let save = save.clone();
        move |_| {
            update_open();
            save.set_sensitive(has_link());
        }
    });
    save.set_sensitive(false);

    let once_group = adw::PreferencesGroup::builder()
        .title(l10n::group_open_link_title())
        .description(l10n::group_open_link_description())
        .build();
    once_group.add(&link);

    // Saved links, or a compact empty state while there are none. The page
    // keeps the one-time field, so the full-page empty state of other list
    // pages does not fit here.
    let list = widgets::boxed_list();
    let empty = adw::StatusPage::builder()
        .icon_name("bookmark-new-symbolic")
        .title(l10n::empty_deep_links_title())
        .description(l10n::empty_deep_links_description())
        .css_classes(["compact"])
        .build();
    let saved_stack = gtk::Stack::builder().vhomogeneous(false).build();
    saved_stack.add_named(&list, Some("list"));
    saved_stack.add_named(&empty, Some("empty"));
    let saved_group = adw::PreferencesGroup::builder()
        .title(l10n::group_saved_links_title())
        .build();
    saved_group.add(&saved_stack);

    let refresh = {
        let state = state.clone();
        let list = list.clone();
        move || {
            list.remove_all();
            let links = state.db.borrow().deep_links.clone();
            saved_stack.set_visible_child_name(if links.is_empty() { "empty" } else { "list" });
            for item in links {
                list.append(&row(&state, &list, item));
            }
        }
    };
    refresh();
    state.subscribe(refresh);

    content.append(&once_group);
    content.append(&saved_group);

    let on_add: Callback = {
        let state = state.clone();
        let parent = widget.clone();
        Rc::new(move || edit_dialog(&state, &parent, DeepLink::default()))
    };

    Page {
        id: "deep_links",
        title: l10n::page_title_deep_links(),
        icon: "insert-link-symbolic",
        widget,
        on_add: Some(on_add),
        on_show: None,
    }
}

fn open_link(state: &Rc<AppState>, link: DeepLink) {
    let Some(device) = state.active_device() else {
        return;
    };
    state.run_action(move |tools| tools.open_deep_link(&device, &link), |_| {});
}

fn row(state: &Rc<AppState>, list: &gtk::ListBox, link: DeepLink) -> adw::ActionRow {
    let subtitle = if link.package.is_empty() {
        link.link.clone()
    } else {
        format!("{}  ·  {}", link.link, link.package)
    };
    let row = widgets::action_row(&link.name, Some(&subtitle));
    row.set_subtitle_lines(1);
    row.set_tooltip_text(Some(&subtitle));

    let open = widgets::icon_button("media-playback-start-symbolic", l10n::button_open(), {
        let state = state.clone();
        let link = link.clone();
        move || open_link(&state, link.clone())
    });
    // Rows are rebuilt on every state change, so this needs no binding.
    open.set_sensitive(state.active_device().is_some());
    row.add_suffix(&open);

    row.add_suffix(&widgets::edit_button({
        let state = state.clone();
        let list = list.clone();
        let link = link.clone();
        move || edit_dialog(&state, &list, link.clone())
    }));
    row.add_suffix(&widgets::icon_button(
        "edit-copy-symbolic",
        l10n::button_duplicate(),
        {
            let state = state.clone();
            let link = link.clone();
            move || {
                let copy = DeepLink {
                    id: 0,
                    name: format!("{} - {}", link.name, l10n::copy_name_postfix()),
                    ..link.clone()
                };
                state.update_db(|db| db.put_deep_link(copy));
            }
        },
    ));
    row.add_suffix(&widgets::delete_button(
        list,
        l10n::dialog_title_deep_link_delete(),
        link.name.clone(),
        {
            let state = state.clone();
            let id = link.id;
            move || state.update_db(|db| db.deep_links.retain(|d| d.id != id))
        },
    ));
    row
}
