#[cfg(not(target_os = "windows"))]
use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
use crate::{
    error::{Error, ErrorKind, Result},
    prelude::Game,
};

#[cfg(target_os = "windows")]
pub use self::windows::*;

mod db;
mod proto;
mod types;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
pub fn executable() -> Result<PathBuf> {
    return Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ));
}

#[cfg(not(target_os = "windows"))]
pub fn games() -> Result<Vec<Game>> {
    return Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ));
}

#[cfg(not(target_os = "windows"))]
pub fn find(_id: &str) -> Result<Game> {
    return Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ));
}
