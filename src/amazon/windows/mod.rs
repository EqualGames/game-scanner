use self::utils::{get_launcher_executable, get_launcher_path, get_manifests_path};
use super::sqlite;
use crate::{
    error::{ErrorKind, Result},
    prelude::Game,
};
use std::path::PathBuf;

mod utils;

pub fn executable() -> Result<PathBuf> {
    return get_launcher_executable();
}

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

pub fn find(id: &str) -> Result<Game> {
    let launcher_path = get_launcher_path()?;
    let launcher_executable = get_launcher_executable()?;
    let manifests_path = get_manifests_path(&launcher_path)?;

    return sqlite::read(id, &manifests_path, &launcher_executable);
}
