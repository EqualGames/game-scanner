use std::path::{Path, PathBuf};

pub fn fix_path_separator(path: &Path) -> PathBuf {
    return PathBuf::from(
        path.display()
            .to_string()
            .replace("/", &std::path::MAIN_SEPARATOR.to_string()),
    );
}

pub fn get_filename(path: &Path) -> String {
    return String::from(
        path.display()
            .to_string()
            .split(std::path::MAIN_SEPARATOR)
            .last()
            .unwrap_or(""),
    );
}
