use std::io;
use std::path::PathBuf;

use crate::origin::mfst;
use crate::prelude::Game;
use crate::util::io::*;
use crate::util::registry::*;

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let launcher_info = get_local_machine_reg_key("Origin")
        .and_then(|launcher_reg| launcher_reg.get_value::<String, &str>("ClientPath"))
        .map(PathBuf::from);

    if launcher_info.is_err() {
        return Ok(games);
    }
    let launcher_executable = launcher_info.unwrap();

    let manifests_path = PathBuf::from("C:")
        .join(std::path::MAIN_SEPARATOR.to_string())
        .join("ProgramData")
        .join("Origin")
        .join("LocalContent");

    if !launcher_executable.exists() || !manifests_path.exists() {
        return Ok(games);
    }

    let manifests = get_files_recursive(&manifests_path, get_manifest_predicate).unwrap();

    for manifest in manifests {
        let game = mfst::read(&manifest, &launcher_executable);

        match game {
            Ok(g) => games.push(g),
            Err(_e) => {}
        }
    }

    return Ok(games);
}

fn get_manifest_predicate(file: &PathBuf) -> bool {
    return file.extension().unwrap().eq("mfst");
}
