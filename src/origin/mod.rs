use self::platform::{get_launcher_executable, get_manifests_path};
use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    utils::io::get_files_recursive,
};
use std::{ops::Add, path::PathBuf};

mod mfst;
#[cfg_attr(target_os = "windows", path = "platform/windows.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
mod platform;

pub fn executable() -> Result<PathBuf> {
    return get_launcher_executable();
}

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable()?;
    let manifests_path = get_manifests_path()?;
    let manifests = get_files_recursive(&manifests_path, |file| {
        file.display().to_string().ends_with(".mfst")
    })
    .map_err(|error| {
        Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Origin path, maybe this launcher is not installed: {}",
                error.to_string()
            ),
        )
    })?;

    let mut games = Vec::new();

    for manifest in manifests {
        match mfst::read(&manifest, &launcher_executable) {
            Ok(g) => games.push(g),
            Err(error) => {
                // if error.kind().ne(&ErrorKind::IgnoredApp) {
                return Err(error);
                // }
            }
        }
    }

    return Ok(games);
}

pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable()?;
    let manifests_path = get_manifests_path()?;

    let manifests = get_files_recursive(&manifests_path, |file| {
        file.display()
            .to_string()
            .contains(&id.replace("%3a", "").add(".mfst"))
    })
    .map_err(|error| {
        Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Origin path, maybe this launcher is not installed: {}",
                error.to_string()
            ),
        )
    })?;

    let manifest = manifests.get(0).ok_or(Error::new(
        ErrorKind::GameNotFound,
        format!("Origin game with id ({}) does not exist", id),
    ))?;

    return mfst::read(&manifest, &launcher_executable);
}
