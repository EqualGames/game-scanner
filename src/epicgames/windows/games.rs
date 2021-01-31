use std::io;
use std::path::PathBuf;

use crate::epicgames::item;
use crate::prelude::Game;
use crate::util::io::*;
use crate::util::registry::*;

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let epicgames_launcher_reg =
        get_local_machine_reg_key("Epic Games\\EpicGamesLauncher").unwrap();
    let epicgames_path = PathBuf::from(
        epicgames_launcher_reg
            .get_value::<String, &str>("AppDataPath")
            .unwrap(),
    );
    let epicgames_manifests_path = epicgames_path.clone().join("Manifests");

    let epicgames_eos_reg = get_current_user_reg_key("Epic Games\\EOS").unwrap();
    let epicgames_executable = PathBuf::from(
        epicgames_eos_reg
            .get_value::<String, &str>("ModSdkCommand")
            .map(|v| v.replace("/", &std::path::MAIN_SEPARATOR.to_string()))
            .unwrap(),
    );

    let files = get_files(&epicgames_manifests_path, |item| {
        item.extension().unwrap().eq("item")
    })
    .unwrap();

    for file in files {
        let game = item::read(&file, &epicgames_executable);

        match game {
            Ok(g) => games.push(g),
            Err(_e) => {}
        }
    }

    return Ok(games);
}
