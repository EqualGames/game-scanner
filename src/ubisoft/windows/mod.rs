use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
};

use self::utils::{get_game_info, get_launcher_executable, get_manifest_ids, parse_game_info};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable()?;
    let manifest_ids = get_manifest_ids()?;

    let mut games = Vec::new();

    for manifest_id in manifest_ids {
        let game_info = get_game_info(&manifest_id)?;

        games.push(parse_game_info(
            &manifest_id,
            &game_info,
            &launcher_executable,
        ));
    }

    return Ok(games);
}

pub fn find(id: &str) -> Result<Game> {
    let launcher_executable = get_launcher_executable()?;

    let manifest_ids = get_manifest_ids()?;

    let manifest_id = manifest_ids.iter().find(|item| item.as_str() == id);

    let manifest_id = manifest_id.ok_or(Error::new(
        ErrorKind::GameNotFound,
        format!("Ubisoft game with id ({}) does not exist", id),
    ))?;

    let game_info = get_game_info(&manifest_id)?;

    Ok(parse_game_info(
        manifest_id,
        &game_info,
        &launcher_executable,
    ))
}
