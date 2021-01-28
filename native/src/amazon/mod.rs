#[cfg(target_os = "windows")]
mod windows;

use crate::scan::types::Game;

pub fn games() -> std::io::Result<Vec<Game>> {
  if cfg!(target_os = "windows") {
    return windows::games::list();
  }


  return Ok(Vec::new());
}
