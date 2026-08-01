use adw::prelude::*;
use adw::{
    Application,
    ApplicationWindow,
    HeaderBar,
    NavigationPage,
    NavigationSplitView,
    ToolbarView,
};

use crate::{pages, widgets};

pub fn build(app: &Application) {
    let sidebar = widgets::sidebar::build();
    let library = pages::library::build();

    let sidebar_page = NavigationPage::new(&sidebar, "Fenrir Launcher");
    let content_page = NavigationPage::new(&library, "Library");

    let split_view = NavigationSplitView::new();
    split_view.set_sidebar(Some(&sidebar_page));
    split_view.set_content(Some(&content_page));
    split_view.set_min_sidebar_width(220.0);
    split_view.set_max_sidebar_width(280.0);

    let header = HeaderBar::new();

    let toolbar_view = ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&split_view));

    let window = ApplicationWindow::builder()
    .application(app)
    .title("Fenrir Launcher")
    .default_width(1200)
    .default_height(800)
    .content(&toolbar_view)
    .build();

    window.present();
}
