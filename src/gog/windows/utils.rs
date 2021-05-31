use std::path::{Path, PathBuf};

use winreg::RegKey;

use crate::error::{Error, ErrorKind, Result};
use crate::prelude::{Game, GameCommands, GameState, GameType};
use crate::util::registry;

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_local_machine_reg_key("GOG.com\\GalaxyClient\\paths")
        .and_then(|launcher_paths_reg| registry::get_value(&launcher_paths_reg, "client"))
        .map(PathBuf::from)
        .and_then(|launcher_path| {
            registry::get_local_machine_reg_key("GOG.com\\GalaxyClient")
                .and_then(|launcher_reg| registry::get_value(&launcher_reg, "clientExecutable"))
                .map(|launcher_filename| launcher_path.join(launcher_filename))
        })
        .map_err(|_error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                "Invalid GOG path, maybe this launcher is not installed",
            )
        })
        .unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid GOG path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    Ok(launcher_executable)
}

pub fn get_manifests() -> Result<(RegKey, Vec<String>)> {
    let manifests = registry::get_local_machine_reg_key("GOG.com\\Games")
        .map(|launcher_games_reg| launcher_games_reg)
        .map_err(|_error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                "Invalid GOG path, maybe this launcher is not installed",
            )
        })
        .unwrap();

    let manifest_ids = manifests
        .enum_keys()
        .map(|key| key.unwrap())
        .collect::<Vec<_>>();

    Ok((manifests, manifest_ids))
}

pub fn get_manifest_info(manifests: &RegKey, id: &str) -> Result<(String, String)> {
    return registry::get_sub_key(manifests, id)
        .and_then(|game_info_reg| {
            registry::get_value(&game_info_reg, "gameName")
                .map(|game_name| (game_info_reg, game_name))
        })
        .and_then(|(game_info_reg, game_name)| {
            registry::get_value(&game_info_reg, "path").map(|game_path| (game_name, game_path))
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error on read the GOG manifest: {} {}",
                    id,
                    error.to_string()
                ),
            )
        });
}

pub fn parse_game(id: &String, name: &String, path: &String, launcher_executable: &Path) -> Game {
    Game {
        _type: GameType::GOG.to_string(),
        id: id.clone(),
        name: name.clone(),
        path: path.clone(),
        commands: GameCommands {
            install: None,
            launch: vec![
                launcher_executable.display().to_string(),
                String::from("/command=runGame"),
                format!("/gameId={}", &id),
                format!("/path={}", &path),
            ],
            uninstall: None,
        },
        state: GameState {
            installed: true,
            needs_update: false,
            downloading: false,
            total_bytes: None,
            received_bytes: None,
        },
    }
}
