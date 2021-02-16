use crate::epicgames::item;
use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::util::io::*;
use crate::util::path::fix_path_separator;
use crate::util::registry::*;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let launcher_data_path = get_local_machine_reg_key("Epic Games\\EpicGamesLauncher")
        .and_then(|launcher_reg| {
            launcher_reg
                .get_value::<String, &str>("AppDataPath")
                .map_err(Error::from)
        })
        .map(PathBuf::from)
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Epic Games path, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    if !launcher_data_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Epic Games path, maybe this launcher is not installed: {}",
                launcher_data_path.display().to_string()
            ),
        ));
    }

    let launcher_executable = get_current_user_reg_key("Epic Games\\EOS")
        .and_then(|eos_reg| {
            eos_reg
                .get_value::<String, &str>("ModSdkCommand")
                .map_err(Error::from)
        })
        .map(|path| fix_path_separator(path.as_ref()))
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Epic Games path, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Epic Games path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    let manifests_path = launcher_data_path.join("Manifests");

    let manifests = get_files(&manifests_path, get_manifest_predicate)
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Epic Games path, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    for manifest in manifests {
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
