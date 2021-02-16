use crate::error::{Error, ErrorKind, Result};
use crate::prelude::{Game, GameType};
use std::path::{Path, PathBuf};

#[derive(Default)]
struct Manifest {
    id: String,
    dipinstallpath: String,
    previousstate: String,
}

pub fn read(file: &Path, launcher_executable: &Path) -> Result<Game> {
    let manifest_data = std::fs::read_to_string(&file).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!(
                "Invalid Origin manifest: {} {}",
                file.display().to_string(),
                error.to_string()
            ),
        )
    });

    if manifest_data.is_err() {
        return Err(manifest_data.err().unwrap());
    }

    let manifest = String::from("http://mock/") + &manifest_data.unwrap();

    let manifest_url = url::Url::parse(&manifest).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!(
                "Error on read the Origin manifest: {} {}",
                file.display().to_string(),
                error.to_string()
            ),
        )
    });

    if manifest_url.is_err() {
        return Err(manifest_url.err().unwrap());
    }

    let manifest_url = manifest_url.unwrap();

    let manifest_lines = manifest_url
        .query()
        .map(|data| data.split("&").collect::<Vec<&str>>())
        .ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error on read the Origin manifest: {}",
                    file.display().to_string(),
                ),
            )
        });

    if manifest_lines.is_err() {
        return Err(manifest_lines.err().unwrap());
    }

    let mut manifest = Manifest::default();

    for manifest_line in manifest_lines.unwrap() {
        let pair = manifest_line.split("=").collect::<Vec<&str>>();

        let attr = pair.get(0).unwrap().to_string();
        let value = pair.get(1).unwrap().to_string();

        match attr.as_str() {
            "id" => {
                manifest.id = value;
            }
            "dipinstallpath" => {
                manifest.dipinstallpath =
                    make_dip_install_path(&value).map_or(String::new(), |value| value);
            }
            "previousstate" => {
                manifest.previousstate = value;
            }
            _ => {}
        }
    }

    let name = get_game_name(file).map_or(String::from("Unknown"), |value| value);

    if manifest.previousstate != "kCompleted" {
        return Err(Error::new(
            ErrorKind::IgnoredApp,
            format!("({}) {} is an invalid game", &manifest.id, &name),
        ));
    }

    return Ok(Game {
        _type: GameType::Origin.to_string(),
        id: manifest.id.clone(),
        name: get_game_name(file).unwrap(),
        path: manifest.dipinstallpath,
        launch_command: vec![
            launcher_executable.display().to_string(),
            format!("origin2://game/launch?offerIds={}", &manifest.id),
        ],
    });
}

fn make_dip_install_path(value: &String) -> Option<String> {
    return Option::from(value.clone())
        .map(|path| path.replace("%5c", &std::path::MAIN_SEPARATOR.to_string()))
        .map(|path| path.replace("%3a", ":"))
        .map(|path| path.replace("%20", " "))
        .map(PathBuf::from)
        .map(|path| path.display().to_string());
}

fn get_game_name(file: &Path) -> Option<String> {
    return file
        .parent()
        .and_then(|path| path.file_name())
        .and_then(|path| path.to_str())
        .map(|path| path.to_string());
}
