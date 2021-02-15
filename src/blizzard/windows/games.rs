use crate::blizzard::db;
use crate::prelude::Game;
use crate::util::registry::get_local_machine_reg_key;
use std::io;
use std::path::{Path, PathBuf};

pub fn list() -> io::Result<Vec<Game>> {
    let launcher_path =
        get_local_machine_reg_key("Microsoft\\Windows\\CurrentVersion\\Uninstall\\Battle.net")
            .and_then(|launcher_reg| launcher_reg.get_value::<String, &str>("DisplayIcon"))
            .map(PathBuf::from);

    if launcher_path.is_err() {
        return Ok(Vec::new());
    }

    let launcher_executable = launcher_path.unwrap();

    return db::read(
        Path::new("C:\\ProgramData\\Battle.net\\Agent\\product.db"),
        &launcher_executable,
    );
}
