use std::path::{Path, PathBuf};

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::{Game, GameCommands, GameState, GameType},
    utils::path::{fix_path_separator, get_filename},
};

use self::super::{
    proto::{product::ProductInstall, read_product},
    types::BlizzardGames,
};

pub fn read_all(file: &Path, launcher_executable: &Path) -> Result<Vec<Game>> {
    let database = read_product(file).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!("Invalid Blizzard manifest: {}", error.to_string()),
        )
    })?;

    let manifests = database
        .product_installs
        .into_iter()
        .filter(get_manifest_predicate)
        .collect::<Vec<ProductInstall>>();

    let mut games = Vec::new();

    for manifest in manifests {
        games.push(parse_manifest(&manifest, launcher_executable))
    }

    return Ok(games);
}

fn get_manifest_predicate(item: &ProductInstall) -> bool {
    item.product_code.ne("agent")
        && item.product_code.ne("bna")
        && item.clone().cached_product_state.map_or(true, |product| {
            product
                .base_product_state
                .map_or(true, |base_state| base_state.playable != false)
        })
}

pub fn read(id: &str, file: &Path, launcher_executable: &Path) -> Result<Game> {
    let database = read_product(file).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!("Invalid Blizzard manifest: {}", error.to_string()),
        )
    })?;

    let manifest = database
        .product_installs
        .into_iter()
        .find(|item| item.uid == id)
        .ok_or(Error::new(
            ErrorKind::GameNotFound,
            format!("Blizzard game with id ({}) does not exist", id),
        ))?;

    Ok(parse_manifest(&manifest, launcher_executable))
}

fn parse_manifest(manifest: &ProductInstall, launcher_executable: &Path) -> Game {
    let mut game_path = manifest
        .settings
        .clone()
        .map(|settings| settings.install_path)
        .map_or(PathBuf::new(), PathBuf::from);

    if cfg!(target_os = "windows") {
        game_path = fix_path_separator(&game_path);
    }

    let launch_code = BlizzardGames::from_uid(&manifest.uid).get_code();

    Game {
        _type: GameType::Blizzard.to_string(),
        id: String::from(&manifest.uid),
        name: get_filename(&game_path),
        path: Some(game_path),
        commands: GameCommands {
            install: Some(vec![
                launcher_executable.display().to_string(),
                format!("--game={}", launch_code),
            ]),
            launch: Some(vec![
                launcher_executable.display().to_string(),
                format!("--exec=\"launch {}\"", launch_code),
            ]),
            uninstall: None,
        },
        state: GameState {
            installed: true,
            needs_update: false,
            downloading: false,
            total_bytes: None,
            received_bytes: None,
        },
    }
}
