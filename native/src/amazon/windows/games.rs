use crate::scan::types::Game;
use rusqlite::{params, Connection, OpenFlags};
use directories::BaseDirs;

pub fn list() -> std::io::Result<Vec<Game>> {
  let base_dirs = BaseDirs::new().unwrap();

  let mut amazon_path = base_dirs.data_local_dir().to_str().unwrap().to_string();
  amazon_path.push_str("\\Amazon Games");

  let mut db_path = amazon_path.clone();
  db_path.push_str("\\Data\\Games\\Sql\\GameInstallInfo.sqlite");

  let conn = Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

  let mut stmt = conn.prepare("SELECT id, ProductTitle, InstallDirectory FROM DbSet WHERE Installed = 1;")
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

  let game_itr = stmt.query_map(params![], |row| {
    let id: String = row.get(0)?;
    let mut launch_command = amazon_path.clone();
    launch_command.push_str("\\App\\Amazon Games.exe");
    launch_command.push_str(" amazon-games://play/");
    launch_command.push_str(&id);

    return Ok(Game {
      _type: "amazon".to_string(),
      id,
      name: row.get(1)?,
      path: row.get(2)?,
      launch_command,
    });
  }).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

  let mut items: Vec<Game> = Vec::new();

  for game in game_itr {
    items.push(game.unwrap());
  }

  return Ok(items);
}
