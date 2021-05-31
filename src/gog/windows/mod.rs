use crate::{
    error::{Error, ErrorKind, Result},
    gog::windows::utils::{get_launcher_executable, get_manifest_info, get_manifests, parse_game},
    prelude::Game,
};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable().unwrap();
    let (manifests, manifest_ids) = get_manifests().unwrap();

    let mut games = Vec::<Game>::new();

    for game_id in manifest_ids {
        let (name, path) = get_manifest_info(&manifests, &game_id).unwrap();

        games.push(parse_game(&game_id, &name, &path, &launcher_executable));
    }

    return Ok(games);
}

pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable().unwrap();
    let (manifests, manifest_ids) = get_manifests().unwrap();

    let manifest_id = manifest_ids.iter().find(|item| item.as_str() == id);

    if manifest_id.is_none() {
        return Err(Error::new(
            ErrorKind::GameNotFound,
            format!("GOG game with id ({}) does not exist", id),
        ));
    }

    let manifest_id = manifest_id.unwrap();

    let (name, path) = get_manifest_info(&manifests, manifest_id).unwrap();

    Ok(parse_game(&manifest_id, &name, &path, &launcher_executable))
}
