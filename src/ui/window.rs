use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use gtk::{gio, glib};

use crate::l10n;
use crate::state::AppState;
use crate::ui::console;
use crate::ui::device_selector::device_selector;
use crate::ui::pages::{self, Callback, Page};

pub fn build(app: &adw::Application, state: &Rc<AppState>) -> adw::ApplicationWindow {
    let pages: Rc<Vec<Page>> = Rc::new(pages::all(state));

    // Content: header bar with Add, device selector and refresh, above a stack of pages.
    let stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::Crossfade)
        .build();
    for page in pages.iter() {
        stack.add_named(&page.widget, Some(page.id));
    }

    let add_button = gtk::Button::builder()
        .icon_name("list-add-symbolic")
        .tooltip_text(l10n::button_add())
        .build();
    let refresh_button = gtk::Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text(l10n::button_refresh())
        .build();
    refresh_button.connect_clicked({
        let state = state.clone();
        move |_| state.load_devices()
    });

    // The device selector and refresh button live in the header bar, and move
    // to a bottom bar in narrow windows (see the breakpoint below).
    let device_controls = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    device_controls.append(&device_selector(state));
    device_controls.append(&refresh_button);

    // The console toggle. A dot on it tells about failed commands while the
    // console is hidden.
    let failure_dot = gtk::Box::builder()
        .css_classes(["console-dot"])
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .visible(false)
        .build();
    let toggle_icon = gtk::Overlay::builder()
        .child(&gtk::Image::from_icon_name("utilities-terminal-symbolic"))
        .build();
    toggle_icon.add_overlay(&failure_dot);
    let console_toggle = gtk::ToggleButton::builder()
        .child(&toggle_icon)
        .tooltip_text(l10n::console_toggle())
        .build();
    console_toggle.update_property(&[gtk::accessible::Property::Label(l10n::console_title())]);

    let content_header = adw::HeaderBar::new();
    content_header.pack_start(&add_button);
    // Packed first so it stays rightmost when the device controls move back in.
    content_header.pack_end(&console_toggle);
    content_header.pack_end(&device_controls);

    // The console sits below the pages; it starts hidden.
    let log = state.command_log.clone();
    let console_panel = console::console(&log, {
        let console_toggle = console_toggle.clone();
        move || console_toggle.set_active(false)
    });
    console_panel.set_visible(false);
    let paned = gtk::Paned::builder()
        .orientation(gtk::Orientation::Vertical)
        .start_child(&stack)
        .end_child(&console_panel)
        .shrink_start_child(false)
        .resize_end_child(false)
        .shrink_end_child(false)
        .build();
    let seen_failures = Rc::new(Cell::new(0u64));
    log.subscribe({
        let (log, console_toggle, failure_dot, seen_failures) = (
            log.clone(),
            console_toggle.clone(),
            failure_dot.clone(),
            seen_failures.clone(),
        );
        move || {
            if !console_toggle.is_active() && log.failed_total() > seen_failures.get() {
                failure_dot.set_visible(true);
            }
        }
    });
    console_toggle.connect_toggled({
        let (state, paned, console_panel) = (state.clone(), paned.clone(), console_panel.clone());
        move |toggle| {
            if toggle.is_active() {
                seen_failures.set(log.failed_total());
                failure_dot.set_visible(false);
                let saved = state.db.borrow().settings.console_height;
                let height = if saved > 0 {
                    saved
                } else {
                    DEFAULT_CONSOLE_HEIGHT
                };
                // Leave the page at least some room in short windows.
                let height = height
                    .min(paned.height() - MIN_PAGE_HEIGHT)
                    .max(MIN_CONSOLE_HEIGHT);
                console_panel.set_visible(true);
                paned.set_position(paned.height() - height);
            } else {
                // Failures shown while the console was open count as seen.
                seen_failures.set(log.failed_total());
                save_console_height(&state, &paned, &console_panel);
                console_panel.set_visible(false);
            }
        }
    });

    let bottom_bar = gtk::ActionBar::builder().revealed(false).build();

    let content_toolbar = adw::ToolbarView::new();
    content_toolbar.add_top_bar(&content_header);
    content_toolbar.add_bottom_bar(&bottom_bar);
    content_toolbar.set_content(Some(&paned));
    let content_page = adw::NavigationPage::builder()
        .child(&content_toolbar)
        .title(pages[0].title)
        .build();

    // Sidebar.
    let sidebar_list = gtk::ListBox::builder()
        .css_classes(["navigation-sidebar"])
        .build();
    for page in pages.iter() {
        let row_box = gtk::Box::builder()
            .spacing(12)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(6)
            .margin_end(6)
            .build();
        row_box.append(&gtk::Image::from_icon_name(page.icon));
        row_box.append(&gtk::Label::new(Some(page.title)));
        sidebar_list.append(&gtk::ListBoxRow::builder().child(&row_box).build());
    }
    let sidebar_toolbar = adw::ToolbarView::new();
    sidebar_toolbar.add_top_bar(&adw::HeaderBar::new());
    sidebar_toolbar.set_content(Some(
        &gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .child(&sidebar_list)
            .build(),
    ));
    let sidebar_page = adw::NavigationPage::builder()
        .child(&sidebar_toolbar)
        .title(l10n::app_name())
        .build();

    let split_view = adw::NavigationSplitView::builder()
        .sidebar(&sidebar_page)
        .content(&content_page)
        .min_sidebar_width(200.0)
        .build();

    let current_add: Rc<RefCell<Option<Callback>>> = Rc::default();
    add_button.connect_clicked({
        let current_add = current_add.clone();
        move |_| {
            let on_add = current_add.borrow().clone();
            if let Some(on_add) = on_add {
                on_add();
            }
        }
    });

    sidebar_list.connect_row_selected({
        let pages = pages.clone();
        let split_view = split_view.clone();
        move |_, row| {
            let Some(page) = row.and_then(|row| pages.get(row.index() as usize)) else {
                return;
            };
            stack.set_visible_child_name(page.id);
            content_page.set_title(page.title);
            add_button.set_visible(page.on_add.is_some());
            current_add.replace(page.on_add.clone());
            if let Some(on_show) = &page.on_show {
                on_show();
            }
            split_view.set_show_content(true);
        }
    });
    // Tapping the row that is already selected emits no `row-selected`, so in a
    // collapsed split view the user would be stuck on the sidebar after Back.
    sidebar_list.connect_row_activated({
        let split_view = split_view.clone();
        move |_, _| split_view.set_show_content(true)
    });
    sidebar_list.select_row(sidebar_list.row_at_index(0).as_ref());

    let toast_overlay = adw::ToastOverlay::new();
    toast_overlay.set_child(Some(&split_view));
    state.set_toast_overlay(&toast_overlay);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(l10n::app_name())
        .default_width(1000)
        .default_height(700)
        .width_request(360)
        .height_request(300)
        .content(&toast_overlay)
        .build();

    let breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
        adw::BreakpointConditionLengthType::MaxWidth,
        640.0,
        adw::LengthUnit::Sp,
    ));
    breakpoint.add_setter(&split_view, "collapsed", Some(&true.to_value()));
    breakpoint.connect_apply({
        let (content_header, bottom_bar, device_controls) = (
            content_header.clone(),
            bottom_bar.clone(),
            device_controls.clone(),
        );
        move |_| {
            content_header.remove(&device_controls);
            bottom_bar.set_center_widget(Some(&device_controls));
            bottom_bar.set_revealed(true);
        }
    });
    breakpoint.connect_unapply(move |_| {
        bottom_bar.set_revealed(false);
        bottom_bar.set_center_widget(None::<&gtk::Widget>);
        content_header.pack_end(&device_controls);
    });
    window.add_breakpoint(breakpoint);

    // Ctrl+` shows and hides the console.
    window.add_action(&gio::PropertyAction::new(
        "toggle-console",
        &console_toggle,
        "active",
    ));
    app.set_accels_for_action("win.toggle-console", &["<Control>grave"]);
    window.connect_close_request({
        let state = state.clone();
        move |_| {
            save_console_height(&state, &paned, &console_panel);
            glib::Propagation::Proceed
        }
    });

    let css = gtk::CssProvider::new();
    css.load_from_string(console::CSS);
    gtk::style_context_add_provider_for_display(
        &WidgetExt::display(&window),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window
}

const DEFAULT_CONSOLE_HEIGHT: i32 = 260;
const MIN_CONSOLE_HEIGHT: i32 = 120;
const MIN_PAGE_HEIGHT: i32 = 100;

/// Remembers the console's height while it is shown, for the next time it opens.
fn save_console_height(state: &Rc<AppState>, paned: &gtk::Paned, console: &gtk::Box) {
    if !console.is_visible() {
        return;
    }
    let height = paned.height() - paned.position();
    if height > 0 && height != state.db.borrow().settings.console_height {
        state.update_db(|db| db.settings.console_height = height);
    }
}
