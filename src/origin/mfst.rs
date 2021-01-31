use std::io;
use std::path::{Path, PathBuf};

use crate::prelude::Game;

struct Manifest {
    id: String,
    dipinstallpath: String,
    previousstate: String,
}

pub fn read(file: &Path, launcher_path: &Path) -> io::Result<Game> {
    let file_data = std::fs::read_to_string(&file).unwrap();

    let mut manifest = String::from("http://mock/");
    manifest.push_str(&file_data);

    let manifest_url = url::Url::parse(&manifest)
        .map_err(|_error| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "error to read the games from the origin launcher",
            )
        })
        .unwrap();

    let query_pairs = manifest_url
        .query()
        .map(|data| data.split("&").collect::<Vec<&str>>())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "error to read the game from the origin launcher",
            )
        })
        .unwrap();

    let mut manifest = Manifest {
        id: String::new(),
        dipinstallpath: String::new(),
        previousstate: String::new(),
    };

    for query_pair in query_pairs {
        let pair = query_pair.split("=").collect::<Vec<&str>>();

        let attr = pair.get(0).unwrap().to_string();
        let value = pair.get(1).unwrap().to_string();

        match attr.as_str() {
            "id" => manifest.id = value,
            "dipinstallpath" => {
                let mut dip_install_path = value.clone();
                dip_install_path =
                    dip_install_path.replace("%5c", &std::path::MAIN_SEPARATOR.to_string());
                dip_install_path = dip_install_path.replace("%3a", ":");
                dip_install_path = dip_install_path.replace("%20", " ");

                manifest.dipinstallpath = PathBuf::from(dip_install_path)
                    .to_str()
                    .unwrap()
                    .to_string();
            }
            "previousstate" => manifest.previousstate = value,
            _ => {}
        }
    }

    if manifest.previousstate != "kCompleted" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "invalid origin game",
        ));
    }

    let mut launch_command: String = String::from(launcher_path.to_str().unwrap());
    launch_command.push_str(" origin2://game/launch?offerIds=");
    launch_command.push_str(&manifest.id);

    let name = file
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let game = Game {
        _type: "origin".to_string(),
        id: manifest.id,
        name,
        path: manifest.dipinstallpath,
        launch_command,
    };

    return Ok(game);
}
