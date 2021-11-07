use std::path::PathBuf;

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    utils::io::get_files,
};

use self::utils::{get_launcher_executable, get_manifests_path};

mod item;
#[cfg_attr(target_os = "windows", path = "utils/windows.rs")]
#[cfg_attr(target_os = "macos", path = "utils/macos.rs")]
mod utils;

pub fn executable() -> Result<PathBuf> {
    return get_launcher_executable();
}

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable()?;

    let manifests_path = get_manifests_path()?;

    let manifests = get_files(&manifests_path, |file| {
        file.display().to_string().ends_with(".item")
    })
    .map_err(|error| {
        Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Epic Games path, maybe this launcher is not installed: {}",
                error.to_string()
            ),
        )
    })?;

    let mut games = Vec::<Game>::new();

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

pub fn find(id: &str) -> Result<Game> {
    let manifests = games()?;
    let game = manifests
        .iter()
        .find(|item| item.id == id)
        .ok_or(Error::new(
            ErrorKind::GameNotFound,
            format!("Epic Games game with id ({}) does not exist", id),
        ))?;

    Ok(game.clone())
}
