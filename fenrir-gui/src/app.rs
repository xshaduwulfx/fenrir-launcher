use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use adw::Application;

use crate::app_state::AppState;
use crate::main_window;

const APP_ID: &str = "io.github.xshaduwulfx.fenrir";

pub fn run() {
    adw::init().expect("Failed to initialize libadwaita");

    let state = Rc::new(RefCell::new(
        AppState::new().expect("Failed to initialize application state"),
    ));

    let app = Application::builder()
    .application_id(APP_ID)
    .build();

    let state_for_window = Rc::clone(&state);

    app.connect_activate(move |app| {
        main_window::build(app, Rc::clone(&state_for_window));
    });

    app.run();
}
