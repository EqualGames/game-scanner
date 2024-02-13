use std::path::PathBuf;

use self::utils::{get_launcher_executable, get_launcher_path, get_manifests_path};
use super::sqlite;
use crate::{
    error::{ErrorKind, Result},
    prelude::Game,
};

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
    let launcher_path = get_launcher_path()?;
    let launcher_executable = get_launcher_executable()?;
    let manifests_path = get_manifests_path(&launcher_path);

    match manifests_path {
        Ok(path) => sqlite::read_all(&path, &launcher_executable),
        Err(error) => match error.kind() {
            ErrorKind::LibraryNotFound => Ok(vec![]),
            _ => Err(error),
        },
    }
}

/// # Errors
///
/// Will return `Err` if the id is not found
pub fn find(id: &str) -> Result<Game> {
    let launcher_path = get_launcher_path()?;
    let launcher_executable = get_launcher_executable()?;
    let manifests_path = get_manifests_path(&launcher_path)?;

    sqlite::read(id, &manifests_path, &launcher_executable)
}
