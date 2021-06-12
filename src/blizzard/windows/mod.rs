use crate::{error::Result, prelude::Game};

use super::db;

use self::utils::{get_launcher_executable, get_manifests_path};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable()?;

    return db::read_all(&get_manifests_path(), &launcher_executable);
}

pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable()?;

    return db::read(id, &get_manifests_path(), &launcher_executable);
}
