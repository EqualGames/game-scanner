use crate::gog::windows::utils::get_manifests_path;
use crate::{
    error::Result, gog::sqlite, gog::windows::utils::get_launcher_executable, prelude::Game,
};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let manifest_path = get_manifests_path();
    let launcher_executable = get_launcher_executable().unwrap();

    return sqlite::read_all(&manifest_path, &launcher_executable);
}

pub fn find(id: &str) -> Result<Game> {
    let manifest_path = get_manifests_path();
    let launcher_executable = get_launcher_executable().unwrap();

    return sqlite::read(id, &manifest_path, &launcher_executable);
}
