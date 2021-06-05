use crate::error::{Error, ErrorKind, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = PathBuf::from("/")
        .join("Applications")
        .join("Steam.app")
        .join("Contents")
        .join("MacOS")
        .join("steam.sh");

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}

pub fn get_manifests_path() -> Result<PathBuf> {
    let manifests_path = ProjectDirs::from("", "", "Steam")
        .unwrap()
        .config_dir()
        .join("steamapps");

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}
