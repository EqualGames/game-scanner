use crate::{
    epicgames::{
        item,
        windows::utils::{get_launcher_executable, get_manifests_path},
    },
    error::{Error, ErrorKind, Result},
    prelude::Game,
    util::io::get_files,
};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable().unwrap();

    let manifests_path = get_manifests_path().unwrap();

    let manifests = get_files(&manifests_path, |file| file.extension().unwrap().eq("item"))
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Epic Games path, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let mut games = Vec::<Game>::new();

    for manifest in manifests {
        match item::read(&manifest, &launcher_executable) {
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
