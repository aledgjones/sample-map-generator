extern crate regex;

use regex::Regex;
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

pub fn get_pitch(filename: &str) -> u8 {
    let parts = Regex::new(r"([A-Ga-g]{1})([#b]{0,1})([0-9]{1})")
        .unwrap()
        .captures(&filename)
        .unwrap();
    let letter = parts[1].to_uppercase();
    let accidental = String::from(&parts[2]);
    let octave = parts[3].parse::<u8>().unwrap();

    let step = ["C", "D", "E", "F", "G", "A", "B"]
        .iter()
        .position(|&r| r == letter)
        .unwrap();
    let alteration: u8 = if accidental == "#" { 1 } else { 0 };
    let position = [0, 2, 4, 5, 7, 9, 11].get(step).unwrap() + alteration;

    12 + position + (12 * octave)
}
