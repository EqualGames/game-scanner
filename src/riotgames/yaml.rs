use crate::prelude::Game;
use crate::riotgames::types::RiotGamesProducts;
use crate::riotgames::utils::fix_path;
use crate::util::error::make_io_error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct ProductSettings {
    product_install_full_path: String,
    product_install_root: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RiotClientInstalls {
    associated_client: HashMap<String, String>,
}

pub fn read(file: &Path, launcher_path: &Path) -> io::Result<Game> {
    let product_settings: ProductSettings = std::fs::read_to_string(&file)
        .and_then(|data| {
            serde_yaml::from_str(data.as_str()).map_err(|error| make_io_error(&error.to_string()))
        })
        .unwrap();

    let riot_client_installs: RiotClientInstalls =
        std::fs::read_to_string(&launcher_path.join("RiotClientInstalls.json"))
            .and_then(|data| {
                serde_yaml::from_str(data.as_str())
                    .map_err(|error| make_io_error(&error.to_string()))
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
        .get(&game_path)
        .map(PathBuf::from);

    if launcher_executable.is_none() {
        return Err(make_io_error("invalid game of riot games"));
    }

    let game = Game {
        _type: String::from("riotgames"),
        id: product.get_code().to_string(),
        name: product.get_name().to_string(),
        path: fix_path(Path::new(&game_path)).display().to_string(),
        launch_command: vec![
            fix_path(&launcher_executable.unwrap())
                .display()
                .to_string(),
            format!("--launch-product={}", product.get_code()),
            format!("--launch-patchline={}", product.get_server()),
        ],
    };

    return Ok(game);
}
