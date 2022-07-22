use crate::{
    error::{Error, ErrorKind, Result},
    steam::vdf,
    utils::io::get_files,
};
use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::macos::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub fn get_library_manifests<T>(predicate: T) -> Result<Vec<(PathBuf, Vec<PathBuf>)>>
where
    T: Fn(&PathBuf) -> bool,
{
    let manifests_path = get_manifests_path()?;
    let library_folders = vdf::read_library_folders(&manifests_path.join("libraryfolders.vdf"))?;

    let mut library_paths = Vec::new();

    for folder in library_folders {
        library_paths.push(folder.join("steamapps"));
    }

    if !library_paths.contains(&manifests_path) {
        library_paths.push(manifests_path.clone())
    }

    let mut library_manifests = Vec::new();

    for path in library_paths {
        match get_files(&path, &predicate) {
            Ok(list) => library_manifests.push((path, list)),
            Err(error) => {
                Error::new(
                    ErrorKind::LibraryNotFound,
                    format!(
                        "Invalid Steam library path, something is wrong: {} {}",
                        path.display().to_string(),
                        error.to_string()
                    ),
                )
                .print();
            }
        }
    }

    Ok(library_manifests)
}
