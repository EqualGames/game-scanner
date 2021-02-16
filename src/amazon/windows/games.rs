use crate::amazon::sqlite;
use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use directories::BaseDirs;
use std::path::{Path, PathBuf};

pub fn list() -> Result<Vec<Game>> {
    let games = get_launcher_path()
        .and_then(|launcher_path| {
            get_manifests_path(&launcher_path).map(|manifests_path| (launcher_path, manifests_path))
        })
        .and_then(|(launcher_path, manifests_path)| sqlite::read(&manifests_path, &launcher_path));

    return match games {
        Ok(data) => Ok(data),
        Err(error) => Err(error),
    };
}

fn get_launcher_path() -> Result<PathBuf> {
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

fn get_manifests_path(launcher_path: &Path) -> Result<PathBuf> {
    let launcher_manifests_path = launcher_path
        .join("Data")
        .join("Games")
        .join("Sql")
        .join("GameInstallInfo.sqlite");

    if !launcher_manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Amazon Games path, maybe this launcher is not installed: {}",
                launcher_manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(launcher_manifests_path);
}
