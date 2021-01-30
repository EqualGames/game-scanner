use crate::types::Game;
use crate::util::registry::*;

pub fn list() -> std::io::Result<Vec<Game>> {
  let mut items = Vec::new();
  let launcher_reg = get_local_machine_reg_key("Ubisoft\\Launcher")?;
  let installs_reg = launcher_reg.open_subkey("Installs")?;

  let mut ubisoft_executable: String = launcher_reg.get_value("InstallDir")?;
  ubisoft_executable.push_str("upc.exe");

  for identifier in installs_reg.enum_keys().map(|x| x.unwrap()) {
    let mut game_reg_path = String::from("Microsoft\\Windows\\CurrentVersion\\Uninstall\\Uplay Install ");
    game_reg_path.push_str(&identifier);
    let game_reg = get_local_machine_reg_key(&game_reg_path)?;

    let mut command = ubisoft_executable.clone();
    command.push_str(" uplay://launch/");
    command.push_str(&identifier);

    let mut install_location: String = game_reg.get_value("InstallLocation")?;
    install_location = install_location.replace("/", "\\");

    let game = Game {
      _type: "ubisoft".to_string(),
      id: String::from(&identifier),
      name: game_reg.get_value("DisplayName")?,
      path: install_location,
      launch_command: command,
    };

    items.push(game)
  }

  return Ok(items);
}
