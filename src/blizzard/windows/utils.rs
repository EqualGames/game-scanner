use std::path::PathBuf;

use crate::error::{Error, ErrorKind, Result};
use crate::util::registry;

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_local_machine_reg_key(
        "Microsoft\\Windows\\CurrentVersion\\Uninstall\\Battle.net",
    )
    .and_then(|launcher_reg| registry::get_value(&launcher_reg, "DisplayIcon"))
    .map(PathBuf::from)
    .map_err(|_error| {
        Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Blizzard path, maybe this launcher is not installed",
        )
    })
    .unwrap();

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
