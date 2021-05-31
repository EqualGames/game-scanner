use std::path::{Path, PathBuf};

use directories::BaseDirs;

use crate::error::{Error, ErrorKind, Result};

pub fn get_launcher_path() -> Result<PathBuf> {
    let launcher_path = BaseDirs::new()
        .map(|base_dirs| PathBuf::from(base_dirs.data_local_dir()))
        .map(|path| path.join("Amazon Games"));

    if launcher_path.is_none() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Amazon Games path, maybe this launcher is not installed",
        ));
    }

    let launcher_path = launcher_path.unwrap();

    if !launcher_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Amazon Games path, maybe this launcher is not installed: {}",
                launcher_path.display().to_string()
            ),
        ));
    }

    return Ok(launcher_path);
}

pub fn get_manifests_path(launcher_path: &Path) -> Result<PathBuf> {
    let launcher_manifests_path = launcher_path
        .join("Data")
        .join("Games")
        .join("Sql")
        .join("GameInstallInfo.sqlite");

    if !launcher_manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LibraryNotFound,
            format!("Amazon library could be empty"),
        ));
    }

    return Ok(launcher_manifests_path);
}
