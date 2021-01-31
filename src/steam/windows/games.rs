use std::io;
use std::path::PathBuf;

use crate::prelude::Game;
use crate::steam::acf;
use crate::steam::vdf;
use crate::steam::windows::utils::get_steam_executable;
use crate::util::io::*;
use crate::util::registry::*;

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let steam_regs = get_local_machine_reg_key("Valve\\Steam").and_then(|launcher_reg| {
        get_current_user_reg_key("Valve\\Steam").map(|installs_reg| (launcher_reg, installs_reg))
    });

    if steam_regs.is_err() {
        return Ok(games);
    }

    let (launcher_reg, user_launcher_reg) = steam_regs.unwrap();

    let steam_path_default = PathBuf::from(
        launcher_reg
            .get_value::<String, &str>("InstallPath")
            .unwrap(),
    )
    .join("steamapps");

    if !steam_path_default.exists() {
        return Ok(games);
    }

    let mut library_paths = Vec::new();
    library_paths.push(steam_path_default.clone());

    let library_folders =
        vdf::read_library_folders(&steam_path_default.join("libraryfolders.vdf")).unwrap();

    for folder in library_folders {
        library_paths.push(folder.join("steamapps"));
    }

    let steam_executable = get_steam_executable(
        &user_launcher_reg
            .get_value::<String, &str>("SteamExe")
            .unwrap(),
    );

    let mut manifests = Vec::new();
    let get_manifest_predicate = |item: &PathBuf| -> bool { item.extension().unwrap().eq("acf") };

    for path in library_paths {
        match get_files(&path, get_manifest_predicate) {
            Ok(list) => manifests.extend(list),
            Err(_e) => {}
        }
    }

    for manifest in manifests {
        match acf::read(&manifest, &steam_executable) {
            Ok(g) => games.push(g),
            Err(_e) => {}
        }
    }

    return Ok(games);
}
