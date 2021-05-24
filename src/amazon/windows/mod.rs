use crate::amazon::{
    sqlite,
    windows::utils::{get_launcher_path, get_manifests_path},
};
use crate::error::Result;
use crate::prelude::Game;

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_path = get_launcher_path().unwrap();
    let manifests_path = get_manifests_path(&launcher_path).unwrap();

    return sqlite::read(&manifests_path, &launcher_path);
}
