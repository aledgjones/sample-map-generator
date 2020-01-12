extern crate regex;

use regex::Regex;
use std::fs::{create_dir, metadata, remove_dir_all};
use std::option::Option;
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

pub fn get_pitch(filename: &str) -> Option<String> {
    lazy_static! {
        static ref PITCH_ACCIDENTAL_OCTAVE: Regex =
            Regex::new(r"([A-Ga-g]{1})([#b]{0,1})([0-9]{1})").unwrap();
    }
    let parts = PITCH_ACCIDENTAL_OCTAVE.captures(&filename);
    match parts {
        None => (),
        Some(parts) => {
            return Some(format!(
                "{}{}{}",
                &parts[1].to_uppercase(),
                &parts[2],
                &parts[3]
            ))
        }
    }

    lazy_static! {
        static ref OCTAVE_PITCH_ACCIDENTAL: Regex =
            Regex::new(r"([0-9]{1})_{1}([A-Ga-g]{1})([#b]{0,1})").unwrap();
    }
    let parts = OCTAVE_PITCH_ACCIDENTAL.captures(&filename);

    match parts {
        None => return None,
        Some(parts) => {
            return Some(format!(
                "{}{}{}",
                &parts[2].to_uppercase(),
                &parts[3],
                &parts[1]
            ))
        }
    }
}
