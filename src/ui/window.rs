use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;

use crate::l10n;
use crate::state::AppState;
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

    let content_header = adw::HeaderBar::new();
    content_header.pack_start(&add_button);
    content_header.pack_end(&device_controls);

    let bottom_bar = gtk::ActionBar::builder().revealed(false).build();

    let content_toolbar = adw::ToolbarView::new();
    content_toolbar.add_top_bar(&content_header);
    content_toolbar.add_bottom_bar(&bottom_bar);
    content_toolbar.set_content(Some(&stack));
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

    window
}
