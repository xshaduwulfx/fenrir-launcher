use std::fs;

use crate::database::sqlite;
use crate::repositories::game_repository::GameRepository;
use crate::utils::paths;

pub struct AppState {
    pub games: GameRepository,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir = paths::data_dir()?;
        fs::create_dir_all(&data_dir)?;

        let database_path = data_dir.join("fenrir.db");
        let connection = sqlite::open(database_path)?;
        let games = GameRepository::new(connection);

        Ok(Self { games })
    }
}
