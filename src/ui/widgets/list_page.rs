//! A page showing a list of records that is rebuilt on every state change.

use std::rc::Rc;

use super::{boxed_list, page_body};
use crate::state::AppState;

/// Builds a boxed list page. After every state change the rows are rebuilt
/// from `items`, with `row` creating the row for each item. Returns the
/// scrollable page body containing the list.
pub fn list_page<T: 'static>(
    state: &Rc<AppState>,
    items: impl Fn(&AppState) -> Vec<T> + 'static,
    row: impl Fn(&Rc<AppState>, &gtk::ListBox, T) -> adw::ActionRow + 'static,
) -> gtk::Widget {
    let list = boxed_list();
    let widget = page_body(&list);

    let refresh = {
        let state = state.clone();
        let list = list.clone();
        move || {
            list.remove_all();
            for item in items(&state) {
                list.append(&row(&state, &list, item));
            }
        }
    };
    refresh();
    state.subscribe(refresh);

    widget
}
