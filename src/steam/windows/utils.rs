use case::CaseExt;
use std::path::{Path, PathBuf};

pub fn fix_path(path: &Path) -> PathBuf {
    let path_string = path.display().to_string();
    let words = path_string.split("/").collect::<Vec<&str>>();
    let mut result_path = PathBuf::new();

    for word in words {
        let mut new_word = word.to_camel();

        if new_word.eq("C:") {
            new_word.push_str(&std::path::MAIN_SEPARATOR.to_string())
        }

        if new_word.ends_with("X86") {
            new_word = new_word.replace("X86", "(x86)")
        }

        if new_word.eq("Steam Exe") {
            new_word = String::from("steam.exe");
        }

        result_path = result_path.join(new_word);
    }

    return result_path;
}
