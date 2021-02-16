use crate::error::{Error, ErrorKind, Result};
use crate::origin::mfst;
use crate::prelude::Game;
use crate::util::io::*;
use crate::util::registry;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let launcher_executable = get_launcher_executable();

    if launcher_executable.is_err() {
        return Err(launcher_executable.err().unwrap());
    }

    let launcher_executable = launcher_executable.unwrap();

    let manifests_path = get_manifests_path();

    if manifests_path.is_err() {
        return Err(manifests_path.err().unwrap());
    }

    let manifests_path = manifests_path.unwrap();

    let manifests = get_files_recursive(&manifests_path, get_manifest_predicate);

    if manifests.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Origin path, maybe this launcher is not installed: {}",
                manifests.err().unwrap().to_string()
            ),
        ));
    }

    for manifest in manifests.unwrap() {
        match mfst::read(&manifest, &launcher_executable) {
            Ok(g) => games.push(g),
            Err(error) => {
                if error.kind().ne(&ErrorKind::IgnoredApp) {
                    return Err(error);
                }
            }
        }
    }

    return Ok(games);
}

fn get_manifest_predicate(file: &PathBuf) -> bool {
    return file.extension().unwrap().eq("mfst");
}

fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_local_machine_reg_key("Origin")
        .and_then(|launcher_reg| registry::get_value(&launcher_reg, "ClientPath"))
        .map(PathBuf::from);

    if launcher_executable.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Origin path, maybe this launcher is not installed",
        ));
    }

    let launcher_executable = launcher_executable.unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid GOG path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}

fn get_manifests_path() -> Result<PathBuf> {
    let manifests_path = PathBuf::from("C:")
        .join(std::path::MAIN_SEPARATOR.to_string())
        .join("ProgramData")
        .join("Origin")
        .join("LocalContent");

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
