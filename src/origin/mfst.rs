use crate::prelude::Game;
use crate::util::error::make_io_error;
use std::io;
use std::path::{Path, PathBuf};

struct Manifest {
    id: String,
    dipinstallpath: String,
    previousstate: String,
}

pub fn read(file: &Path, launcher_executable: &Path) -> io::Result<Game> {
    let file_data = std::fs::read_to_string(&file).unwrap();

    let manifest = String::from("http://mock/") + &file_data;

    let manifest_url = url::Url::parse(&manifest)
        .map_err(|_error| make_io_error("error to read the games from the origin launcher"))
        .unwrap();

    let manifest_data = manifest_url
        .query()
        .map(|data| data.split("&").collect::<Vec<&str>>())
        .ok_or_else(|| make_io_error("error to read the game from the origin launcher"))
        .unwrap();

    let mut manifest = Manifest {
        id: String::new(),
        dipinstallpath: String::new(),
        previousstate: String::new(),
    };

    for manifest_line in manifest_data {
        let pair = manifest_line.split("=").collect::<Vec<&str>>();

        let attr = pair.get(0).unwrap().to_string();
        let value = pair.get(1).unwrap().to_string();

        match attr.as_str() {
            "id" => manifest.id = value,
            "dipinstallpath" => manifest.dipinstallpath = make_dip_install_path(&value).unwrap(),
            "previousstate" => manifest.previousstate = value,
            _ => {}
        }
    }

    if manifest.previousstate != "kCompleted" {
        return Err(make_io_error("invalid origin game"));
    }

    let game = Game {
        _type: String::from("origin"),
        id: manifest.id.clone(),
        name: get_game_name(file).unwrap(),
        path: manifest.dipinstallpath,
        launch_command: make_launch_command(&launcher_executable, &manifest.id),
    };

    return Ok(game);
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

fn make_launch_command(launcher_executable: &Path, id: &String) -> Vec<String> {
    let mut command = Vec::new();

    command.push(launcher_executable.display().to_string());
    command.push(format!("origin2://game/launch?offerIds={}", id));

    return command;
}
