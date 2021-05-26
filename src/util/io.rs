use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, ErrorKind, Result};

pub fn get_files<T>(path: &Path, predicate: T) -> Result<Vec<PathBuf>>
where
    T: Fn(&PathBuf) -> bool,
{
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::IO,
            format!("invalid path {}", path.display().to_string()).as_str(),
        ));
    }

    let mut files = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry_path = entry.unwrap().path();

        if entry_path.is_file() {
            if predicate(&entry_path) {
                files.push(entry_path);
            }
        }
    }

    return Ok(files);
}

pub fn get_files_recursive<T>(path: &Path, predicate: T) -> Result<Vec<PathBuf>>
where
    T: Fn(&PathBuf) -> bool,
{
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::IO,
            format!("invalid path {}", path.display().to_string()).as_str(),
        ));
    }

    let mut files = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry_path = entry.unwrap().path();

        if entry_path.is_file() {
            if predicate(&entry_path) {
                files.push(entry_path);
            }
        } else if entry_path.is_dir() {
            files.extend(get_files(&entry_path, &predicate).unwrap());
        }
    }

    return Ok(files);
}
