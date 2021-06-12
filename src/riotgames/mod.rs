use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    utils::io::get_files_recursive,
};

use self::utils::{get_launcher_path, get_manifests_path};

mod types;
#[cfg_attr(target_os = "windows", path = "utils/windows.rs")]
#[cfg_attr(target_os = "macos", path = "utils/macos.rs")]
mod utils;
mod yaml;

pub fn games() -> Result<Vec<Game>> {
    let launcher_path = get_launcher_path()?;
    let manifests_path = get_manifests_path(&launcher_path)?;
    let manifests = get_files_recursive(&manifests_path, |file| {
        file.display()
            .to_string()
            .ends_with(".product_settings.yaml")
    })
    .map_err(|error| {
        Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Riot Games path, maybe this launcher is not installed: {}",
                error.to_string()
            ),
        )
    })?;

    let mut games = Vec::new();

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

pub fn find(id: &str) -> Result<Game> {
    let launcher_path = get_launcher_path()?;
    let manifests_path = get_manifests_path(&launcher_path)?;
    let manifests = get_files_recursive(&manifests_path, |file| {
        file.display()
            .to_string()
            .ends_with(".product_settings.yaml")
            && file.file_name().unwrap().to_str().unwrap().starts_with(&id)
    })
    .map_err(|error| {
        Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Riot Games path, maybe this launcher is not installed: {}",
                error.to_string()
            ),
        )
    })?;

    let manifest = manifests.get(0).ok_or(Error::new(
        ErrorKind::GameNotFound,
        format!("Riot Games game with id ({}) does not exist", id),
    ))?;

    return yaml::read(&manifest, &launcher_path);
}
