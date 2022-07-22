use crate::{
    error::{Error, ErrorKind, Result},
    utils::registry,
};
use std::path::PathBuf;

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_local_machine_reg_key("Origin")
        .and_then(|launcher_reg| registry::get_value(&launcher_reg, "ClientPath"))
        .map(PathBuf::from);

    if launcher_executable.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Origin path, maybe this launcher is not installed",
        ));
    }

    let launcher_executable = launcher_executable?;

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Origin path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}

pub fn get_manifests_path() -> Result<PathBuf> {
    let manifests_path = PathBuf::from("C:")
        .join(std::path::MAIN_SEPARATOR.to_string())
        .join("ProgramData")
        .join("Origin")
        .join("LocalContent");

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Origin path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}
