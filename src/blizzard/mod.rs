use self::platform::{get_launcher_executable, get_manifests_path};
use crate::{error::Result, prelude::Game};
use std::path::PathBuf;

mod db;
#[cfg_attr(target_os = "windows", path = "platform/windows.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
mod platform;
mod proto;
mod types;

pub fn executable() -> Result<PathBuf> {
    return get_launcher_executable();
}

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable()?;
    let manifests_path = get_manifests_path()?;

    return db::read_all(&manifests_path, &launcher_executable);
}

pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable()?;
    let manifests_path = get_manifests_path()?;

    return db::read(id, &manifests_path, &launcher_executable);
}
