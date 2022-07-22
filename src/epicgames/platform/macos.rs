use crate::error::{Error, ErrorKind, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable_path = PathBuf::from("/")
        .join("Applications")
        .join("Epic Games Launcher.app")
        .join("Contents")
        .join("MacOS")
        .join("EpicGamesLauncher-Mac-Shipping");

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

pub fn get_manifests_path() -> Result<PathBuf> {
    let manifests_path = ProjectDirs::from("", "", "Epic")
        .unwrap()
        .config_dir()
        .join("EpicGamesLauncher")
        .join("Data")
        .join("Manifests");

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
