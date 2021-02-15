use std::path::{Path, PathBuf};

pub fn fix_path(path: &Path) -> PathBuf {
    return PathBuf::from(
        path.display()
            .to_string()
            .replace("/", &std::path::MAIN_SEPARATOR.to_string()),
    );
}
