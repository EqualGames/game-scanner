use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use self::super::types::RiotGamesProducts;
use crate::{
    error::{Error, ErrorKind, Result},
    prelude::{Game, GameCommands, GameState, GameType},
    utils::path::fix_path_separator,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductSettings {
    pub product_install_full_path: String,
    pub product_install_root:      String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RiotClientInstalls {
    pub associated_client: Option<HashMap<String, String>>,
    pub rc_default:        String,
    pub rc_live:           String,
}

pub fn read_riot_client_installs(file: &Path) -> Result<RiotClientInstalls> {
    let installs_file = std::fs::read_to_string(file).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!("Invalid Riot Games config: {} {error}", file.display()),
        )
    })?;

    return serde_json::from_str::<RiotClientInstalls>(installs_file.as_str()).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!("Invalid Riot Games config: {} {error}", file.display()),
        )
    });
}

pub fn read(file: &Path, launcher_path: &Path) -> Result<Game> {
    let manifest_data = std::fs::read_to_string(file).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!("Invalid Riot Games manifest: {} {error}", file.display()),
        )
    })?;

    let product_settings =
        serde_yaml::from_str::<ProductSettings>(&manifest_data).map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid Riot Games manifest: {} {error}", file.display()),
            )
        })?;

    let manifest_filename = String::from(file.file_name().unwrap().to_str().unwrap())
        .replace(".product_settings.yaml", "");

    let product = RiotGamesProducts::from_manifest_name(&manifest_filename);

    let mut game_path = product_settings.product_install_full_path;

    if !game_path.ends_with('/') {
        game_path.push('/');
    }

    let launcher_client_installs = launcher_path.join("RiotClientInstalls.json");

    let launcher_executable = read_riot_client_installs(&launcher_client_installs)?
        .associated_client
        .and_then(|data| {
            if cfg!(target_os = "windows") {
                return data.get(&game_path).cloned();
            }

            if cfg!(target_os = "macos") {
                let key = data
                    .keys()
                    .find(|path| path.starts_with(&game_path.to_string()));

                key?;

                return data.get(key.unwrap()).cloned();
            }

            None
        })
        .map(PathBuf::from);

    if launcher_executable.is_none() {
        return Err(Error::new(
            ErrorKind::IgnoredApp,
            format!(
                "({}) {} is an invalid game",
                &product.get_code(),
                &product.get_name()
            ),
        ));
    }

    let mut game_install_path = PathBuf::from(&game_path);

    if cfg!(target_os = "windows") {
        game_install_path = fix_path_separator(&game_install_path);
    }

    let mut launcher_executable_path = launcher_executable.unwrap();

    if cfg!(target_os = "windows") {
        launcher_executable_path = fix_path_separator(&launcher_executable_path);
    }

    Ok(Game {
        type_:    GameType::RiotGames.to_string(),
        id:       product.get_code().to_string(),
        name:     product.get_name().to_string(),
        path:     Some(game_install_path),
        commands: GameCommands {
            install:   None,
            launch:    Some(vec![
                launcher_executable_path.display().to_string(),
                format!("--launch-product={}", product.get_code()),
                format!("--launch-patchline={}", product.get_server()),
            ]),
            uninstall: Some(vec![
                launcher_executable_path.display().to_string(),
                format!("--uninstall-product={}", product.get_code()),
                format!("--uninstall-patchline={}", product.get_server()),
            ]),
        },
        state:    GameState {
            installed:      true,
            needs_update:   false,
            downloading:    false,
            total_bytes:    None,
            received_bytes: None,
        },
    })
}
