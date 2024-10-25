use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub fn get_valid_child_dirs(path: PathBuf) -> impl Iterator<Item = DirEntry> {
    fs::read_dir(&path)
        .expect(&format!("Error reading directory: {:#?}", path))
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_dir() && !entry.file_name().to_string_lossy().starts_with(".")
        })
}

pub fn get_valid_child_files(path: PathBuf) -> impl Iterator<Item = DirEntry> {
    fs::read_dir(&path)
        .expect(&format!("Error reading directory: {:#?}", path))
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_file() && !entry.file_name().to_string_lossy().starts_with(".")
        })
}
