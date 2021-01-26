use crate::scan::types::Game;
use crate::origin::mfst;
use crate::util::registry::*;
use crate::util::io::*;

pub fn list() -> std::io::Result<Vec<Game>> {
  let mut items: Vec<Game> = Vec::new();

  let reg = get_local_machine_reg_key("Origin")?;
  let origin_executable: String = reg.get_value("ClientPath")?;

  let manifests_path = String::from("C:\\ProgramData\\Origin\\LocalContent");

  let files = get_files_recursive(&manifests_path, |item| item.contains(".mfst"))?;

  for file in files {
    let game = mfst::read(&file, &origin_executable);

    match game {
      Ok(g) => items.push(g),
      Err(_e) => {}
    }
  }

  return Ok(items);
}
