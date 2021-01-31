use std::io;
use std::path::Path;

use crate::prelude::Game;

struct Manifest {
    id: String,
    dipinstallpath: String,
    previousstate: String,
}

pub fn read(file: &Path, launcher_path: &Path) -> io::Result<Game> {
    let file_data = std::fs::read_to_string(&file)?;

    let mut manifest = String::from("http://mock/");
    manifest.push_str(&file_data);

    let manifest_url = url::Url::parse(&manifest)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let query_data = manifest_url
        .query()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Invalid url query"))?;
    let query_pairs: Vec<&str> = query_data.split("&").collect();

    let mut manifest = Manifest {
        id: String::new(),
        dipinstallpath: String::new(),
        previousstate: String::new(),
    };

    for query_pair in query_pairs {
        let pair: Vec<&str> = query_pair.split("=").collect();

        let attr = pair.get(0).unwrap().to_string();
        let value = pair.get(1).unwrap().to_string();

        match attr.as_str() {
            "id" => manifest.id = value,
            "dipinstallpath" => {
                let mut dipinstallpath = value.clone();
                dipinstallpath = dipinstallpath.replace("%5c", "\\");
                dipinstallpath = dipinstallpath.replace("%3a", ":");
                dipinstallpath = dipinstallpath.replace("%20", " ");

                manifest.dipinstallpath = dipinstallpath;
            }
            "previousstate" => manifest.previousstate = value,
            _ => {}
        }
    }

    if manifest.previousstate != "kCompleted" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "invalid game",
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
