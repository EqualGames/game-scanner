use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
    steam::types::{SteamAppState, SteamUpdateResult},
    utils::string::remove_quotes,
};
use std::path::{Path, PathBuf};

pub fn read(file: &Path, launcher_executable: &Path, library_path: &Path) -> Result<Game> {
    let manifest_data = std::fs::read_to_string(&file).map_err(|error| {
        Error::new(
            ErrorKind::InvalidManifest,
            format!(
                "Error on read the Steam manifest: {} {}",
                file.display().to_string(),
                error.to_string()
            ),
        )
    })?;

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
                let state = value.parse::<i64>().unwrap();

                game.state.installed = has_app_state(state, SteamAppState::FullyInstalled);

                game.state.needs_update = has_app_state(state, SteamAppState::UpdateRequired);

                game.state.downloading = has_app_state(state, SteamAppState::Downloading);
            }
            "UpdateResult" => {
                let state = value.parse::<i64>().unwrap();

                if game.state.needs_update && !game.state.downloading {
                    game.state.downloading =
                        has_update_result(state, SteamUpdateResult::Downloading);
                }
            }
            "BytesToDownload" => game.state.total_bytes = value.parse::<u64>().ok(),
            "BytesDownloaded" => game.state.received_bytes = value.parse::<u64>().ok(),
            "installdir" => {
                game.path = Some(PathBuf::from(library_path).join("common").join(value))
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

    game.commands.launch = Some(vec![
        launcher_executable.display().to_string(),
        String::from("-silent"),
        format!("steam://run/{}", &game.id),
    ]);

    game.commands.uninstall = Some(vec![
        launcher_executable.display().to_string(),
        String::from("-silent"),
        format!("steam://uninstall/{}", &game.id),
    ]);

    return Ok(game);
}

fn has_app_state(state: i64, flag: SteamAppState) -> bool {
    if flag == SteamAppState::Invalid {
        return state == 0;
    }

    if flag == SteamAppState::Uninstalled {
        return state == 1;
    }

    (state & flag.get_code()) == flag.get_code()
}

fn has_update_result(state: i64, flag: SteamUpdateResult) -> bool {
    if flag == SteamUpdateResult::Downloading {
        return state == 0;
    }

    (state & flag.get_code()) == flag.get_code()
}
