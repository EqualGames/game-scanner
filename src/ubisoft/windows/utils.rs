use std::path::PathBuf;

use crate::error::{Error, ErrorKind, Result};
use crate::util::path::fix_path_separator;
use crate::util::registry;

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable =
        registry::get_local_machine_reg_key("Ubisoft\\Launcher").and_then(|launcher_reg| {
            registry::get_value(&launcher_reg, "InstallDir")
                .map(PathBuf::from)
                .map(|path| path.join("upc.exe"))
        });

    if launcher_executable.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Ubisoft path, maybe this launcher is not installed: {}",
                launcher_executable.err().unwrap().to_string()
            ),
        ));
    }

    let launcher_executable = launcher_executable.unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Ubisoft path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}

pub fn get_manifest_ids() -> Result<Vec<String>> {
    let manifests = registry::get_local_machine_reg_key("Ubisoft\\Launcher")
        .and_then(|launcher_reg| registry::get_sub_key(&launcher_reg, "Installs"))
        .map(|manifests| {
            manifests
                .enum_keys()
                .map(|x| x.unwrap())
                .collect::<Vec<String>>()
        });

    if manifests.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Ubisoft path, maybe this launcher is not installed: {}",
                manifests.err().unwrap().to_string()
            ),
        ));
    }

    return manifests;
}

pub fn get_game_info(manifest_id: &String) -> Result<(String, String)> {
    let manifest = registry::get_local_machine_reg_key(
        format!(
            "Microsoft\\Windows\\CurrentVersion\\Uninstall\\Uplay Install {}",
            manifest_id
        )
        .as_str(),
    )
    .and_then(|game_reg| {
        registry::get_value(&game_reg, "DisplayName").and_then(|game_name| {
            registry::get_value(&game_reg, "InstallLocation")
                .map(|value| fix_path_separator(value.as_ref()).display().to_string())
                .map(|game_path| (game_name, game_path))
        })
    });

    if manifest.is_err() {
        return Err(Error::new(
            ErrorKind::InvalidApp,
            format!(
                "Error on read the Ubisoft manifest: {}",
                manifest.err().unwrap().to_string()
            ),
        ));
    }

    let manifest = manifest.unwrap();

    return Ok(manifest);
}