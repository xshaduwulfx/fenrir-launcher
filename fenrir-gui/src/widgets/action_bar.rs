use adw::prelude::*;
use gtk::{Align, Box, Button, Entry, Orientation};

pub struct ActionBar {
    pub container: Box,
    pub import_button: Button,
}

pub fn build() -> ActionBar {
    let import_button = Button::builder()
    .icon_name("list-add-symbolic")
    .tooltip_text("Import Game")
    .build();

    let settings_button = Button::builder()
    .icon_name("emblem-system-symbolic")
    .tooltip_text("Settings")
    .build();

    let search_entry = Entry::builder()
    .placeholder_text("Search games")
    .hexpand(true)
    .build();

    let kill_button = Button::builder()
    .icon_name("process-stop-symbolic")
    .tooltip_text("Kill running game")
    .sensitive(false)
    .build();

    let play_button = Button::builder()
    .icon_name("media-playback-start-symbolic")
    .tooltip_text("Play selected game")
    .sensitive(false)
    .build();

    let container = Box::new(Orientation::Horizontal, 8);
    container.set_margin_top(8);
    container.set_margin_bottom(8);
    container.set_margin_start(8);
    container.set_margin_end(8);
    container.set_valign(Align::Center);

    container.append(&import_button);
    container.append(&settings_button);
    container.append(&search_entry);
    container.append(&kill_button);
    container.append(&play_button);

    ActionBar {
        container,
        import_button,
    }
}
