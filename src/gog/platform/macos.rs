use crate::error::{Error, ErrorKind, Result};
use std::path::PathBuf;

pub fn get_manifests_path() -> Result<PathBuf> {
    let manifest_path = PathBuf::from("/")
        .join("Users")
        .join("Shared")
        .join("GOG.com")
        .join("Galaxy")
        .join("Storage")
        .join("galaxy-2.0.db");

    if !manifest_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid GOG path, maybe this launcher is not installed: {}",
                manifest_path.display().to_string()
            ),
        ));
    }

    return Ok(manifest_path);
}

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = PathBuf::from("/")
        .join("Applications")
        .join("GOG Galaxy.app")
        .join("Contents")
        .join("MacOS")
        .join("GOG Galaxy");

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid GOG path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}
