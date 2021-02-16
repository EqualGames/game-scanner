use crate::error::{Error, ErrorKind, Result};
use crate::prelude::{Game, GameType};
use crate::riotgames::types::RiotGamesProducts;
use crate::util::path::fix_path_separator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct ProductSettings {
    product_install_full_path: String,
    product_install_root: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RiotClientInstalls {
    associated_client: Option<HashMap<String, String>>,
}

pub fn read(file: &Path, launcher_path: &Path) -> Result<Game> {
    let manifest_data = std::fs::read_to_string(&file).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!(
                "Invalid Riot Games manifest: {} {}",
                file.display().to_string(),
                error.to_string()
            ),
        )
    });

    if manifest_data.is_err() {
        return Err(manifest_data.err().unwrap());
    }

    let manifest_data = manifest_data.unwrap();

    let product_settings =
        serde_yaml::from_str::<ProductSettings>(&manifest_data).map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Invalid Riot Games manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        });

    if product_settings.is_err() {
        return Err(product_settings.err().unwrap());
    }

    let product_settings = product_settings.unwrap();

    let launcher_installs = launcher_path.join("RiotClientInstalls.json");

    let installs_file = std::fs::read_to_string(&launcher_installs).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!(
                "Invalid Riot Games manifest: {} {}",
                file.display().to_string(),
                error.to_string()
            ),
        )
    });

    if installs_file.is_err() {
        return Err(installs_file.err().unwrap());
    }

    let installs_file = installs_file.unwrap();

    let riot_client_installs = serde_yaml::from_str::<RiotClientInstalls>(installs_file.as_str())
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Invalid Riot Games manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        });

    if riot_client_installs.is_err() {
        return Err(riot_client_installs.err().unwrap());
    }

    let riot_client_installs = riot_client_installs.unwrap();

    let manifest_filename = String::from(file.file_name().unwrap().to_str().unwrap())
        .replace(".product_settings.yaml", "");

    let product = RiotGamesProducts::from_manifest_name(&manifest_filename);

    let mut game_path = product_settings.product_install_full_path.clone();

    if !game_path.ends_with("/") {
        game_path.push_str("/");
    }

    let launcher_executable = riot_client_installs
        .associated_client
        .and_then(|data| data.get(&game_path).cloned())
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

    if cfg!(windows) {
        game_install_path = fix_path_separator(&game_install_path);
    }

    let mut launcher_executable_path = launcher_executable.unwrap();

    if cfg!(windows) {
        launcher_executable_path = fix_path_separator(&launcher_executable_path);
    }

    return Ok(Game {
        _type: GameType::RiotGames.to_string(),
        id: product.get_code().to_string(),
        name: product.get_name().to_string(),
        path: game_install_path.display().to_string(),
        launch_command: vec![
            launcher_executable_path.display().to_string(),
            format!("--launch-product={}", product.get_code()),
            format!("--launch-patchline={}", product.get_server()),
        ],
    });
}
