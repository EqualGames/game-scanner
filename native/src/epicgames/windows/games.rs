use crate::scan::types::Game;
use crate::epicgames::item;
use crate::util::registry::*;
use crate::util::io::*;

pub fn list() -> std::io::Result<Vec<Game>> {
  let mut items: Vec<Game> = Vec::new();

  let reg = get_local_machine_reg_key("Epic Games\\EpicGamesLauncher")?;
  let mut epicgames_path: String = reg.get_value("AppDataPath")?;
  epicgames_path.push_str("\\Manifests");

  let eos_reg = get_current_user_reg_key("Epic Games\\EOS")?;
  let mut epicgames_executable: String = eos_reg.get_value("ModSdkCommand")?;
  epicgames_executable = epicgames_executable.replace("/", "\\");

  let files = get_files(&epicgames_path, |item| item.contains(".item") && !item.contains("\\Pending\\"))?;

  for file in files {
    let game = item::read(&file, &epicgames_executable);

    match game {
      Ok(g) => items.push(g),
      Err(_e) => {}
    }
  }

  return Ok(items);
}
