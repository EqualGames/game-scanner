use std::path::{Path, PathBuf};

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::{Game, GameCommands, GameState, GameType},
    utils::{path::fix_path_separator, registry},
};

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_local_machine_reg_key("Ubisoft\\Launcher")
        .and_then(|launcher_reg| {
            registry::get_value(&launcher_reg, "InstallDir")
                .map(PathBuf::from)
                .map(|path| path.join("upc.exe"))
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Ubisoft path, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        })?;

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Ubisoft path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}

pub fn get_manifest_ids() -> Result<Vec<String>> {
    return registry::get_local_machine_reg_key("Ubisoft\\Launcher")
        .and_then(|launcher_reg| registry::get_sub_key(&launcher_reg, "Installs"))
        .map(|manifests| {
            manifests
                .enum_keys()
                .map(|x| x.unwrap())
                .collect::<Vec<String>>()
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Ubisoft path, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        });
}

pub fn get_game_info(manifest_id: &String) -> Result<(String, PathBuf)> {
    registry::get_local_machine_reg_key(
        format!(
            "Microsoft\\Windows\\CurrentVersion\\Uninstall\\Uplay Install {}",
            manifest_id
        )
        .as_str(),
    )
    .and_then(|game_reg| {
        registry::get_value(&game_reg, "DisplayName").and_then(|game_name| {
            registry::get_value(&game_reg, "InstallLocation")
                .map(|value| fix_path_separator(value.as_ref()))
                .map(|game_path| (game_name, game_path))
        })
    })
    .map_err(|error| {
        Error::new(
            ErrorKind::InvalidGame,
            format!("Error on read the Ubisoft manifest: {}", error.to_string()),
        )
    })
}

pub fn parse_game_info(
    id: &String,
    game_info: &(String, PathBuf),
    launcher_executable: &Path,
) -> Game {
    let (name, path) = game_info;

    Game {
        _type: GameType::Ubisoft.to_string(),
        id: id.clone(),
        name: name.clone(),
        path: Some(path.clone()),
        commands: GameCommands {
            install: Some(vec![
                launcher_executable.display().to_string(),
                format!("uplay://install/{}", &id),
            ]),
            launch: Some(vec![
                launcher_executable.display().to_string(),
                format!("uplay://launch/{}/0", &id),
            ]),
            uninstall: Some(vec![
                launcher_executable.display().to_string(),
                format!("uplay://uninstall/{}", &id),
            ]),
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
