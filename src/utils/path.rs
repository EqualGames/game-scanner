use std::path::{Path, PathBuf, MAIN_SEPARATOR, MAIN_SEPARATOR_STR};

pub fn fix_path_separator(path: &Path) -> PathBuf {
    PathBuf::from(path.display().to_string().replace('/', MAIN_SEPARATOR_STR))
}

pub fn get_filename(path: &Path) -> String {
    String::from(
        path.display()
            .to_string()
            .split(MAIN_SEPARATOR)
            .last()
            .unwrap_or(""),
    )
}
