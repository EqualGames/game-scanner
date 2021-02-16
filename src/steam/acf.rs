use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use crate::util::string::remove_quotes;
use std::path::{Path, PathBuf};

pub fn read(file: &Path, launcher_executable: &Path, library_path: &Path) -> Result<Game> {
    let manifest_file = std::fs::read_to_string(&file)
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Error on read the Steam manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let manifest = manifest_file.split("\n").collect::<Vec<&str>>();

    let mut game = Game::default();
    game._type = String::from("steam");

    for file_line in manifest {
        let line = file_line
            .split("\t")
            .filter(|str| !str.trim().is_empty())
            .collect::<Vec<&str>>();

        if line.len() != 2 {
            continue;
        }

        let attr = remove_quotes(
            line.get(0)
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidLauncher,
                        format!(
                            "Error on read the Steam manifest: {}",
                            file.display().to_string()
                        ),
                    )
                })
                .unwrap(),
        );
        let value = remove_quotes(
            line.get(1)
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidLauncher,
                        format!(
                            "Error on read the Steam manifest: {}",
                            file.display().to_string(),
                        ),
                    )
                })
                .unwrap(),
        );

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
        return Err(Error::new(
            ErrorKind::IgnoredApp,
            format!("({}) {} is an invalid game", &game.id, &game.name),
        ));
    }

    game.launch_command = vec![
        launcher_executable.display().to_string(),
        String::from("-silent"),
        format!("steam://run/{}", &game.id),
    ];

    return Ok(game);
}
