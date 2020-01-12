#[macro_use]
extern crate lazy_static;
extern crate base64;

mod converter;
mod progress;
mod utils;

use crate::converter::get_samples;
use crate::utils::{clear_directory, create_directory, delete_directory};
use std::env;
use std::fs::{read_dir, write, File};
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let root_dir = env::current_dir()?;
    let source_dir = root_dir.join("source");
    let patch_dir = root_dir.join("../composer/public/patches");

    clear_directory(&patch_dir);

    let mut buffer = File::create(patch_dir.join("contents.txt"))?;

    // clear console
    print!("\x1B[2J");

    for entry in read_dir(source_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() == true {
            let filename = entry.file_name();
            let filename = filename.to_str();
            let filename = filename.unwrap();

            let tokens: Vec<&str> = filename.split("--").collect();

            let instrument = tokens[0];
            let expression = tokens[1];

            let source_dir = root_dir.join("source").join(filename);
            let build_dir = root_dir.join("temp");
            let out_dir = root_dir.join("../composer/public/patches").join(instrument);

            create_directory(&build_dir);
            create_directory(&out_dir);

            let samples = get_samples(&source_dir, &build_dir, &instrument, &expression).unwrap();
            let json = format!(
                "{{\n\
                 \"envelope\": {{\
                 \"attackTime\": 0.0,\
                 \"decayTime\": 1.0,\
                 \"peakLevel\": 1.0,\
                 \"sustainLevel\": 0.8,\
                 \"releaseTime\": 0.7,\
                 \"gateTime\": 0.0,\
                 \"releaseCurve\": \"exp\"\
                 }},\n\
                 \"samples\": {:?}\n\
                 }}",
                samples
            );

            write(out_dir.join(format!("{}.json", expression)), json)?;
            write!(
                buffer,
                "{:<20}{:<20}{}/{}/{}.json\n",
                instrument, expression, "/patches", instrument, expression
            )?;

            delete_directory(&build_dir);
        }
    }
    Ok(())
}
