use crate::blizzard::db;
use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::util::registry;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let manifests_path = PathBuf::from("C:\\ProgramData\\Battle.net\\Agent\\product.db");

    let games = get_launcher_executable()
        .and_then(|launcher_executable| db::read(&manifests_path, &launcher_executable));

    return match games {
        Ok(data) => Ok(data),
        Err(error) => Err(error),
    };
}

fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_info = registry::get_local_machine_reg_key(
        "Microsoft\\Windows\\CurrentVersion\\Uninstall\\Battle.net",
    )
    .and_then(|launcher_reg| registry::get_value(&launcher_reg, "DisplayIcon"))
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

    return Ok(launcher_executable);
}
