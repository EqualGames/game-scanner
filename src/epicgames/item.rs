use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::prelude::Game;

#[derive(Serialize, Deserialize)]
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

pub fn read(file: &Path, launcher_path: &Path) -> std::io::Result<Game> {
    let file_data = std::fs::read_to_string(&file)?;
    let manifest: Manifest = serde_json::from_str(&file_data)?;

    if manifest.display_name.contains("Unreal Engine") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "invalid game",
        ));
    }

    let mut command: String = launcher_path.to_str().unwrap().to_string();
    command.push_str(" com.epicgames.launcher://apps/");
    command.push_str(&manifest.app_name);
    command.push_str("?action=launch&silent=true");

    let game = Game {
        _type: "epicgames".to_string(),
        id: manifest.catalog_item_id,
        name: manifest.display_name,
        path: manifest.install_location,
        launch_command: command,
    };

    return Ok(game);
}
