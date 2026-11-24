//! The pages shown in the sidebar, one module each. Pages with dialogs or
//! editors have a directory with those in separate files.

mod automations;
mod connections;
mod deep_links;
mod emulators;
mod install;
mod keyboard;
mod proxies;
mod remote_control;
mod screenshot;
mod settings;

use std::rc::Rc;

use crate::state::AppState;

pub type Callback = Rc<dyn Fn()>;

/// A page in the sidebar.
pub struct Page {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub widget: gtk::Widget,
    /// Shown as an "Add" button in the header bar.
    pub on_add: Option<Callback>,
    /// Called each time the page becomes visible.
    pub on_show: Option<Callback>,
}

/// All pages, in sidebar order.
pub fn all(state: &Rc<AppState>) -> Vec<Page> {
    vec![
        connections::page(state),
        emulators::page(state),
        keyboard::page(state),
        remote_control::page(state),
        install::page(state),
        automations::page(state),
        proxies::page(state),
        screenshot::page(state),
        deep_links::page(state),
        settings::page(state),
    ]
}
