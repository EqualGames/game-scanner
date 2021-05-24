use std::path::PathBuf;

use crate::blizzard::db;
use crate::blizzard::windows::utils::get_launcher_executable;
use crate::error::Result;
use crate::prelude::Game;

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let manifests_path = PathBuf::from("C:\\ProgramData\\Battle.net\\Agent\\product.db");
    let launcher_executable = get_launcher_executable().unwrap();

    return db::read(&manifests_path, &launcher_executable);
}
