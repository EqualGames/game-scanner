use crate::prelude::Game;
use crate::riotgames::yaml;
use crate::util::io::get_files_recursive;
use std::io;
use std::path::PathBuf;

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();
    let launcher_path = PathBuf::from("C:")
        .join(std::path::MAIN_SEPARATOR.to_string())
        .join("ProgramData")
        .join("Riot Games");
    let launcher_manifests_path = launcher_path.join("Metadata");

    let manifests = get_files_recursive(&launcher_manifests_path, get_manifest_predicate).unwrap();

    for manifest in manifests {
        match yaml::read(&manifest, &launcher_path) {
            Ok(g) => games.push(g),
            Err(error) => {
                return Err(io::Error::new(
                    error.kind(),
                    format!(
                        "Error on read the riot games manifest ({}): {}",
                        manifest.display().to_string(),
                        error.to_string()
                    ),
                ))
            }
        }
    }

    return Ok(games);
}

fn get_manifest_predicate(file: &PathBuf) -> bool {
    file.display()
        .to_string()
        .ends_with(".product_settings.yaml")
}
