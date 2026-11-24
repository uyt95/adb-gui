//! Building blocks shared by the dialog's pages.

use adw::prelude::*;

/// A subpage of the dialog: a header bar with a back button, the title and
/// an optional action button, above `content`.
pub fn subpage(
    tag: &str,
    title: &str,
    content: &gtk::Widget,
    action: Option<&gtk::Button>,
) -> adw::NavigationPage {
    let header = adw::HeaderBar::new();
    if let Some(action) = action {
        header.pack_end(action);
    }
    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&header);
    toolbar.set_content(Some(content));
    adw::NavigationPage::builder()
        .tag(tag)
        .title(title)
        .child(&toolbar)
        .build()
}

pub fn suggested_button(label: &str) -> gtk::Button {
    gtk::Button::builder()
        .label(label)
        .css_classes(["suggested-action"])
        .build()
}

/// A page's content box, starting with its help text.
pub fn page_content(help: &str) -> gtk::Box {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(18)
        .margin_start(18)
        .margin_end(18)
        .build();
    content.append(
        &gtk::Label::builder()
            .label(help)
            .wrap(true)
            .xalign(0.0)
            .css_classes(["dim-label"])
            .build(),
    );
    content
}

pub fn scrolled(content: &gtk::Box) -> gtk::Widget {
    gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_height(true)
        .max_content_height(640)
        .child(content)
        .build()
        .upcast()
}
