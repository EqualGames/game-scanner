use std::path::{Path, PathBuf};

use self::platform::{get_launcher_executable, get_library_manifests};
use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    utils::path::get_filename,
};

mod acf;
mod platform;
mod types;
mod vdf;

/// # Errors
///
/// Will return `Err` if the executable is not found
pub fn executable() -> Result<PathBuf> {
    get_launcher_executable()
}

/// # Errors
///
/// Will return `Err` if games are not found
pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable()?;
    let library_manifests = get_library_manifests(|file| {
        Path::new(&get_filename(file))
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("acf"))
    })?;

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

    Ok(games)
}

/// # Errors
///
/// Will return `Err` if the id is not found
pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable()?;
    let library_manifests = get_library_manifests(|file| get_filename(file).contains(id))?;
    let library_manifests = library_manifests.iter().find(|(_, list)| !list.is_empty());
    let (library_path, manifests) = library_manifests.ok_or_else(|| {
        Error::new(
            ErrorKind::GameNotFound,
            format!("Steam game with id ({id}) does not exist"),
        )
    })?;

    let manifest = manifests.first().ok_or_else(|| {
        Error::new(
            ErrorKind::GameNotFound,
            format!("Steam game with id ({id}) does not exist"),
        )
    })?;

    acf::read(manifest, &launcher_executable, library_path)
}
