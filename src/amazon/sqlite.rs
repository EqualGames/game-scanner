use std::path::Path;

use rusqlite::{Connection, OpenFlags};

use crate::error::{Error, ErrorKind, Result};
use crate::prelude::{Game, GameType};

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
        .prepare("SELECT * FROM DbSet")
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

    let rows = stmt
        .query_map([], |row| {
            let columns = row.column_count();

            let mut game = Game::default();
            game._type = GameType::AmazonGames.to_string();

            for col in 0..columns {
                let name = row.column_name(col).unwrap();

                match name {
                    "Id" => game.id = row.get(col).unwrap(),
                    "ProductTitle" => game.name = row.get(col).unwrap(),
                    "InstallDirectory" => game.path = row.get(col).unwrap(),
                    "Installed" => game.state.installed = row.get(col).unwrap(),
                    _ => {}
                }
            }

            game.commands.launch = vec![
                launcher_path
                    .join("App")
                    .join("Amazon Games.exe")
                    .display()
                    .to_string(),
                format!("amazon-games://play/{}", game.id),
            ];

            Ok(game)
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error to parse the Amazon Games manifest: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let mut games = Vec::<Game>::new();

    for game in rows {
        games.push(game.unwrap());
    }

    return Ok(games);
}
