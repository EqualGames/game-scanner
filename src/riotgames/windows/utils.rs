use std::path::{Path, PathBuf};

use crate::error::{Error, ErrorKind, Result};

pub fn get_manifest_predicate(file: &PathBuf) -> bool {
    file.display()
        .to_string()
        .ends_with(".product_settings.yaml")
}

pub fn get_launcher_path() -> Result<PathBuf> {
    let launcher_path = PathBuf::from("C:")
        .join(std::path::MAIN_SEPARATOR.to_string())
        .join("ProgramData")
        .join("Riot Games");

    if !launcher_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Riot Games path, maybe this launcher is not installed: {}",
                launcher_path.display().to_string()
            ),
        ));
    }

    return Ok(launcher_path);
}

pub fn get_manifests_path(launcher_path: &Path) -> Result<PathBuf> {
    let manifests_path = launcher_path.join("Metadata");

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Riot Games path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}
