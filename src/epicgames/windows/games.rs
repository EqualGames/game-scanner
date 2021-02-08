use crate::epicgames::item;
use crate::prelude::Game;
use crate::util::io::*;
use crate::util::registry::*;
use std::io;
use std::path::PathBuf;

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let launcher_info = get_local_machine_reg_key("Epic Games\\EpicGamesLauncher")
        .and_then(|launcher_reg| launcher_reg.get_value::<String, &str>("AppDataPath"))
        .map(PathBuf::from)
        .and_then(|app_data_path| {
            get_current_user_reg_key("Epic Games\\EOS")
                .and_then(|eos_reg| eos_reg.get_value::<String, &str>("ModSdkCommand"))
                .map(|path| path.replace("/", &std::path::MAIN_SEPARATOR.to_string()))
                .map(PathBuf::from)
                .map(|mod_sdk_command| (app_data_path, mod_sdk_command))
        });

    if launcher_info.is_err() {
        return Ok(games);
    }

    let (app_data_path, mod_sdk_command) = launcher_info.unwrap();
    let manifests_path = app_data_path.join("Manifests");

    if !manifests_path.exists() || !app_data_path.exists() || !mod_sdk_command.exists() {
        return Ok(games);
    }

    let manifests = get_files(&manifests_path, get_manifest_predicate).unwrap();

    for manifest in manifests {
        match item::read(&manifest, &mod_sdk_command) {
            Ok(g) => games.push(g),
            Err(_e) => {}
        }
    }

    return Ok(games);
}

fn get_manifest_predicate(file: &PathBuf) -> bool {
    file.extension().unwrap().eq("item")
}
