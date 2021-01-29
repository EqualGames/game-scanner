use crate::types::Game;
use crate::epicgames::item;
use crate::util::registry::*;
use crate::util::io::*;
use std::path::Path;

pub fn list() -> std::io::Result<Vec<Game>> {
    let mut items: Vec<Game> = Vec::new();

    let epicgames_launcher_reg = get_local_machine_reg_key("Epic Games\\EpicGamesLauncher")?;
    let epicgames_app_data_path: String = epicgames_launcher_reg.get_value("AppDataPath")?;
    let epicgames_path = Path::new(&epicgames_app_data_path);
    let epicgames_manifests_path = epicgames_path.clone().join("Manifests");

    let epicgames_eos_reg = get_current_user_reg_key("Epic Games\\EOS")?;
    let epicgames_mod_sdk_command: String = epicgames_eos_reg.get_value("ModSdkCommand")?;
    let epicgames_executable = Path::new(&epicgames_mod_sdk_command);

    let files = get_files(&epicgames_manifests_path, |item| item.extension().unwrap().eq("item"))?;

    for file in files {
        let game = item::read(&file, epicgames_executable);

        match game {
            Ok(g) => items.push(g),
            Err(_e) => {}
        }
    }

    return Ok(items);
}
