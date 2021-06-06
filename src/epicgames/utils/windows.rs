use std::path::PathBuf;

use crate::{
    error::{Error, ErrorKind, Result},
    utils::{path::fix_path_separator, registry},
};

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_current_user_reg_key("Epic Games\\EOS")
        .and_then(|eos_reg| registry::get_value(&eos_reg, "ModSdkCommand"))
        .map(|path| fix_path_separator(path.as_ref()));

    if launcher_executable.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Epic Games path, maybe this launcher is not installed: {}",
        ));
    }

    let launcher_executable_path = launcher_executable.unwrap();

    if !launcher_executable_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Epic Games path, maybe this launcher is not installed: {}",
                launcher_executable_path.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable_path);
}

pub fn get_manifests_path() -> Result<PathBuf> {
    let launcher_data = registry::get_local_machine_reg_key("Epic Games\\EpicGamesLauncher")
        .and_then(|launcher_reg| registry::get_value(&launcher_reg, "AppDataPath"))
        .map(PathBuf::from);

    if launcher_data.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Epic Games path, maybe this launcher is not installed",
        ));
    }

    let manifests_path = launcher_data.unwrap().join("Manifests");

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Epic Games path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}
