use std::fs;
use std::path::Path;

pub fn get_file_separator() -> char {
    if cfg!(target_os = "windows") {
        '\\'
    } else {
        '/'
    }
}

pub fn folder_exists(folder_path: &str) -> bool {
    let path = Path::new(folder_path);
    let metadata_result = fs::metadata(path);

    match metadata_result {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    let metadata_result = fs::metadata(path);

    match metadata_result {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}