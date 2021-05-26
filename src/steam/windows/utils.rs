use std::path::PathBuf;

use case::CaseExt;

use crate::{
    error::{Error, ErrorKind, Result},
    steam::vdf,
    util::{io::get_files, registry},
};

pub fn get_library_manifests<T>(predicate: T) -> Result<Vec<(PathBuf, Vec<PathBuf>)>>
where
    T: Fn(&PathBuf) -> bool,
{
    let manifests_path = get_manifests_path().unwrap();

    let mut library_paths = Vec::new();
    library_paths.push(manifests_path.clone());

    let library_folders =
        vdf::read_library_folders(&manifests_path.join("libraryfolders.vdf")).unwrap();

    for folder in library_folders {
        library_paths.push(folder.join("steamapps"));
    }

    let mut library_manifests = Vec::new();

    for path in library_paths {
        match get_files(&path, &predicate) {
            Ok(list) => library_manifests.push((path, list)),
            Err(error) => {
                Error::new(
                    ErrorKind::LibraryNotFound,
                    format!(
                        "Invalid Steam library path, maybe this launcher is not installed: {} {}",
                        manifests_path.display().to_string(),
                        error.to_string()
                    ),
                )
                .print();
            }
        }
    }

    Ok(library_manifests)
}

pub fn get_launcher_executable() -> Result<PathBuf> {
    let launcher_executable = registry::get_current_user_reg_key("Valve\\Steam")
        .and_then(|user_launcher_reg| registry::get_value(&user_launcher_reg, "SteamExe"))
        .map(PathBuf::from)
        .map(fix_launcher_executable_path)
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Steam path, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    if !launcher_executable.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                launcher_executable.display().to_string()
            ),
        ));
    }

    return Ok(launcher_executable);
}

pub fn get_manifests_path() -> Result<PathBuf> {
    let manifests_path = registry::get_local_machine_reg_key("Valve\\Steam")
        .and_then(|launcher_reg| registry::get_value(&launcher_reg, "InstallPath"))
        .map(PathBuf::from)
        .map(|path| path.join("steamapps"));

    if manifests_path.is_err() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                manifests_path.err().unwrap().to_string()
            ),
        ));
    }

    let manifests_path = manifests_path.unwrap();

    if !manifests_path.exists() {
        return Err(Error::new(
            ErrorKind::LauncherNotFound,
            format!(
                "Invalid Steam path, maybe this launcher is not installed: {}",
                manifests_path.display().to_string()
            ),
        ));
    }

    return Ok(manifests_path);
}

pub fn fix_launcher_executable_path(path: PathBuf) -> PathBuf {
    let path_string = path.display().to_string();
    let words = path_string.split("/").collect::<Vec<_>>();
    let mut result_path = PathBuf::new();

    for word in words {
        let mut new_word = String::from(word);

        if new_word.contains(":") {
            new_word = new_word.to_camel();
            new_word.push_str(&std::path::MAIN_SEPARATOR.to_string())
        } else if new_word.contains("x86") {
            new_word = new_word
                .split(" ")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|value| {
                    if !value.contains("86") {
                        value.to_camel()
                    } else {
                        String::from(value)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
        } else if !new_word.contains(".exe") {
            new_word = new_word.to_camel();
        }

        result_path = result_path.join(new_word);
    }

    return result_path;
}
