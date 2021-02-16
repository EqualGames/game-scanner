use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::util::path::fix_path_separator;
use crate::util::registry::*;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let (launcher_executable, launcher_games) = get_local_machine_reg_key("Ubisoft\\Launcher")
        .and_then(|launcher_reg| {
            launcher_reg
                .get_value::<String, &str>("InstallDir")
                .map_err(Error::from)
                .map(PathBuf::from)
                .map(|path| path.join("upc.exe"))
                .and_then(|launcher_executable| {
                    launcher_reg
                        .open_subkey("Installs")
                        .map_err(Error::from)
                        .map(|launcher_games| (launcher_executable, launcher_games))
                })
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Ubisoft path, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Ubisoft path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    let game_ids = launcher_games
        .enum_keys()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    for id in game_ids {
        let (game_name, game_path) = get_local_machine_reg_key(&get_game_reg_path(&id))
            .and_then(|game_reg| {
                game_reg
                    .get_value::<String, &str>("DisplayName")
                    .map_err(Error::from)
                    .and_then(|game_name| {
                        game_reg
                            .get_value::<String, &str>("InstallLocation")
                            .map_err(Error::from)
                            .map(|value| fix_path_separator(value.as_ref()).display().to_string())
                            .map(|game_path| (game_name, game_path))
                    })
            })
            .map_err(|error| {
                Error::new(
                    ErrorKind::InvalidApp,
                    format!("Error on read the Ubisoft manifest: {}", error.to_string()),
                )
            })
            .unwrap();

        let game = Game {
            _type: String::from("ubisoft"),
            id: id.clone(),
            name: game_name,
            path: game_path,
            launch_command: vec![
                launcher_executable.display().to_string(),
                format!("uplay://launch/{}/0", &id),
            ],
        };

        games.push(game)
    }

    return Ok(games);
}

fn get_game_reg_path(id: &String) -> String {
    return format!(
        "Microsoft\\Windows\\CurrentVersion\\Uninstall\\Uplay Install {}",
        id
    );
}
