use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::riotgames::yaml;
use crate::util::io::get_files_recursive;
use std::path::{Path, PathBuf};

pub fn list() -> Result<Vec<Game>> {
    let launcher_path = get_launcher_path();

    if launcher_path.is_err() {
        return Err(launcher_path.err().unwrap());
    }

    let launcher_path = launcher_path.unwrap();

    let manifests_path = get_manifests_path(&launcher_path);

    if manifests_path.is_err() {
        return Err(manifests_path.err().unwrap());
    }

    let manifests_path = manifests_path.unwrap();

    let manifests = get_files_recursive(&manifests_path, get_manifest_predicate);

    if manifests.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Riot Games path, maybe this launcher is not installed: {}",
                manifests.err().unwrap().to_string()
            ),
        ));
    }

    let mut games = Vec::new();

    for manifest in manifests.unwrap() {
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

fn get_launcher_path() -> Result<PathBuf> {
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

    return Ok(launcher_path);
}

fn get_manifests_path(launcher_path: &Path) -> Result<PathBuf> {
    let manifests_path = launcher_path.join("Metadata");

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Riot Games path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}
