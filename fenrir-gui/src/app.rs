use adw::prelude::*;
use adw::Application;

use crate::main_window;

const APP_ID: &str = "io.github.xshaduwulfx.fenrir";

pub fn run() {
    adw::init().expect("Failed to initialize libadwaita");

    let app = Application::builder()
    .application_id(APP_ID)
    .build();

    app.connect_activate(main_window::build);
    app.run();
}
