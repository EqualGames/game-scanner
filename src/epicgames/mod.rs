mod windows;
mod item;

use crate::scan::types::Game;
use std::env;

pub fn games() -> std::io::Result<Vec<Game>> {
  if env::consts::OS == "windows" {
    return windows::games::list();
  }

  return Ok(Vec::new());
}
