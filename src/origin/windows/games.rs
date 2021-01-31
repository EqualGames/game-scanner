use std::io;
use std::path::PathBuf;

use crate::origin::mfst;
use crate::prelude::Game;
use crate::util::io::*;
use crate::util::registry::*;

pub fn list() -> io::Result<Vec<Game>> {
    let mut games = Vec::new();

    let origin_reg = get_local_machine_reg_key("Origin").unwrap();

    let origin_executable =
        PathBuf::from(origin_reg.get_value::<String, &str>("ClientPath").unwrap());
    let manifests_path = PathBuf::from("C:")
        .join(std::path::MAIN_SEPARATOR.to_string())
        .join("ProgramData")
        .join("Origin")
        .join("LocalContent");

    let files =
        get_files_recursive(&manifests_path, |file| file.extension().unwrap().eq("mfst")).unwrap();

    for file in files {
        let game = mfst::read(&file, &origin_executable);

        match game {
            Ok(g) => games.push(g),
            Err(_e) => {}
        }
    }

    return Ok(games);
}
