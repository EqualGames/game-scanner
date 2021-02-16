use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::riotgames::yaml;
use crate::util::io::get_files_recursive;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let launcher_path = PathBuf::from("C:")
        .join(std::path::MAIN_SEPARATOR.to_string())
        .join("ProgramData")
        .join("Riot Games");

    if !launcher_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Riot Games path, maybe this launcher is not installed: {}",
                launcher_path.display().to_string()
            ),
        ));
    }

    let launcher_manifests_path = launcher_path.join("Metadata");

    let manifests = get_files_recursive(&launcher_manifests_path, get_manifest_predicate)
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Riot Games path, maybe this launcher is not installed: {} {}",
                    launcher_manifests_path.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    for manifest in manifests {
        match yaml::read(&manifest, &launcher_path) {
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
    file.display()
        .to_string()
        .ends_with(".product_settings.yaml")
}
