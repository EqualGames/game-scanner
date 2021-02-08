use crate::prelude::Game;
use crate::util::error::make_io_error;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

pub fn read(file: &Path, launcher_executable: &Path) -> std::io::Result<Game> {
    let file_data = std::fs::read_to_string(&file).unwrap();
    let manifest: Manifest = serde_json::from_str(&file_data).unwrap();

    if manifest.display_name.contains("Unreal Engine") {
        return Err(make_io_error("invalid game"));
    }

    let game = Game {
        _type: String::from("epicgames"),
        id: manifest.catalog_item_id,
        name: manifest.display_name,
        path: manifest.install_location,
        launch_command: make_launch_command(&launcher_executable, &manifest.app_name),
    };

    return Ok(game);
}

fn make_launch_command(launcher_executable: &Path, id: &String) -> Vec<String> {
    let mut command = Vec::new();

    command.push(launcher_executable.display().to_string());
    command.push(format!(
        "com.epicgames.launcher://apps/{}?action=launch&silent=true",
        id
    ));

    return command;
}
