use std::path::PathBuf;

use self::platform::{get_launcher_executable, get_manifests_path};
use crate::{error::Result, prelude::Game};

mod db;
#[cfg_attr(target_os = "windows", path = "platform/windows.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
mod platform;
mod proto;
mod types;

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
    let launcher_executable = get_launcher_executable()?;
    let manifests_path = get_manifests_path();

    db::read_all(&manifests_path, &launcher_executable)
}

/// # Errors
///
/// Will return `Err` if the id is not found
pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable()?;
    let manifests_path = get_manifests_path();

    db::read(id, &manifests_path, &launcher_executable)
}
