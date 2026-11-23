//! A page showing a list of records that is rebuilt on every state change.

use std::rc::Rc;

use adw::prelude::*;

use super::{boxed_list, page_body};
use crate::state::AppState;

/// What a list page shows instead of the list while it has no items.
pub struct EmptyState {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

/// Builds a boxed list page. After every state change the rows are rebuilt
/// from `items`, with `row` creating the row for each item. While there are
/// no items the page shows `empty` instead of the list. Returns the page body.
pub fn list_page<T: 'static>(
    state: &Rc<AppState>,
    empty: EmptyState,
    items: impl Fn(&AppState) -> Vec<T> + 'static,
    row: impl Fn(&Rc<AppState>, &gtk::ListBox, T) -> adw::ActionRow + 'static,
) -> gtk::Widget {
    let list = boxed_list();
    let empty_page = adw::StatusPage::builder()
        .icon_name(empty.icon)
        .title(empty.title)
        .description(empty.description)
        .vexpand(true)
        .build();
    let stack = gtk::Stack::new();
    stack.add_named(&page_body(&list), Some("list"));
    stack.add_named(&empty_page, Some("empty"));

    let refresh = {
        let state = state.clone();
        let list = list.clone();
        let stack = stack.clone();
        move || {
            list.remove_all();
            let items = items(&state);
            stack.set_visible_child_name(if items.is_empty() { "empty" } else { "list" });
            for item in items {
                list.append(&row(&state, &list, item));
            }
        }
    };
    refresh();
    state.subscribe(refresh);

    stack.upcast()
}
