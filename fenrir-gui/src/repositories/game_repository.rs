use std::rc::Rc;

use rusqlite::{params, Connection, Result};

use crate::models::game::Game;

pub struct GameRepository {
    connection: Rc<Connection>,
}

impl GameRepository {
    pub fn new(connection: Rc<Connection>) -> Self {
        Self { connection }
    }

    pub fn add(&self, game: &Game) -> Result<()> {
        self.connection.execute(
            "
            INSERT INTO games (id, title, executable)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(id) DO UPDATE SET
        title = excluded.title,
        executable = excluded.executable
        ",
        params![game.id, game.title, game.executable],
        )?;

        Ok(())
    }

    pub fn all(&self) -> Result<Vec<Game>> {
        let mut statement = self.connection.prepare(
            "
            SELECT id, title, executable
            FROM games
            ORDER BY title COLLATE NOCASE
            ",
        )?;

        let games = statement
        .query_map([], |row| {
            Ok(Game::new(
                row.get::<_, String>(0)?,
                         row.get::<_, String>(1)?,
                         row.get::<_, String>(2)?,
            ))
        })?
        .collect::<Result<Vec<_>>>()?;

        Ok(games)
    }
}
