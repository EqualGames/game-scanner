use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    steam::{
        acf,
        windows::utils::{get_launcher_executable, get_library_manifests},
    },
};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable().unwrap();
    let library_manifests =
        get_library_manifests(|file| file.extension().unwrap().eq("acf")).unwrap();

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

pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable().unwrap();

    let library_manifests =
        get_library_manifests(|file| file.display().to_string().contains(&id)).unwrap();

    let library_manifests = library_manifests.iter().find(|(_, list)| list.len() > 0);

    if library_manifests.is_none() {
        return Err(Error::new(
            ErrorKind::GameNotFound,
            format!("Steam game with id ({}) does not exist", id),
        ));
    }

    let (library_path, manifests) = library_manifests.unwrap();
    let manifest = manifests.get(0).unwrap();

    return acf::read(&manifest, &launcher_executable, &library_path);
}
