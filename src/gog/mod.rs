use self::utils::{get_launcher_executable, get_manifests_path};
use crate::{error::Result, prelude::Game};
use std::path::PathBuf;

mod sqlite;
#[cfg_attr(target_os = "windows", path = "platform/windows.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
mod utils;

pub fn executable() -> Result<PathBuf> {
    return get_launcher_executable();
}

pub fn games() -> Result<Vec<Game>> {
    let manifest_path = get_manifests_path()?;
    let launcher_executable = get_launcher_executable()?;

    return sqlite::read_all(&manifest_path, &launcher_executable);
}

pub fn find(id: &str) -> Result<Game> {
    let manifest_path = get_manifests_path().unwrap();
    let launcher_executable = get_launcher_executable().unwrap();

    return sqlite::read(id, &manifest_path, &launcher_executable);
}
