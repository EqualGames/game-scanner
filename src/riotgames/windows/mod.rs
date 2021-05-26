use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    riotgames::{
        windows::utils::{get_launcher_path, get_manifests_path},
        yaml,
    },
    util::io::get_files_recursive,
};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_path = get_launcher_path().unwrap();
    let manifests_path = get_manifests_path(&launcher_path).unwrap();
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
    })
    .unwrap();

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
