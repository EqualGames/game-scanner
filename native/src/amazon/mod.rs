#[cfg(target_os = "windows")]
mod windows;

use crate::scan::types::Game;

#[cfg(target_os = "windows")]
pub fn games() -> std::io::Result<Vec<Game>> {
  return windows::games::list();
}
