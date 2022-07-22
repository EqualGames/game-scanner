use crate::error::{Error, ErrorKind, Result};
use std::path::PathBuf;

pub fn get_launcher_executable() -> Result<PathBuf> {
    return Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ));
}

pub fn get_manifests_path() -> Result<PathBuf> {
    return Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ));
}
