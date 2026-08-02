use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, ToolbarView};
use gtk::{FileDialog, FileFilter};

use crate::app_state::AppState;
use crate::models::game::Game;
use crate::{pages, widgets};

pub fn build(app: &Application, state: Rc<RefCell<AppState>>) {
    let games = {
        let state = state.borrow();

        state
        .games
        .all()
        .expect("Failed to load games from database")
    };

    let library = pages::library::build(&games);
    let action_bar = widgets::action_bar::build();

    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.set_vexpand(true);

    library.set_vexpand(true);

    content.append(&library);
    content.append(&action_bar.container);

    let header = HeaderBar::new();

    let toolbar = ToolbarView::new();
    toolbar.add_top_bar(&header);
    toolbar.set_content(Some(&content));

    let window = ApplicationWindow::builder()
    .application(app)
    .title("Fenrir Launcher")
    .default_width(1200)
    .default_height(800)
    .content(&toolbar)
    .build();

    let window_for_dialog = window.clone();
    let state_for_import = Rc::clone(&state);

    action_bar.import_button.connect_clicked(move |_| {
        let filter = FileFilter::new();
        filter.set_name(Some("Windows executables"));
        filter.add_pattern("*.exe");

        let filters = gtk::gio::ListStore::new::<FileFilter>();
        filters.append(&filter);

        let dialog = FileDialog::builder()
        .title("Import Game")
        .filters(&filters)
        .default_filter(&filter)
        .build();

        let state_for_result = Rc::clone(&state_for_import);

        dialog.open(
            Some(&window_for_dialog),
                    None::<&gtk::gio::Cancellable>,
                    move |result| match result {
                        Ok(file) => {
                            let Some(path) = file.path() else {
                                eprintln!("Selected file has no local path");
                                return;
                            };

                            let Some(file_stem) = path.file_stem() else {
                                eprintln!("Unable to determine the game title");
                                return;
                            };

                            let title = file_stem.to_string_lossy().into_owned();
                            let executable = path.to_string_lossy().into_owned();

                            let game = Game::new(
                                title.to_lowercase(),
                                                 title,
                                                 executable,
                            );

                            if let Err(error) = state_for_result.borrow().games.add(&game) {
                                eprintln!("Failed to save game: {error}");
                                return;
                            }

                            println!("Imported game: {}", game.title);
                        }
                        Err(error) => {
                            eprintln!("File selection closed or failed: {error}");
                        }
                    },
        );
    });

    window.present();
}
