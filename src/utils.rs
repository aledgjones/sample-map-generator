use std::fs::{create_dir, metadata, remove_dir_all};
use std::path::PathBuf;

pub fn delete_directory(dir: &PathBuf) {
    let exists = metadata(dir).is_ok();
    if exists {
        remove_dir_all(dir).unwrap();
    }
}

pub fn clear_directory(dir: &PathBuf) {
    let exists = metadata(dir).is_ok();
    if exists {
        remove_dir_all(dir).unwrap();
    }
    create_dir(dir).unwrap();
}

pub fn create_directory(dir: &PathBuf) {
    let exists = metadata(dir).is_ok();
    if !exists {
        create_dir(dir).unwrap();
    }
}
