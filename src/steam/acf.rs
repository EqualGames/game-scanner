use std::io;
use std::path::Path;

use crate::prelude::Game;
use crate::util::string::remove_quotes;

pub fn read(file: &Path, launcher_path: &Path) -> io::Result<Game> {
    let file_content = std::fs::read_to_string(&file).unwrap();
    let file_data = file_content.split("\n").collect::<Vec<&str>>();

    let mut game = Game {
        _type: "steam".to_string(),
        id: String::new(),
        name: String::new(),
        path: String::new(),
        launch_command: String::new(),
    };

    for file_line in file_data {
        let line = file_line
            .split("\t")
            .filter(|str| !str.trim().is_empty())
            .collect::<Vec<&str>>();

        if line.len() != 2 {
            continue;
        }

        let attr = remove_quotes(line.get(0).unwrap());
        let value = remove_quotes(line.get(1).unwrap());

        match attr.as_str() {
            "appid" => game.id = value,
            "name" => game.name = value,
            "installdir" => game.path = value,
            _ => {}
        }
    }

    if game.id == "228980" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "invalid steam game",
        ));
    }

    let mut command: String = launcher_path.to_str().unwrap().to_string();
    command.push_str(" -silent steam://run/");
    command.push_str(&game.id);

    game.launch_command = command;

    return Ok(game);
}
