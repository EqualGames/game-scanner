use crate::amazon::sqlite;
use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use directories::BaseDirs;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
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
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Amazon Games path, maybe this launcher is not installed: {}",
                launcher_path.display().to_string()
            ),
        ));
    }

    return sqlite::read(&launcher_database_path, &launcher_path);
}
