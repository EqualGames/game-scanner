use self::super::yaml::read_riot_client_installs;
use crate::error::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_client_installs =
        get_launcher_path().map(|path| path.join("RiotClientInstalls.json"))?;

    return read_riot_client_installs(&launcher_client_installs)
        .map(|data| data.rc_default)
        .map(PathBuf::from);
}

pub fn get_launcher_path() -> Result<PathBuf> {
    let launcher_path = PathBuf::from("/")
        .join("Users")
        .join("Shared")
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
