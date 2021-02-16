use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::util::registry::*;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let launcher_info = get_local_machine_reg_key("GOG.com\\GalaxyClient\\paths")
        .and_then(|launcher_paths_reg| {
            launcher_paths_reg
                .get_value::<String, &str>("client")
                .map_err(Error::from)
        })
        .map(PathBuf::from)
        .and_then(|launcher_path| {
            get_local_machine_reg_key("GOG.com\\GalaxyClient")
                .and_then(|launcher_reg| {
                    launcher_reg
                        .get_value::<String, &str>("clientExecutable")
                        .map_err(Error::from)
                })
                .map(|launcher_filename| launcher_path.join(launcher_filename))
        })
        .and_then(|launcher_executable| {
            get_local_machine_reg_key("GOG.com\\Games")
                .map(|launcher_games_reg| (launcher_executable, launcher_games_reg))
        });

    if launcher_info.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid GOG path, maybe this launcher is not installed",
        ));
    }

    let (launcher_executable, launcher_games) = launcher_info.unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid GOG path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    let launcher_games_ids = launcher_games.enum_keys().map(|key| key.unwrap());

    for game_id in launcher_games_ids {
        let (game_name, game_path) = launcher_games
            .open_subkey(&game_id)
            .and_then(|game_info_reg| {
                game_info_reg
                    .get_value::<String, &str>("gameName")
                    .and_then(|game_name| {
                        game_info_reg
                            .get_value::<String, &str>("path")
                            .map(|game_path| (game_name, game_path))
                    })
            })
            .map_err(|error| {
                Error::new(
                    ErrorKind::InvalidApp,
                    format!(
                        "Error on read the GOG manifest: {} {}",
                        game_id,
                        error.to_string()
                    ),
                )
            })
            .unwrap();

        let game = Game {
            _type: String::from("gog"),
            id: game_id.clone(),
            name: game_name,
            path: game_path.clone(),
            launch_command: vec![
                launcher_executable.display().to_string(),
                String::from("/command=runGame"),
                format!("/gameId={}", &game_id),
                format!("/path={}", &game_path),
            ],
        };

        games.push(game);
    }

    return Ok(games);
}
