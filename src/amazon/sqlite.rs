use crate::prelude::Game;
use crate::util::error::make_io_error;
use rusqlite::{params, Connection, OpenFlags};
use std::io;
use std::path::Path;

pub fn read(file: &Path, launcher_path: &Path) -> io::Result<Vec<Game>> {
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
                launch_command: make_launch_command(&launcher_path, &id),
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

fn make_launch_command(launcher_path: &Path, id: &String) -> Vec<String> {
    let mut command = Vec::new();

    command.push(
        launcher_path
            .join("App")
            .join("Amazon Games.exe")
            .display()
            .to_string(),
    );
    command.push(format!("amazon-games://play/{}", id));

    return command;
}
