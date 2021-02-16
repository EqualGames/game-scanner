use crate::error::{Error, ErrorKind, Result};
use crate::prelude::{Game, GameType};
use rusqlite::{Connection, OpenFlags, NO_PARAMS};
use std::path::Path;

pub fn read(file: &Path, launcher_path: &Path) -> Result<Vec<Game>> {
    let conn = Connection::open_with_flags(&file, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid Amazon Games manifest: {}", error.to_string()),
            )
        })
        .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, ProductTitle, InstallDirectory FROM DbSet WHERE Installed = 1;")
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error to read the Amazon Games manifest: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let games_iter = stmt
        .query_map(NO_PARAMS, |row| {
            let id = row.get::<usize, String>(0).unwrap();
            let name = row.get::<usize, String>(1).unwrap();
            let path = row.get::<usize, String>(2).unwrap();

            return Ok(make_game(id, name, path, &launcher_path));
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error to read the Amazon Games manifest: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let mut games = Vec::<Game>::new();

    for game in games_iter {
        games.push(game.unwrap());
    }

    return Ok(games);
}

fn make_game(id: String, name: String, path: String, launcher_path: &Path) -> Game {
    return Game {
        _type: GameType::AmazonGames.to_string(),
        id: id.clone(),
        name,
        path,
        launch_command: vec![
            launcher_path
                .join("App")
                .join("Amazon Games.exe")
                .display()
                .to_string(),
            format!("amazon-games://play/{}", id),
        ],
    };
}
