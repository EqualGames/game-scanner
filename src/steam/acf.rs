use crate::prelude::Game;
use crate::util::string::remove_quotes;
use std::io;
use std::path::{Path, PathBuf};

pub fn read(file: &Path, launcher_executable: &Path, library_path: &Path) -> io::Result<Game> {
    let file_content = std::fs::read_to_string(&file).unwrap();
    let file_data = file_content.split("\n").collect::<Vec<&str>>();

    let mut game = Game::default();
    game._type = String::from("steam");

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
            "installdir" => {
                game.path = PathBuf::from(library_path)
                    .join("common")
                    .join(value)
                    .display()
                    .to_string()
            }
            _ => {}
        }
    }

    if game.id == "228980" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid steam game: ({}){}", game.id, game.name).as_str(),
        ));
    }

    game.launch_command = vec![
        launcher_executable.display().to_string(),
        String::from("-silent"),
        format!("steam://run/{}", &game.id),
    ];

    return Ok(game);
}
