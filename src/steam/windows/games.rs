use std::io;
use std::path::PathBuf;

use crate::prelude::Game;
use crate::steam::acf;
use crate::steam::vdf;
use crate::steam::windows::utils::get_steam_executable;
use crate::util::io::*;
use crate::util::registry::*;

pub fn list() -> io::Result<Vec<Game>> {
    let mut items = Vec::new();

    let reg = get_local_machine_reg_key("Valve\\Steam")?;
    let steam_install_path: String = reg.get_value("InstallPath")?;
    let steam_path_default = PathBuf::from(steam_install_path).join("steamapps");

    let library_folders_file = steam_path_default.join("libraryfolders.vdf");
    let mut library_paths = Vec::new();
    library_paths.push(steam_path_default);

    for folder in vdf::read_library_folders(&library_folders_file)? {
        library_paths.push(folder.join("steamapps"));
    }

    let steam_reg = get_current_user_reg_key("Valve\\Steam")?;
    let steam_exe: String = steam_reg.get_value("SteamExe")?;
    let steam_executable = get_steam_executable(&steam_exe);

    let mut files = Vec::new();

    for folder in library_paths {
        match get_files(&folder, |item| item.extension().unwrap().eq("acf")) {
            Ok(list) => files.extend(list),
            Err(_e) => {}
        }
    }

    for file in files {
        let game = acf::read(&file, &steam_executable);

        match game {
            Ok(g) => items.push(g),
            Err(_e) => {}
        }
    }

    return Ok(items);
}
