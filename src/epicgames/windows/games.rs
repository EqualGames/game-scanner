use crate::epicgames::item;
use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::util::io::*;
use crate::util::path::fix_path_separator;
use crate::util::registry;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable();

    if launcher_executable.is_err() {
        return Err(launcher_executable.err().unwrap());
    }

    let launcher_executable = launcher_executable.unwrap();

    let manifests_path = get_manifests_path();

    if manifests_path.is_err() {
        return Err(manifests_path.err().unwrap());
    }

    let manifests_path = manifests_path.unwrap();

    let manifests = get_files(&manifests_path, get_manifest_predicate);

    if manifests.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Epic Games path, maybe this launcher is not installed: {}",
                manifests.err().unwrap().to_string()
            ),
        ));
    }

    let mut games = Vec::<Game>::new();

    for manifest in manifests.unwrap() {
        match item::read(&manifest, &launcher_executable) {
            Ok(g) => games.push(g),
            Err(error) => {
                if error.kind().ne(&ErrorKind::IgnoredApp) {
                    return Err(error);
                }
            }
        }
    }

    return Ok(games);
}

fn get_manifest_predicate(file: &PathBuf) -> bool {
    file.extension().unwrap().eq("item")
}

fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_current_user_reg_key("Epic Games\\EOS")
        .and_then(|eos_reg| registry::get_value(&eos_reg, "ModSdkCommand"))
        .map(|path| fix_path_separator(path.as_ref()));

    if launcher_executable.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Epic Games path, maybe this launcher is not installed: {}",
        ));
    }

    let launcher_executable_path = launcher_executable.unwrap();

    if !launcher_executable_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Epic Games path, maybe this launcher is not installed: {}",
                launcher_executable_path.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable_path);
}

fn get_manifests_path() -> Result<PathBuf> {
    let launcher_data = registry::get_local_machine_reg_key("Epic Games\\EpicGamesLauncher")
        .and_then(|launcher_reg| registry::get_value(&launcher_reg, "AppDataPath"))
        .map(PathBuf::from);

    if launcher_data.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Epic Games path, maybe this launcher is not installed",
        ));
    }

    let manifests_path = launcher_data.unwrap().join("Manifests");

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Epic Games path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}
