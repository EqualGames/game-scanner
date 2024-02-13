use std::path::{Path, PathBuf, MAIN_SEPARATOR, MAIN_SEPARATOR_STR};

use crate::{
    error::{Error, ErrorKind, Result},
    utils::string::remove_quotes,
};

pub fn read_library_folders(file: &Path) -> Result<Vec<PathBuf>> {
    let library_folders_data = std::fs::read_to_string(file).map_err(|error| {
        Error::new(
            ErrorKind::InvalidLibrary,
            format!("Invalid Steam library config, maybe this launcher is not installed: {error}"),
        )
    })?;

    let library_folders = library_folders_data.split('\n').collect::<Vec<&str>>();

    let mut folders = Vec::new();

    for file_line in library_folders {
        let line: Vec<&str> = file_line
            .split('\t')
            .filter(|str| !str.trim().is_empty())
            .collect();

        if line.len() != 2 {
            continue;
        }

        let attr = remove_quotes(line.first().unwrap());
        let mut value = remove_quotes(line.get(1).unwrap());

        if attr.as_str() == "path" {
            if cfg!(target_os = "windows") {
                let double_separator = MAIN_SEPARATOR.to_string() + MAIN_SEPARATOR_STR;

                value = value.replace(&double_separator, MAIN_SEPARATOR_STR);
            }

            folders.push(PathBuf::from(value));
        }
    }

    Ok(folders)
}
