use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
};

use self::utils::{get_launcher_executable, get_manifests_path};

mod sqlite;
#[cfg_attr(target_os = "windows", path = "utils/windows.rs")]
#[cfg_attr(target_os = "macos", path = "utils/macos.rs")]
mod utils;

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
