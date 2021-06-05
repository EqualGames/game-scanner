use std::path::{Path, PathBuf};

use crate::{
    error::{Error, ErrorKind, Result},
    utils::string::remove_quotes,
};

pub fn read_library_folders(file: &Path) -> Result<Vec<PathBuf>> {
    let library_data = std::fs::read_to_string(&file);

    if library_data.is_err() {
        return Err(Error::new(
            ErrorKind::InvalidLibrary,
            format!(
                "Invalid Steam library config, maybe this launcher is not installed: {}",
                library_data.err().unwrap().to_string()
            ),
        ));
    }

    let library_data = library_data.unwrap();

    let library = library_data.split("\n").collect::<Vec<&str>>();

    let mut folders = Vec::new();

    for file_line in library {
        let line: Vec<&str> = file_line
            .split("\t")
            .filter(|str| str.trim().len() != 0)
            .collect();

        if line.len() != 2 {
            continue;
        }

        let attr = remove_quotes(line.get(0).unwrap());
        let mut value = remove_quotes(line.get(1).unwrap());

        match attr.parse::<i32>() {
            Ok(_n) => {
                if cfg!(target_os = "windows") {
                    let double_separator = std::path::MAIN_SEPARATOR.to_string()
                        + &std::path::MAIN_SEPARATOR.to_string();

                    value =
                        value.replace(&double_separator, &std::path::MAIN_SEPARATOR.to_string());
                }

                folders.push(PathBuf::from(value))
            }
            Err(_e) => {}
        }
    }

    return Ok(folders);
}
