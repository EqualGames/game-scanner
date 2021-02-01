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

    let launcher_info = get_local_machine_reg_key("Valve\\Steam").and_then(|launcher_reg| {
        get_current_user_reg_key("Valve\\Steam")
            .and_then(|user_launcher_reg| user_launcher_reg.get_value::<String, &str>("SteamExe"))
            .map(|path| get_steam_executable(&path))
            .map(PathBuf::from)
            .map(|launcher_executable| (launcher_executable, launcher_reg))
    });

    if launcher_info.is_err() {
        return Ok(games);
    }

    let (launcher_executable, launcher_reg) = launcher_info.unwrap();

    let steam_path_default = PathBuf::from(
        launcher_reg
            .get_value::<String, &str>("InstallPath")
            .unwrap(),
    )
    .join("steamapps");

    if !launcher_executable.exists() || !steam_path_default.exists() {
        return Ok(games);
    }

    let mut library_paths = Vec::new();
    library_paths.push(steam_path_default.clone());

    let library_folders =
        vdf::read_library_folders(&steam_path_default.join("libraryfolders.vdf")).unwrap();

    for folder in library_folders {
        library_paths.push(folder.join("steamapps"));
    }

    let mut library_manifests = Vec::<(PathBuf, Vec<PathBuf>)>::new();

    for path in library_paths {
        match get_files(&path, get_manifest_predicate) {
            Ok(list) => library_manifests.push((path, list)),
            Err(_e) => {}
        }
    }

    for library_manifest in library_manifests {
        let (library_path, manifests) = library_manifest;

        for manifest in manifests {
            match acf::read(&manifest, &launcher_executable, &library_path) {
                Ok(g) => games.push(g),
                Err(_e) => {}
            }
        }
    }

    return Ok(games);
}

fn get_manifest_predicate(file: &PathBuf) -> bool {
    return file.extension().unwrap().eq("acf");
}
