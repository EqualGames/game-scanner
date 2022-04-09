use std::path::{Path, PathBuf};

use crate::error::{Error, ErrorKind, Result};

pub fn get_launcher_executable() -> Result<PathBuf> {
    return Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ));
}

pub fn get_launcher_path() -> Result<PathBuf> {
    return Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ));
}

pub fn get_manifests_path(launcher_path: &Path) -> Result<PathBuf> {
    return Err(Error::new(
        ErrorKind::InvalidLauncher,
        "launcher not supported",
    ));
}
