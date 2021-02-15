use crate::blizzard::proto::{product, read_product};
use crate::blizzard::types::BlizzardProducts;
use crate::prelude::Game;
use std::path::{Path, PathBuf};

pub fn read(file: &Path, launcher_executable: &Path) -> std::io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let product_installs = read_product(file).map(|data| {
        data.product_installs
            .into_iter()
            .filter(|item| {
                item.product_code.ne("agent")
                    && item.product_code.ne("bna")
                    && item.clone().cached_product_state.map_or(true, |product| {
                        product
                            .base_product_state
                            .map_or(true, |base_state| base_state.playable != false)
                    })
            })
            .collect::<Vec<product::ProductInstall>>()
    });

    if product_installs.is_err() {
        return Ok(games);
    }

    for product in product_installs.unwrap() {
        let path = product.settings.map_or(PathBuf::new(), |settings| {
            PathBuf::from(settings.install_path)
        });

        let launch_code = BlizzardProducts::from_uid(&product.uid).get_code();

        games.push(Game {
            _type: String::from("blizzard"),
            id: String::from(&product.uid),
            name: String::from(path.file_name().unwrap().to_str().unwrap()),
            path: path.display().to_string(),
            launch_command: vec![
                launcher_executable.display().to_string(),
                format!("--exec=\"launch {}\"", launch_code),
            ],
        })
    }

    return Ok(games);
}
