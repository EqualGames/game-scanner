use std::path::PathBuf;

use crate::error::{Error, ErrorKind, Result};

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = PathBuf::from("/")
        .join("Applications")
        .join("Battle.net.app")
        .join("Contents")
        .join("MacOS")
        .join("Battle.net");

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Blizzard path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}

pub fn get_manifests_path() -> Result<PathBuf> {
    let manifests_path = PathBuf::from("/")
        .join("Users")
        .join("Shared")
        .join("Battle.net")
        .join("Agent")
        .join("product.db");

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Blizzard path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}
