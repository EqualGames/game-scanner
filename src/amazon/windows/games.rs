use std::io;
use std::path::PathBuf;

use directories::BaseDirs;

use crate::amazon::sqlite;
use crate::prelude::Game;

pub fn list() -> io::Result<Vec<Game>> {
    let launcher_path = BaseDirs::new()
        .map(|base_dirs| PathBuf::from(base_dirs.data_local_dir()))
        .map(|path| path.join("Amazon Games"))
        .unwrap();

    let launcher_database_path = launcher_path
        .join("Data")
        .join("Games")
        .join("Sql")
        .join("GameInstallInfo.sqlite");

    if !launcher_path.exists() || !launcher_database_path.exists() {
        return Ok(Vec::new());
    }

    return sqlite::read(&launcher_database_path, &launcher_path);
}
