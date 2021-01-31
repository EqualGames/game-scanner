use std::io;
use std::path::PathBuf;

use directories::BaseDirs;
use rusqlite::{params, Connection, OpenFlags};

use crate::prelude::Game;

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let amazon_path = PathBuf::from(BaseDirs::new().unwrap().data_local_dir()).join("Amazon Games");
    let db_path = amazon_path
        .clone()
        .join("Data")
        .join("Games")
        .join("Sql")
        .join("GameInstallInfo.sqlite");

    if !db_path.exists() {
        return Ok(games);
    }

    let conn = Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(
        |_error| io::Error::new(io::ErrorKind::Other, "invalid amazon launcher database"),
    )?;

    let mut stmt = conn
        .prepare("SELECT id, ProductTitle, InstallDirectory FROM DbSet WHERE Installed = 1;")
        .map_err(|_error| {
            io::Error::new(
                io::ErrorKind::Other,
                "error to read the amazon launcher database",
            )
        })?;

    let game_itr = stmt
        .query_map(params![], |row| {
            let id: String = row.get(0)?;
            let mut launch_command: String = amazon_path
                .clone()
                .join("App")
                .join("Amazon Games.exe")
                .to_str()
                .unwrap()
                .to_string();
            launch_command.push_str(" amazon-games://play/");
            launch_command.push_str(&id);

            return Ok(Game {
                _type: "amazon".to_string(),
                id,
                name: row.get(1)?,
                path: row.get(2)?,
                launch_command,
            });
        })
        .map_err(|_error| {
            io::Error::new(
                io::ErrorKind::Other,
                "error to read the games from the amazon launcher",
            )
        })
        .unwrap();

    for game in game_itr {
        games.push(game.unwrap());
    }

    return Ok(games);
}
