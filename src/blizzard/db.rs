use crate::blizzard::proto::product::ProductInstall;
use crate::blizzard::proto::{product, read_product};
use crate::blizzard::types::BlizzardProducts;
use crate::error::Result;
use crate::prelude::Game;
use crate::util::path::{fix_path_separator, get_filename};
use std::path::{Path, PathBuf};

pub fn read(file: &Path, launcher_executable: &Path) -> Result<Vec<Game>> {
    let mut games = Vec::new();

    let product_installs = read_product(file)
        .map(|data| data.product_installs)
        .map(|product_installs| {
            product_installs
                .into_iter()
                .filter(get_manifest_predicate)
                .collect::<Vec<product::ProductInstall>>()
        })
        .unwrap();

    for product in product_installs {
        let path = product
            .settings
            .map(|settings| settings.install_path)
            .map_or(PathBuf::new(), PathBuf::from);

        let launch_code = BlizzardProducts::from_uid(&product.uid).get_code();

        games.push(Game {
            _type: String::from("blizzard"),
            id: String::from(&product.uid),
            name: get_filename(&path),
            path: fix_path_separator(&path).display().to_string(),
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
