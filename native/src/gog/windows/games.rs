use crate::scan::types::Game;
use crate::util::registry::*;

pub fn list() -> std::io::Result<Vec<Game>> {
  let mut items: Vec<Game> = Vec::new();

  let reg = get_local_machine_reg_key("GOG.com\\GalaxyClient\\paths")?;
  let gog_path: String = reg.get_value("client")?;

  let gog_reg = get_local_machine_reg_key("GOG.com\\GalaxyClient")?;
  let gog_filename: String = gog_reg.get_value("clientExecutable")?;
  let mut gog_executable: String = gog_path.clone();
  gog_executable.push_str("\\");
  gog_executable.push_str(&gog_filename);

  let gog_games_reg = get_local_machine_reg_key("GOG.com\\Games")?;

  for identifier in gog_games_reg.enum_keys().map(|x| x.unwrap()) {
    let game_reg = gog_games_reg.open_subkey(&identifier)?;

    let game_path: String = game_reg.get_value("path")?;
    let mut launch_command = gog_executable.clone();
    launch_command.push_str(" /command=runGame /gameId=");
    launch_command.push_str(&identifier);
    launch_command.push_str(" path=");
    launch_command.push_str(&game_path);

    let game = Game {
      _type: "gog".to_string(),
      id: identifier,
      name: game_reg.get_value("gameName")?,
      path: game_path,
      launch_command,
    };

    items.push(game);
  }

  return Ok(items);
}
