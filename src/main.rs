#[macro_use]
extern crate base64;

mod converter;
mod progress;
mod utils;

use crate::converter::get_samples;
use crate::utils::{clear_directory, create_directory};
use std::env;
use std::fs::{read_dir, write, File};
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let root_dir = env::current_dir()?;
    let source_dir = root_dir.join("source");
    let output_dir = root_dir.join("../solo-composer-ui/public/patches");

    clear_directory(&output_dir);
    let mut buffer = File::create(output_dir.join("contents.csv"))?;

    write!(buffer, "Instrument, Technique, Path\n")?;

    // clear console
    print!("\x1B[2J");

    for entry in read_dir(&source_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() == true {
            let filename = entry.file_name();
            let filename = filename.to_str();
            let filename = filename.unwrap();

            let tokens: Vec<&str> = filename.split("--").collect();

            let instrument = tokens[0];
            let expression = tokens[1];

            let source = source_dir.join(filename);

            let samples = match get_samples(&source, &instrument, &expression) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let output = output_dir.join(instrument);
            create_directory(&output);

            let json = format!("{:#?}", samples);

            write(output.join(format!("{}.json", expression)), json).unwrap();
            write!(
                buffer,
                "{}, {}, {}/{}/{}.json\n",
                instrument, expression, "/output", instrument, expression
            )?;
        }
    }

    Ok(())
}
