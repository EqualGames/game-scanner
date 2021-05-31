use std::path::{Path, PathBuf};

use crate::blizzard::proto::product::ProductInstall;
use crate::blizzard::proto::{product, read_product};
use crate::blizzard::types::BlizzardGames;
use crate::error::{Error, ErrorKind, Result};
use crate::prelude::{Game, GameCommands, GameState, GameType};
use crate::util::path::{fix_path_separator, get_filename};

pub fn read_all(file: &Path, launcher_executable: &Path) -> Result<Vec<Game>> {
    let manifests = read_product(file)
        .map(|data| data.product_installs)
        .map(|product_installs| {
            product_installs
                .into_iter()
                .filter(get_manifest_predicate)
                .collect::<Vec<product::ProductInstall>>()
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid Blizzard manifest: {}", error.to_string()),
            )
        })
        .unwrap();

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
    let manifest = read_product(file)
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid Blizzard manifest: {}", error.to_string()),
            )
        })
        .ok()
        .map(|data| data.product_installs)
        .and_then(|product_installs| product_installs.into_iter().find(|item| item.uid == id));

    if manifest.is_none() {
        return Err(Error::new(
            ErrorKind::GameNotFound,
            format!("Blizzard game with id ({}) does not exist", id),
        ));
    }

    let manifest = manifest.unwrap();

    Ok(parse_manifest(&manifest, launcher_executable))
}

fn parse_manifest(manifest: &ProductInstall, launcher_executable: &Path) -> Game {
    let mut game_path = manifest
        .settings
        .clone()
        .map(|settings| settings.install_path)
        .map_or(PathBuf::new(), PathBuf::from);

    if cfg!(windows) {
        game_path = fix_path_separator(&game_path);
    }

    let launch_code = BlizzardGames::from_uid(&manifest.uid).get_code();

    Game {
        _type: GameType::Blizzard.to_string(),
        id: String::from(&manifest.uid),
        name: get_filename(&game_path),
        path: game_path.display().to_string(),
        commands: GameCommands {
            install: None,
            launch: vec![
                launcher_executable.display().to_string(),
                String::from("--exec"),
                format!("launch {}", launch_code),
            ],
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
