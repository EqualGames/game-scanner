use std::path::PathBuf;

use self::utils::{get_launcher_executable, get_manifests_path};
use crate::{error::Result, prelude::Game};

mod sqlite;
#[cfg_attr(target_os = "windows", path = "platform/windows.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
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
    let manifest_path = get_manifests_path()?;
    let launcher_executable = get_launcher_executable()?;

    sqlite::read_all(&manifest_path, &launcher_executable)
}

/// # Errors
///
/// Will return `Err` if the id is not found
///
/// # Panics
///
/// Will panic if game is not installed
pub fn find(id: &str) -> Result<Game> {
    let manifest_path = get_manifests_path().unwrap();
    let launcher_executable = get_launcher_executable().unwrap();

    sqlite::read(id, &manifest_path, &launcher_executable)
}
