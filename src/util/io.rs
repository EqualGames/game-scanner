use std::fs;
use std::path::{Path, PathBuf};

pub fn get_files(
    path: &Path,
    predicate: fn(item: &PathBuf) -> bool,
) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();

        if entry_path.is_file() {
            if predicate(&entry_path) {
                files.push(entry_path);
            }
        }
    }

    return Ok(files);
}

pub fn get_files_recursive(
    path: &Path,
    predicate: fn(item: &PathBuf) -> bool,
) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();

        if entry_path.is_file() {
            if predicate(&entry_path) {
                files.push(entry_path);
            }
        } else if entry_path.is_dir() {
            files.extend(get_files(&entry_path, predicate)?);
        }
    }

    return Ok(files);
}
