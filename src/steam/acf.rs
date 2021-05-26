use std::path::{Path, PathBuf};

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    steam::types::SteamAppState,
    util::string::remove_quotes,
};

pub fn read(file: &Path, launcher_executable: &Path, library_path: &Path) -> Result<Game> {
    let manifest_data = std::fs::read_to_string(&file)
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error on read the Steam manifest: {} {}",
                    file.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let manifest = manifest_data.split("\n").collect::<Vec<&str>>();

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

        let attr = remove_quotes(line.get(0).unwrap());
        let value = remove_quotes(line.get(1).unwrap());

        match attr.as_str() {
            "appid" => game.id = value,
            "name" => game.name = value,
            "StateFlags" => {
                let state = value.parse::<i32>().unwrap();

                game.state.installed = has_app_state(state, SteamAppState::FullyInstalled);

                game.state.needs_update = has_app_state(state, SteamAppState::UpdateRequired)
                    || has_app_state(state, SteamAppState::UpdateStarted);

                game.state.downloading = has_app_state(state, SteamAppState::PreAllocating)
                    || has_app_state(state, SteamAppState::Downloading)
                    || has_app_state(state, SteamAppState::UpdateRunning);
            }
            "BytesToDownload" => game.state.total_bytes = value.parse::<i32>().ok(),
            "BytesDownloaded" => game.state.received_bytes = value.parse::<i32>().ok(),
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

    game.commands.install = Some(vec![
        launcher_executable.display().to_string(),
        String::from("-silent"),
        format!("steam://install/{}", &game.id),
    ]);

    game.commands.launch = vec![
        launcher_executable.display().to_string(),
        String::from("-silent"),
        format!("steam://run/{}", &game.id),
    ];

    game.commands.uninstall = Some(vec![
        launcher_executable.display().to_string(),
        String::from("-silent"),
        format!("steam://uninstall/{}", &game.id),
    ]);

    return Ok(game);
}

fn has_app_state(state: i32, flag: SteamAppState) -> bool {
    (state & flag.get_code()) == state
}
