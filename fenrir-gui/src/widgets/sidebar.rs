use adw::prelude::*;
use gtk::{Align, Box, Button, Orientation, Separator};

pub fn build() -> Box {
    let library_button = Button::builder()
    .label("Library")
    .halign(Align::Fill)
    .hexpand(true)
    .build();

    let import_button = Button::builder()
    .label("Import Game")
    .halign(Align::Fill)
    .hexpand(true)
    .build();

    let settings_button = Button::builder()
    .label("Settings")
    .halign(Align::Fill)
    .hexpand(true)
    .build();

    let sidebar = Box::new(Orientation::Vertical, 12);

    sidebar.set_margin_top(16);
    sidebar.set_margin_bottom(16);
    sidebar.set_margin_start(16);
    sidebar.set_margin_end(16);

    sidebar.append(&library_button);

    sidebar.append(&Separator::new(Orientation::Horizontal));

    sidebar.append(&import_button);

    // Spinge Settings in fondo alla sidebar
    let spacer = Box::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    sidebar.append(&spacer);

    sidebar.append(&Separator::new(Orientation::Horizontal));

    sidebar.append(&settings_button);

    sidebar
}
