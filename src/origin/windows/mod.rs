use crate::{
    error::{Error, ErrorKind, Result},
    origin::{
        mfst,
        windows::utils::{get_launcher_executable, get_manifests_path},
    },
    prelude::Game,
    util::io::get_files_recursive,
};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable().unwrap();
    let manifests_path = get_manifests_path().unwrap();
    let manifests =
        get_files_recursive(&manifests_path, |file| file.extension().unwrap().eq("mfst"))
            .map_err(|error| {
                Error::new(
                    ErrorKind::LauncherNotFound,
                    format!(
                        "Invalid Origin path, maybe this launcher is not installed: {}",
                        error.to_string()
                    ),
                )
            })
            .unwrap();

    let mut games = Vec::new();

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
