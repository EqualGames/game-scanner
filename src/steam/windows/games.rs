use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::steam::acf;
use crate::steam::vdf;
use crate::steam::windows::utils::fix_path;
use crate::util::io::*;
use crate::util::registry::*;
use std::path::PathBuf;

pub fn list() -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let (launcher_executable, launcher_reg) = get_local_machine_reg_key("Valve\\Steam")
        .and_then(|launcher_reg| {
            get_current_user_reg_key("Valve\\Steam")
                .and_then(|user_launcher_reg| {
                    user_launcher_reg
                        .get_value::<String, &str>("SteamExe")
                        .map_err(Error::from)
                })
                .map(|path| fix_path(&path))
                .map(PathBuf::from)
                .map(|launcher_executable| (launcher_executable, launcher_reg))
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Steam path, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    let steam_path_default = PathBuf::from(fix_path(
        &launcher_reg
            .get_value::<String, &str>("InstallPath")
            .map_err(|error| {
                Error::new(
                    ErrorKind::LauncherNotFound,
                    format!(
                        "Invalid Steam path, maybe this launcher is not installed: {}",
                        error.to_string()
                    ),
                )
            })
            .unwrap(),
    ))
    .join("steamapps");

    if !steam_path_default.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                steam_path_default.display().to_string()
            ),
        ));
    }

    let mut library_paths = Vec::new();
    library_paths.push(steam_path_default.clone());

    let library_folders =
        vdf::read_library_folders(&steam_path_default.join("libraryfolders.vdf")).unwrap();

    for folder in library_folders {
        library_paths.push(folder.join("steamapps"));
    }

    let mut library_manifests = Vec::<(PathBuf, Vec<PathBuf>)>::new();

    for path in library_paths {
        match get_files(&path, get_manifest_predicate) {
            Ok(list) => library_manifests.push((path, list)),
            Err(error) => {
                Error::new(
                    ErrorKind::LauncherLibraryNotFound,
                    format!(
                        "Invalid Steam library path, maybe this launcher is not installed: {} {}",
                        steam_path_default.display().to_string(),
                        error.to_string()
                    ),
                )
                .print();
            }
        }
    }

    for library_manifest in library_manifests {
        let (library_path, manifests) = library_manifest;

        for manifest in manifests {
            match acf::read(&manifest, &launcher_executable, &library_path) {
                Ok(g) => games.push(g),
                Err(error) => {
                    if error.kind().ne(&ErrorKind::IgnoredApp) {
                        return Err(error);
                    }
                }
            }
        }
    }

    return Ok(games);
}

fn get_manifest_predicate(file: &PathBuf) -> bool {
    return file.extension().unwrap().eq("acf");
}
