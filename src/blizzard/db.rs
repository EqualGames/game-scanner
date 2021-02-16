use crate::blizzard::proto::product::ProductInstall;
use crate::blizzard::proto::{product, read_product};
use crate::blizzard::types::BlizzardGames;
use crate::error::{Error, ErrorKind, Result};
use crate::prelude::{Game, GameType};
use crate::util::path::{fix_path_separator, get_filename};
use std::path::{Path, PathBuf};

pub fn read(file: &Path, launcher_executable: &Path) -> Result<Vec<Game>> {
    let manifests = read_product(file)
        .map(|data| data.product_installs)
        .map(|product_installs| {
            product_installs
                .into_iter()
                .filter(get_manifest_predicate)
                .collect::<Vec<product::ProductInstall>>()
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid Blizzard manifest: {}", error.to_string()),
            )
        })
        .unwrap();

    let mut games = Vec::new();

    for manifest in manifests {
        let mut game_path = manifest
            .settings
            .map(|settings| settings.install_path)
            .map_or(PathBuf::new(), PathBuf::from);

        if cfg!(windows) {
            game_path = fix_path_separator(&game_path);
        }

        let launch_code = BlizzardGames::from_uid(&manifest.uid).get_code();

        games.push(Game {
            _type: GameType::Blizzard.to_string(),
            id: String::from(&manifest.uid),
            name: get_filename(&game_path),
            path: game_path.display().to_string(),
            launch_command: vec![
                launcher_executable.display().to_string(),
                format!("--exec=\"launch {}\"", launch_code),
            ],
        })
    }

    return Ok(games);
}

fn get_manifest_predicate(item: &ProductInstall) -> bool {
    item.product_code.ne("agent")
        && item.product_code.ne("bna")
        && item.clone().cached_product_state.map_or(true, |product| {
            product
                .base_product_state
                .map_or(true, |base_state| base_state.playable != false)
        })
}
