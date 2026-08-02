use std::path::Path;
use std::rc::Rc;

use rusqlite::{Connection, Result};

pub fn open(path: impl AsRef<Path>) -> Result<Rc<Connection>> {
    let connection = Rc::new(Connection::open(path)?);

    initialize(&connection)?;

    Ok(connection)
}

fn initialize(connection: &Connection) -> Result<()> {
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS games (
            id          TEXT PRIMARY KEY NOT NULL,
            title       TEXT NOT NULL,
            executable  TEXT NOT NULL
    )
    ",
    [],
    )?;

    Ok(())
}
