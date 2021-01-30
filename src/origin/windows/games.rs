use crate::types::Game;
use crate::origin::mfst;
use crate::util::registry::*;
use crate::util::io::*;
use std::path::Path;

pub fn list() -> std::io::Result<Vec<Game>> {
    let mut items = Vec::new();

    let origin_reg = get_local_machine_reg_key("Origin")?;
    let origin_client_path: String = origin_reg.get_value("ClientPath")?;
    let origin_executable = Path::new(&origin_client_path);

    let manifests_path = Path::new("C:").join("ProgramData").join("Origin").join("LocalContent");

    let files = get_files_recursive(&manifests_path, |item| item.extension().unwrap().eq("mfst"))?;

    for file in files {
        let game = mfst::read(&file, origin_executable);

        match game {
            Ok(g) => items.push(g),
            Err(_e) => {}
        }
    }

    return Ok(items);
}
