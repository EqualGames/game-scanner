use std::io;
use std::path::PathBuf;

use crate::prelude::Game;
use crate::util::registry::*;

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let ubisoft_regs = get_local_machine_reg_key("Ubisoft\\Launcher").and_then(|launcher_reg| {
        launcher_reg
            .open_subkey("Installs")
            .map(|installs_reg| (launcher_reg, installs_reg))
    });

    if ubisoft_regs.is_err() {
        return Ok(games);
    }

    let (launcher_reg, installs_reg) = ubisoft_regs.unwrap();

    let ubisoft_executable = PathBuf::from(
        launcher_reg
            .get_value::<String, &str>("InstallDir")
            .unwrap(),
    )
    .join("upc.exe");

    if !ubisoft_executable.exists() {
        return Ok(games);
    }

    let game_identifiers = installs_reg
        .enum_keys()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    for identifier in game_identifiers {
        let game_reg = get_local_machine_reg_key(&get_game_reg_path(&identifier)).unwrap();

        let mut command = ubisoft_executable.to_str().unwrap().to_string();
        command.push_str(" uplay://launch/");
        command.push_str(&identifier);

        let install_location = game_reg
            .get_value::<String, &str>("InstallLocation")
            .map(|value| value.replace("/", "\\"))
            .unwrap();

        let game = Game {
            _type: "ubisoft".to_string(),
            id: String::from(&identifier),
            name: game_reg.get_value("DisplayName")?,
            path: install_location,
            launch_command: command,
        };

        games.push(game)
    }

    return Ok(games);
}

fn get_game_reg_path(id: &String) -> String {
    let mut path = String::from("Microsoft\\Windows\\CurrentVersion\\Uninstall\\Uplay Install ");
    path.push_str(id);

    return path;
}
