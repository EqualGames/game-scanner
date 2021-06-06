use crate::{
    error::{ErrorKind, Result},
    prelude::Game,
};

use super::sqlite;

use self::utils::{get_launcher_path, get_manifests_path};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_path = get_launcher_path().unwrap();
    let manifests_path = get_manifests_path(&launcher_path);

    match manifests_path {
        Ok(path) => sqlite::read_all(&path, &launcher_path),
        Err(error) => match error.kind() {
            ErrorKind::LibraryNotFound => Ok(vec![]),
            _ => Err(error),
        },
    }
}

pub fn find(id: &str) -> Result<Game> {
    let launcher_path = get_launcher_path().unwrap();
    let manifests_path = get_manifests_path(&launcher_path).unwrap();

    return sqlite::read(id, &manifests_path, &launcher_path);
}
