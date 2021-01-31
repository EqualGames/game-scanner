use std::io;
use std::path::PathBuf;

use crate::prelude::Game;
use crate::util::registry::*;

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let reg = get_local_machine_reg_key("GOG.com\\GalaxyClient\\paths").unwrap();
    let gog_reg = get_local_machine_reg_key("GOG.com\\GalaxyClient").unwrap();

    let gog_path = PathBuf::from(reg.get_value::<String, &str>("client").unwrap());
    let gog_executable = gog_path
        .clone()
        .join(std::path::MAIN_SEPARATOR.to_string())
        .join(
            gog_reg
                .get_value::<String, &str>("clientExecutable")
                .unwrap(),
        );

    let gog_games_reg = get_local_machine_reg_key("GOG.com\\Games")?;

    for identifier in gog_games_reg.enum_keys().map(|key| key.unwrap()) {
        let game_reg = gog_games_reg.open_subkey(&identifier)?;

        let game_path = game_reg.get_value::<String, &str>("path")?;
        let mut launch_command = gog_executable.clone().to_str().unwrap().to_string();
        launch_command.push_str(" /command=runGame /gameId=");
        launch_command.push_str(&identifier);
        launch_command.push_str(" /path=");
        launch_command.push_str(&game_path);

        let game = Game {
            _type: "gog".to_string(),
            id: identifier,
            name: game_reg.get_value("gameName")?,
            path: game_path,
            launch_command,
        };

        games.push(game);
    }

    return Ok(games);
}
