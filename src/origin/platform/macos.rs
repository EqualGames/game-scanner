use crate::error::{Error, ErrorKind, Result};
use std::path::PathBuf;

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = PathBuf::from("/")
        .join("Applications")
        .join("Origin.app")
        .join("Contents")
        .join("MacOS")
        .join("Origin");

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Origin path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}

pub fn get_manifests_path() -> Result<PathBuf> {
    let manifests_path = PathBuf::from("/")
        .join("Library")
        .join("Application Support")
        .join("Origin")
        .join("LocalContent");

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Origin path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}
