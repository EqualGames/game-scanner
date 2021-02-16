use crate::error::{Error, ErrorKind, Result};
use crate::prelude::{Game, GameType};
use crate::util::registry;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let launcher_info = registry::get_local_machine_reg_key("GOG.com\\GalaxyClient\\paths")
        .and_then(|launcher_paths_reg| registry::get_value(&launcher_paths_reg, "client"))
        .map(PathBuf::from)
        .and_then(|launcher_path| {
            registry::get_local_machine_reg_key("GOG.com\\GalaxyClient")
                .and_then(|launcher_reg| registry::get_value(&launcher_reg, "clientExecutable"))
                .map(|launcher_filename| launcher_path.join(launcher_filename))
        })
        .and_then(|launcher_executable| {
            registry::get_local_machine_reg_key("GOG.com\\Games")
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

    let mut games = Vec::<Game>::new();

    for game_id in launcher_games_ids {
        let game_info = registry::get_sub_key(&launcher_games, &game_id)
            .and_then(|game_info_reg| {
                registry::get_value(&game_info_reg, "gameName")
                    .map(|game_name| (game_info_reg, game_name))
            })
            .and_then(|(game_info_reg, game_name)| {
                registry::get_value(&game_info_reg, "path").map(|game_path| (game_name, game_path))
            });

        if game_info.is_err() {
            return Err(Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error on read the GOG manifest: {} {}",
                    game_id,
                    game_info.err().unwrap().to_string()
                ),
            ));
        }

        let (game_name, game_path) = game_info.unwrap();

        games.push(Game {
            _type: GameType::GOG.to_string(),
            id: game_id.clone(),
            name: game_name,
            path: game_path.clone(),
            launch_command: vec![
                launcher_executable.display().to_string(),
                String::from("/command=runGame"),
                format!("/gameId={}", &game_id),
                format!("/path={}", &game_path),
            ],
        });
    }

    return Ok(games);
}
