#[cfg(target_os = "windows")]
mod windows;
mod item;

use crate::scan::types::Game;

pub fn games() -> std::io::Result<Vec<Game>> {
  if cfg!(target_os = "windows") {
    return windows::games::list();
  }

  return Ok(Vec::new());
}
