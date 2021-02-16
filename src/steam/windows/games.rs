use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::steam::acf;
use crate::steam::vdf;
use crate::steam::windows::utils::fix_path;
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

    let mut library_paths = Vec::new();
    library_paths.push(manifests_path.clone());

    let library_folders =
        vdf::read_library_folders(&manifests_path.join("libraryfolders.vdf")).unwrap();

    for folder in library_folders {
        library_paths.push(fix_path(&folder).join("steamapps"));
    }

    let mut library_manifests = Vec::<(PathBuf, Vec<PathBuf>)>::new();

    for path in library_paths {
        match get_files(&path, get_manifest_predicate) {
            Ok(list) => library_manifests.push((path, list)),
            Err(error) => {
                Error::new(
                    ErrorKind::LibraryNotFound,
                    format!(
                        "Invalid Steam library path, maybe this launcher is not installed: {} {}",
                        manifests_path.display().to_string(),
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

fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_current_user_reg_key("Valve\\Steam")
        .and_then(|user_launcher_reg| registry::get_value(&user_launcher_reg, "SteamExe"))
        .map(PathBuf::from);

    if launcher_executable.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                launcher_executable.err().unwrap().to_string()
            ),
        ));
    }

    let launcher_executable = fix_path(&launcher_executable.unwrap());

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}

fn get_manifests_path() -> Result<PathBuf> {
    let manifests_path = registry::get_local_machine_reg_key("Valve\\Steam")
        .and_then(|launcher_reg| registry::get_value(&launcher_reg, "InstallPath"))
        .map(PathBuf::from)
        .map(|path| path.join("steamapps"));

    if manifests_path.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                manifests_path.err().unwrap().to_string()
            ),
        ));
    }

    let manifests_path = manifests_path.unwrap();

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}
