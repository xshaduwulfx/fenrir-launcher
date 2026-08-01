use adw::prelude::*;
use adw::{Application, ApplicationWindow};

use crate::widgets;

pub fn build(app: &Application) {
    let content = widgets::welcome_page::build();

    let window = ApplicationWindow::builder()
    .application(app)
    .title("Fenrir Launcher")
    .default_width(1200)
    .default_height(800)
    .content(&content)
    .build();

    window.present();
}
