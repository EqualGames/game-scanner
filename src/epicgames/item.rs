use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
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

pub fn read(file: &Path, launcher_executable: &Path) -> Result<Game> {
    let manifest_data = std::fs::read_to_string(&file)
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Invalid Epic Games manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let manifest = serde_json::from_str::<Manifest>(manifest_data.as_str())
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Error on read the Epic Games manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    if manifest.display_name.contains("Unreal Engine") {
        return Err(Error::new(
            ErrorKind::IgnoredApp,
            format!(
                "({}) {} is an invalid game",
                &manifest.catalog_item_id, &manifest.display_name
            ),
        ));
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
