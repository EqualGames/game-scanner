use std::path::PathBuf;

use crate::error::{Error, ErrorKind, Result};

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
