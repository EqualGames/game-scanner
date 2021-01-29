use std::path::{Path, PathBuf};

pub fn read_library_folders(file: &Path) -> std::io::Result<Vec<PathBuf>> {
    let file_data = std::fs::read_to_string(&file)?;
    let file_lines: Vec<&str> = file_data.split("\n").collect();

    let mut folders: Vec<PathBuf> = Vec::new();

    for file_line in file_lines {
        let line: Vec<&str> = file_line.split("\t").filter(|str| str.trim().len() != 0).collect();

        if line.len() != 2 {
            continue;
        }

        let attr = remove_quotes(line.get(0).unwrap());
        let mut value = remove_quotes(line.get(1).unwrap());

        match attr.parse::<i32>() {
            Ok(_n) => {
                let mut double_separator = std::path::MAIN_SEPARATOR.to_string();
                double_separator.push_str(&std::path::MAIN_SEPARATOR.to_string());

                value = value.replace(&double_separator, std::path::MAIN_SEPARATOR.to_string().as_str());

                folders.push(PathBuf::from(value))
            }
            Err(_e) => {}
        }
    }

    return Ok(folders);
}


fn remove_quotes(text: &str) -> String {
    return String::from(text.to_string()).replace("\"",
                                                  "");
}
