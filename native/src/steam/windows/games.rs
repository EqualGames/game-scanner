use crate::scan::types::Game;
use crate::steam::acf;
use crate::util::registry::*;
use crate::util::io::*;
use inflector::Inflector;

pub fn list() -> std::io::Result<Vec<Game>> {
  let mut items: Vec<Game> = Vec::new();

  let reg = get_local_machine_reg_key("Valve\\Steam")?;
  let mut steam_path: String = reg.get_value("InstallPath")?;
  steam_path.push_str("\\steamapps");

  let steam_reg = get_current_user_reg_key("Valve\\Steam")?;
  let steam_executable: String = get_steam_executable(&steam_reg.get_value("SteamExe")?);


  let files = get_files(&steam_path, |item| item.contains(".acf") && !item.contains("\\workshop\\"))?;

  for file in files {
    let game = acf::read(&file, &steam_executable);

    match game {
      Ok(g) => items.push(g),
      Err(_e) => {}
    }
  }

  return Ok(items);
}

fn get_steam_executable(path: &String) -> String {
  let words: Vec<&str> = path.split("/").collect();
  let mut transformed: Vec<String> = Vec::new();

  for see in words {
    let mut fmt = see.to_title_case();

    if fmt.eq("C") {
      fmt.push_str(":")
    }

    if fmt.ends_with("X86") {
      fmt = fmt.replace("X86", "(x86)")
    }

    if fmt.eq("Steam Exe") {
      fmt = String::from("steam.exe");
    }

    transformed.push(fmt);
  }

  return transformed.join("\\");
}