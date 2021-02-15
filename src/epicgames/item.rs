use crate::prelude::Game;
use crate::util::error::make_io_error;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    #[serde(rename(deserialize = "AppName"))]
    app_name: String,
    #[serde(rename(deserialize = "CatalogItemId"))]
    catalog_item_id: String,
    #[serde(rename(deserialize = "DisplayName"))]
    display_name: String,
    #[serde(rename(deserialize = "InstallLocation"))]
    install_location: String,
    #[serde(rename(deserialize = "LaunchExecutable"))]
    launch_executable: String,
}

pub fn read(file: &Path, launcher_executable: &Path) -> std::io::Result<Game> {
    let manifest: Manifest = std::fs::read_to_string(&file)
        .and_then(|data| {
            serde_json::from_str(data.as_str()).map_err(|error| make_io_error(&error.to_string()))
        })
        .unwrap();

    if manifest.display_name.contains("Unreal Engine") {
        return Err(make_io_error("invalid game"));
    }

    let game = Game {
        _type: String::from("epicgames"),
        id: manifest.catalog_item_id.clone(),
        name: manifest.display_name.clone(),
        path: manifest.install_location.clone(),
        launch_command: vec![
            launcher_executable.display().to_string(),
            format!(
                "com.epicgames.launcher://apps/{}?action=launch&silent=true",
                &manifest.catalog_item_id
            ),
        ],
    };

    return Ok(game);
}
