use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::ubisoft::windows::utils::{
    get_game_info, get_launcher_executable, get_manifest_ids, parse_game_info,
};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable().unwrap();
    let manifest_ids = get_manifest_ids().unwrap();

    let mut games = Vec::new();

    for manifest_id in manifest_ids {
        let game_info = get_game_info(&manifest_id).unwrap();

        games.push(parse_game_info(
            &manifest_id,
            &game_info,
            &launcher_executable,
        ));
    }

    return Ok(games);
}

pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable().unwrap();

    let manifest_ids = get_manifest_ids().unwrap();

    let manifest_id = manifest_ids.iter().find(|item| item.as_str() == id);

    if manifest_id.is_none() {
        return Err(Error::new(
            ErrorKind::GameNotFound,
            format!("Ubisoft game with id ({}) does not exist", id),
        ));
    }

    let manifest_id = manifest_id.unwrap();
    let game_info = get_game_info(&manifest_id).unwrap();

    Ok(parse_game_info(
        manifest_id,
        &game_info,
        &launcher_executable,
    ))
}
