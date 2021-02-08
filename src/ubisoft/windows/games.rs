use crate::prelude::Game;
use crate::util::registry::*;
use std::io;
use std::path::{Path, PathBuf};

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let launcher_info = get_local_machine_reg_key("Ubisoft\\Launcher").and_then(|launcher_reg| {
        launcher_reg
            .get_value::<String, &str>("InstallDir")
            .map(PathBuf::from)
            .map(|path| path.join("upc.exe"))
            .and_then(|launcher_executable| {
                launcher_reg
                    .open_subkey("Installs")
                    .map(|launcher_games| (launcher_executable, launcher_games))
            })
    });

    if launcher_info.is_err() {
        return Ok(games);
    }

    let (launcher_executable, launcher_games) = launcher_info.unwrap();

    if !launcher_executable.exists() {
        return Ok(games);
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
                    .and_then(|game_name| {
                        game_reg
                            .get_value::<String, &str>("InstallLocation")
                            .map(|value| value.replace("/", &std::path::MAIN_SEPARATOR.to_string()))
                            .map(|game_path| (game_name, game_path))
                    })
            })
            .unwrap();

        let game = Game {
            _type: String::from("ubisoft"),
            id: String::from(&id),
            name: game_name,
            path: game_path,
            launch_command: make_launch_command(&launcher_executable, &id),
        };

        games.push(game)
    }

    return Ok(games);
}

fn get_game_reg_path(id: &String) -> String {
    return String::from("Microsoft\\Windows\\CurrentVersion\\Uninstall\\Uplay Install ") + id;
}

fn make_launch_command(launcher_executable: &Path, id: &String) -> Vec<String> {
    let mut command = Vec::new();

    command.push(launcher_executable.display().to_string());
    command.push(format!("uplay://launch/{}", id));

    return command;
}
