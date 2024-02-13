use std::path::{Path, PathBuf};

use self::super::yaml::read_riot_client_installs;
use crate::{
    error::{Error, ErrorKind, Result},
    utils::path::fix_path_separator,
};

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_client_installs = get_launcher_path()?.join("RiotClientInstalls.json");

    read_riot_client_installs(&launcher_client_installs)
        .map(|data| data.rc_default)
        .map(PathBuf::from)
        .map(|path| fix_path_separator(&path))
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
                launcher_path.display()
            ),
        ));
    }

    Ok(launcher_path)
}

pub fn get_manifests_path(launcher_path: &Path) -> Result<PathBuf> {
    let manifests_path = launcher_path.join("Metadata");

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Riot Games path, maybe this launcher is not installed: {}",
                manifests_path.display()
            ),
        ));
    }

    Ok(manifests_path)
}
