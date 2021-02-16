use crate::blizzard::db;
use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::util::registry::get_local_machine_reg_key;
use std::path::{Path, PathBuf};

pub fn list() -> Result<Vec<Game>> {
    let launcher_info =
        get_local_machine_reg_key("Microsoft\\Windows\\CurrentVersion\\Uninstall\\Battle.net")
            .and_then(|launcher_reg| {
                launcher_reg
                    .get_value::<String, &str>("DisplayIcon")
                    .map_err(Error::from)
            })
            .map(PathBuf::from);

    if launcher_info.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Blizzard path, maybe this launcher is not installed",
        ));
    }

    let launcher_executable = launcher_info.unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Blizzard path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return db::read(
        Path::new("C:\\ProgramData\\Battle.net\\Agent\\product.db"),
        &launcher_executable,
    );
}
