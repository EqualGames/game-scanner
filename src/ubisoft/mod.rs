#[cfg(target_os = "windows")]
pub use self::windows::*;
use crate::error::{Error, ErrorKind};
use crate::prelude::Game;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
pub fn games() -> Result<Vec<Game>, Error> {
    Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ))
}

#[cfg(not(target_os = "windows"))]
pub fn find(_id: &str) -> Result<Game, Error> {
    Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ))
}
