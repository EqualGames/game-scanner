use crate::error::Result;
use crate::prelude::{Game, GameCommands, GameState, GameType};
use crate::ubisoft::windows::utils::{get_game_info, get_launcher_executable, get_manifest_ids};

mod utils;

pub fn games() -> Result<Vec<Game>> {
    let launcher_executable = get_launcher_executable().unwrap();
    let manifest_ids = get_manifest_ids().unwrap();

    let mut games = Vec::new();

    for manifest_id in manifest_ids {
        match get_game_info(&manifest_id) {
            Ok((name, path)) => games.push(Game {
                _type: GameType::Ubisoft.to_string(),
                id: manifest_id.clone(),
                name,
                path,
                commands: GameCommands {
                    install: Some(vec![
                        launcher_executable.display().to_string(),
                        format!("uplay://install/{}", &manifest_id),
                    ]),
                    launch: vec![
                        launcher_executable.display().to_string(),
                        format!("uplay://launch/{}/0", &manifest_id),
                    ],
                    uninstall: Some(vec![
                        launcher_executable.display().to_string(),
                        format!("uplay://uninstall/{}", &manifest_id),
                    ]),
                },
                state: GameState {
                    installed: true,
                    needs_update: false,
                    downloading: false,
                    total_bytes: None,
                    received_bytes: None,
                },
            }),
            Err(error) => {
                return Err(error);
            }
        }
    }

    return Ok(games);
}
