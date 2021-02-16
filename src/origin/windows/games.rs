use crate::error::{Error, ErrorKind, Result};
use crate::origin::mfst;
use crate::prelude::Game;
use crate::util::io::*;
use crate::util::registry::*;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let launcher_info = get_local_machine_reg_key("Origin")
        .and_then(|launcher_reg| {
            launcher_reg
                .get_value::<String, &str>("ClientPath")
                .map_err(Error::from)
        })
        .map(PathBuf::from);

    if launcher_info.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            "Invalid Origin path, maybe this launcher is not installed",
        ));
    }

    let launcher_executable = launcher_info.unwrap();

    let manifests_path = PathBuf::from("C:")
        .join(std::path::MAIN_SEPARATOR.to_string())
        .join("ProgramData")
        .join("Origin")
        .join("LocalContent");

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid GOG path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    let manifests = get_files_recursive(&manifests_path, get_manifest_predicate)
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid GOG path, maybe this launcher is not installed: {} {}",
                    manifests_path.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    for manifest in manifests {
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
