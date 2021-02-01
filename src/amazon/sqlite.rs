use std::path::Path;

use rusqlite::{params, Connection, OpenFlags};

use crate::prelude::Game;
use crate::util::error::make_io_error;

pub fn read(file: &Path, launcher_path: &Path) -> std::io::Result<Vec<Game>> {
    let conn = Connection::open_with_flags(&file, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|_error| make_io_error("invalid amazon launcher database"))
        .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, ProductTitle, InstallDirectory FROM DbSet WHERE Installed = 1;")
        .map_err(|_error| make_io_error("error to read the amazon launcher database"))
        .unwrap();

    let game_itr = stmt
        .query_map(params![], |row| {
            let id: String = row.get(0).unwrap();

            return Ok(Game {
                _type: String::from("amazon"),
                id: id.clone(),
                name: row.get(1).unwrap(),
                path: row.get(2).unwrap(),
                launch_command: make_launch_command(&launcher_path, &id).unwrap(),
            });
        })
        .map_err(|_error| make_io_error("error to read the games from the amazon launcher"))
        .unwrap();

    let mut games = Vec::new();

    for game in game_itr {
        games.push(game.unwrap());
    }

    return Ok(games);
}

fn make_launch_command(launcher_path: &Path, id: &String) -> Option<String> {
    return launcher_path
        .clone()
        .join("App")
        .join("Amazon Games.exe")
        .to_str()
        .map(|command| format!("{} amazon-games://play/{}", command, id));
}
