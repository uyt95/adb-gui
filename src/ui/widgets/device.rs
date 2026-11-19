//! Keeping widgets sensitive only while a device is selected.

use std::rc::Rc;

use adw::prelude::*;

use crate::state::AppState;

/// Makes `widget` sensitive only while a device is selected and `condition`
/// holds. Returns the update function, to call when `condition` may change
/// for reasons other than the app state.
pub fn bind_sensitive(
    state: &Rc<AppState>,
    widget: &impl IsA<gtk::Widget>,
    condition: impl Fn() -> bool + 'static,
) -> Rc<dyn Fn()> {
    let update: Rc<dyn Fn()> = {
        let state = state.clone();
        let widget = widget.clone().upcast::<gtk::Widget>();
        Rc::new(move || widget.set_sensitive(state.active_device().is_some() && condition()))
    };
    update();
    state.subscribe({
        let update = update.clone();
        move || update()
    });
    update
}

/// Makes `widget` sensitive only while a device is selected.
pub fn bind_to_device(state: &Rc<AppState>, widget: &impl IsA<gtk::Widget>) {
    bind_sensitive(state, widget, || true);
}
