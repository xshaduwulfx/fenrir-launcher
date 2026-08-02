use adw::prelude::*;
use gtk::{Align, Box, Label, ListBox, ListBoxRow, Orientation};

use crate::models::game::Game;

pub fn build(games: &[Game]) -> Box {
    let content = Box::new(Orientation::Vertical, 12);
    content.set_margin_top(16);
    content.set_margin_bottom(16);
    content.set_margin_start(16);
    content.set_margin_end(16);
    content.set_vexpand(true);

    if games.is_empty() {
        content.set_halign(Align::Center);
        content.set_valign(Align::Center);

        let subtitle = Label::builder()
        .label("No games in your library yet.")
        .css_classes(["title-2"])
        .build();

        let hint = Label::builder()
        .label("Press + to import your first game.")
        .css_classes(["dim-label"])
        .build();

        content.append(&subtitle);
        content.append(&hint);
    } else {
        let game_list = ListBox::new();
        game_list.set_selection_mode(gtk::SelectionMode::Single);
        game_list.add_css_class("boxed-list");

        for game in games {
            let title = Label::builder()
            .label(&game.title)
            .halign(Align::Start)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(16)
            .margin_end(16)
            .build();

            let row = ListBoxRow::new();
            row.set_child(Some(&title));

            game_list.append(&row);
        }

        content.append(&game_list);
    }

    content
}
