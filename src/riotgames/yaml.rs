use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
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
    let manifest_file = std::fs::read_to_string(&file)
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Invalid Riot Games manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let product_settings = serde_yaml::from_str::<ProductSettings>(manifest_file.as_str())
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Invalid Riot Games manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let installs_file = std::fs::read_to_string(&launcher_path.join("RiotClientInstalls.json"))
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Invalid Riot Games manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let riot_client_installs = serde_yaml::from_str::<RiotClientInstalls>(installs_file.as_str())
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Invalid Riot Games manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

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

    let game = Game {
        _type: String::from("riotgames"),
        id: product.get_code().to_string(),
        name: product.get_name().to_string(),
        path: game_install_path.display().to_string(),
        launch_command: vec![
            launcher_executable_path.display().to_string(),
            format!("--launch-product={}", product.get_code()),
            format!("--launch-patchline={}", product.get_server()),
        ],
    };

    return Ok(game);
}
