use std::path::{Path, PathBuf};

pub fn fix_path_separator(path: &Path) -> PathBuf {
    return PathBuf::from(
        path.display()
            .to_string()
            .replace("/", &std::path::MAIN_SEPARATOR.to_string()),
    );
}

pub fn get_filename(path: &Path) -> String {
    return String::from(path.file_name().unwrap().to_str().unwrap());
}
