use adw::prelude::*;
use gtk::{Align, Box, Button, Label, Orientation};

pub fn build() -> Box {
    let title = Label::builder()
    .label("Welcome to Fenrir Launcher")
    .css_classes(["title-1"])
    .build();

    let subtitle = Label::builder()
    .label("Your game library is currently empty.")
    .css_classes(["dim-label"])
    .build();

    let add_game_button = Button::builder()
    .label("Add your first game")
    .halign(Align::Center)
    .build();

    let content = Box::new(Orientation::Vertical, 12);
    content.set_halign(Align::Center);
    content.set_valign(Align::Center);
    content.append(&title);
    content.append(&subtitle);
    content.append(&add_game_button);

    content
}
