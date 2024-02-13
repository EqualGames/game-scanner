use std::path::PathBuf;

use self::utils::{get_launcher_executable, get_manifests_path};
use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    utils::io::get_files,
};

mod item;
#[cfg_attr(target_os = "windows", path = "platform/windows.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
mod utils;

/// # Errors
///
/// Will return `Err` if the executable is not found
pub fn executable() -> Result<PathBuf> {
    get_launcher_executable()
}

/// # Errors
///
/// Will return `Err` if games are not found
pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable()?;

    let manifests_path = get_manifests_path()?;

    let manifests = get_files(&manifests_path, |file| {
        std::path::Path::new(&file.display().to_string())
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("item"))
    })
    .map_err(|error| {
        Error::new(
            ErrorKind::LauncherNotFound,
            format!("Invalid Epic Games path, maybe this launcher is not installed: {error}"),
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

    Ok(games)
}

/// # Errors
///
/// Will return `Err` if the id is not found
pub fn find(id: &str) -> Result<Game> {
    let manifests = games()?;
    let game = manifests.iter().find(|item| item.id == id).ok_or_else(|| {
        Error::new(
            ErrorKind::GameNotFound,
            format!("Epic Games game with id ({id}) does not exist"),
        )
    })?;

    Ok(game.clone())
}
