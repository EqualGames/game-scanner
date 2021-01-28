#[cfg(target_os = "windows")]
mod windows;
mod item;

use crate::scan::types::Game;

#[cfg(target_os = "windows")]
pub fn games() -> std::io::Result<Vec<Game>> {
  return windows::games::list();
}

#[cfg(target_os = "linux")]
pub fn games() -> std::io::Result<Vec<Game>> {
  return Ok(Vec::new());
}

#[cfg(target_os = "macos")]
pub fn games() -> std::io::Result<Vec<Game>> {
  return Ok(Vec::new());
}