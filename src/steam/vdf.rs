use crate::error::{Error, ErrorKind, Result};
use crate::util::string::remove_quotes;
use std::path::{Path, PathBuf};

pub fn read_library_folders(file: &Path) -> Result<Vec<PathBuf>> {
    let library_file = std::fs::read_to_string(&file)
        .map_err(|error| {
            Error::new(
                ErrorKind::LauncherNotFound,
                format!(
                    "Invalid Steam library config, maybe this launcher is not installed: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let library = library_file.split("\n").collect::<Vec<&str>>();

    let mut folders = Vec::new();

    for file_line in library {
        let line: Vec<&str> = file_line
            .split("\t")
            .filter(|str| str.trim().len() != 0)
            .collect();

        if line.len() != 2 {
            continue;
        }

        let attr = remove_quotes(
            line.get(0)
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidLauncher,
                        format!(
                            "Error on read the Steam library config: {}",
                            file.display().to_string(),
                        ),
                    )
                })
                .unwrap(),
        );
        let mut value = remove_quotes(
            line.get(1)
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidLauncher,
                        format!(
                            "Error on read the Steam library config: {}",
                            file.display().to_string()
                        ),
                    )
                })
                .unwrap(),
        );

        match attr.parse::<i32>() {
            Ok(_n) => {
                let double_separator =
                    std::path::MAIN_SEPARATOR.to_string() + &std::path::MAIN_SEPARATOR.to_string();

                value = value.replace(&double_separator, &std::path::MAIN_SEPARATOR.to_string());

                folders.push(PathBuf::from(value))
            }
            Err(_e) => {}
        }
    }

    return Ok(folders);
}
