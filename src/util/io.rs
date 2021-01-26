use std::fs;

pub fn get_files(path: &String, predicate: fn(item: &String) -> bool) -> std::io::Result<Vec<String>> {
  let mut files: Vec<String> = Vec::new();

  for entry in fs::read_dir(path)? {
    let entry_path = entry?.path();

    if entry_path.is_file() {
      let file = entry_path.to_str().unwrap().to_string();

      if predicate(&file) {
        files.push(file);
      }
    }
  }

  return Ok(files);
}

pub fn get_files_recursive(path: &String, predicate: fn(item: &String) -> bool) -> std::io::Result<Vec<String>> {
  let mut files: Vec<String> = Vec::new();

  for entry in fs::read_dir(path)? {
    let entry_path = entry?.path();

    if entry_path.is_file() {
      let file = entry_path.to_str().unwrap().to_string();

      if predicate(&file) {
        files.push(file);
      }
    }

    if entry_path.is_dir() {
      let sub_path = entry_path.to_str().unwrap().to_string();
      files.extend(get_files(&sub_path, predicate).unwrap());
    }
  }

  return Ok(files);
}