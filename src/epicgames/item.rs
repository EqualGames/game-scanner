use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::{Game, GameCommands, GameState, GameType},
};

#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    #[serde(rename(deserialize = "bIsIncompleteInstall"))]
    is_incomplete_install: bool,
    #[serde(rename(deserialize = "AppName"))]
    app_name: String,
    #[serde(rename(deserialize = "DisplayName"))]
    display_name: String,
    #[serde(rename(deserialize = "InstallLocation"))]
    install_location: String,
    #[serde(rename(deserialize = "LaunchExecutable"))]
    launch_executable: String,
}

pub fn read(file: &Path, launcher_executable: &Path) -> Result<Game> {
    let manifest_data = fs::read_to_string(&file).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!(
                "Invalid Epic Games manifest: {} {}",
                file.display().to_string(),
                error.to_string()
            ),
        )
    })?;

    let manifest = serde_json::from_str::<Manifest>(manifest_data.as_str()).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!(
                "Error on read the Epic Games manifest: {} {}",
                file.display().to_string(),
                error.to_string()
            ),
        )
    })?;

    if manifest.display_name.contains("Unreal Engine") {
        return Err(Error::new(
            ErrorKind::IgnoredApp,
            format!(
                "({}) {} is an invalid game",
                &manifest.app_name, &manifest.display_name
            ),
        ));
    }

    return Ok(Game {
        _type: GameType::EpicGames.to_string(),
        id: manifest.app_name.clone(),
        name: manifest.display_name.clone(),
        path: Some(PathBuf::from(manifest.install_location)),
        commands: GameCommands {
            install: None,
            launch: Some(vec![
                launcher_executable.display().to_string(),
                format!(
                    "com.epicgames.launcher://apps/{}?action=launch&silent=true",
                    &manifest.app_name
                ),
            ]),
            uninstall: None,
        },
        state: GameState {
            installed: !manifest.is_incomplete_install,
            needs_update: false,
            downloading: false,
            total_bytes: None,
            received_bytes: None,
        },
    });
}
