#[cfg(target_os = "windows")]
mod windows;
mod mfst;

use crate::scan::types::Game;
use std::env;

pub fn games() -> std::io::Result<Vec<Game>> {
  if cfg!(target_os = "windows") {
    return windows::games::list();
  }

  return Ok(Vec::new());
}
