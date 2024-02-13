use std::path::PathBuf;

use self::platform::{get_launcher_executable, get_launcher_path, get_manifests_path};
use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    utils::io::get_files_recursive,
};

#[cfg_attr(target_os = "windows", path = "platform/windows.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
mod platform;
mod types;
mod yaml;

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
    let launcher_path = get_launcher_path()?;
    let manifests_path = get_manifests_path(&launcher_path)?;
    let manifests = get_files_recursive(&manifests_path, |file| {
        file.display()
            .to_string()
            .ends_with(".product_settings.yaml")
    })
    .map_err(|error| {
        Error::new(
            ErrorKind::LauncherNotFound,
            format!("Invalid Riot Games path, maybe this launcher is not installed: {error}"),
        )
    })?;

    let mut games = Vec::new();

    for manifest in manifests {
        match yaml::read(&manifest, &launcher_path) {
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
///
/// # Panics
///
/// Will panic if the path terminates in `..`
pub fn find(id: &str) -> Result<Game> {
    let launcher_path = get_launcher_path()?;
    let manifests_path = get_manifests_path(&launcher_path)?;
    let manifests = get_files_recursive(&manifests_path, |file| {
        file.display()
            .to_string()
            .ends_with(".product_settings.yaml")
            && file.file_name().unwrap().to_str().unwrap().starts_with(id)
    })
    .map_err(|error| {
        Error::new(
            ErrorKind::LauncherNotFound,
            format!("Invalid Riot Games path, maybe this launcher is not installed: {error}"),
        )
    })?;

    let manifest = manifests.first().ok_or_else(|| {
        Error::new(
            ErrorKind::GameNotFound,
            format!("Riot Games game with id ({id}) does not exist"),
        )
    })?;

    yaml::read(manifest, &launcher_path)
}
