use std::path::{Path, PathBuf};

use rusqlite::{Connection, OpenFlags, Row};

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::{Game, GameType},
};

pub fn read_all(file: &Path, launcher_path: &Path) -> Result<Vec<Game>> {
    let conn =
        Connection::open_with_flags(&file, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid Amazon Games manifest: {}", error.to_string()),
            )
        })?;

    let mut stmt = conn.prepare("SELECT * FROM DbSet").map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!(
                "Error to read the Amazon Games manifest: {}",
                error.to_string()
            ),
        )
    })?;

    let rows = stmt
        .query_map([], |row| parse_row(row, launcher_path))
        .map_err(|error| match error {
            rusqlite::Error::QueryReturnedNoRows => Error::new(
                ErrorKind::LibraryNotFound,
                format!("Amazon library could be empty"),
            ),
            _ => Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error to read the Amazon Games manifest: {}",
                    error.to_string()
                ),
            ),
        })?;

    let mut games = Vec::<Game>::new();

    for game in rows {
        games.push(game?);
    }

    return Ok(games);
}

pub fn read(id: &str, file: &Path, launcher_path: &Path) -> Result<Game> {
    let conn =
        Connection::open_with_flags(&file, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid Amazon Games manifest: {}", error.to_string()),
            )
        })?;

    let mut stmt = conn
        .prepare("SELECT * FROM DbSet WHERE Id = :id")
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error to read the Amazon Games manifest: {}",
                    error.to_string()
                ),
            )
        })?;

    return stmt
        .query_row(&[(":id", id)], |row| parse_row(row, launcher_path))
        .map_err(|error| match error {
            rusqlite::Error::QueryReturnedNoRows => Error::new(
                ErrorKind::GameNotFound,
                format!("Amazon game with id ({}) does not exist", id),
            ),
            _ => Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error to read the Amazon Games manifest: {}",
                    error.to_string()
                ),
            ),
        });
}

fn parse_row(row: &Row, launcher_path: &Path) -> rusqlite::Result<Game> {
    let columns = row.column_count();

    let mut game = Game::default();
    game._type = GameType::AmazonGames.to_string();

    for col in 0..columns {
        let name = row.column_name(col)?;

        match name {
            "Id" => game.id = row.get(col)?,
            "ProductTitle" => game.name = row.get(col)?,
            "InstallDirectory" => game.path = row.get::<_, String>(col).map(PathBuf::from).ok(),
            "Installed" => game.state.installed = row.get(col)?,
            _ => {}
        }
    }

    let launcher_executable = launcher_path.join("App").join("Amazon Games.exe");

    game.commands.launch = Some(vec![
        launcher_executable.display().to_string(),
        format!("amazon-games://play/{}", game.id),
    ]);

    Ok(game)
}
