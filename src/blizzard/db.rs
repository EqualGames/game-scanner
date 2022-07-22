use self::super::{
    proto::{product::ProductInstall, read_product},
    types::BlizzardGames,
};
use crate::{
    error::{Error, ErrorKind, Result},
    prelude::{Game, GameCommands, GameState, GameType},
    utils::path::{fix_path_separator, get_filename},
};
use std::{
    convert::identity,
    path::{Path, PathBuf},
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
    item.product_code.ne("agent") && item.product_code.ne("bna")
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

    let installed = manifest
        .cached_product_state
        .clone()
        .map_or(false, |cached_product_state| {
            cached_product_state
                .base_product_state
                .map_or(false, |state| state.installed || state.playable)
        });

    let needs_update =
        manifest
            .cached_product_state
            .clone()
            .map_or(false, |cached_product_state| {
                cached_product_state
                    .base_product_state
                    .map_or(false, |state| !state.update_complete)
            });

    let downloading = manifest
        .cached_product_state
        .clone()
        .map_or(false, |cached_product_state| {
            cached_product_state
                .update_progress
                .map_or(false, |update_progress| {
                    update_progress.download_remaining > 0
                })
        });

    let total_bytes = manifest
        .cached_product_state
        .clone()
        .map_or(None, |product| {
            product.update_progress.map_or(None, |update_progress| {
                Some(update_progress.total_to_download)
            })
        });

    let received_bytes = manifest
        .cached_product_state
        .clone()
        .map_or(None, |product| {
            product.update_progress.map_or(None, |update_progress| {
                let total_to_download: i64 =
                    i64::try_from(update_progress.total_to_download).map_or(0, identity);

                let download_remaining: i64 =
                    i64::try_from(update_progress.download_remaining).map_or(0, identity);

                let mut received = total_to_download - download_remaining;

                if received < 0 {
                    received = download_remaining;
                }

                u64::try_from(received).map_or(None, Some)
            })
        });

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
            installed,
            needs_update,
            downloading,
            total_bytes,
            received_bytes,
        },
    }
}
