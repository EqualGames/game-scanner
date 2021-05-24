use std::path::PathBuf;

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    steam::{
        acf, vdf,
        windows::utils::{get_launcher_executable, get_manifest_predicate, get_manifests_path},
    },
    util::io::get_files,
};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable().unwrap();
    let manifests_path = get_manifests_path().unwrap();

    let mut library_paths = Vec::new();
    library_paths.push(manifests_path.clone());

    let library_folders =
        vdf::read_library_folders(&manifests_path.join("libraryfolders.vdf")).unwrap();

    for folder in library_folders {
        library_paths.push(folder.join("steamapps"));
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

    let mut games = Vec::new();

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
