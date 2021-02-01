use std::io;
use std::path::{Path, PathBuf};

use crate::util::string::remove_quotes;

pub fn read_library_folders(file: &Path) -> io::Result<Vec<PathBuf>> {
    let file_content = std::fs::read_to_string(&file).unwrap();
    let file_data = file_content.split("\n").collect::<Vec<&str>>();

    let mut folders = Vec::new();

    for file_line in file_data {
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
