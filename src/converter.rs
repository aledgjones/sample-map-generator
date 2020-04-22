use crate::progress::{next, start};
use crate::utils::get_pitch;
use crate::utils::{clear_directory, delete_directory};
use base64::encode;
use std::collections::HashMap;
use std::env;
use std::fs::read;
use std::fs::read_dir;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn get_samples(
    source_dir: &PathBuf,
    instrument: &str,
    expression: &str,
) -> Result<HashMap<String, String>> {
    let root_dir = env::current_dir()?;
    let temp_dir = root_dir.join("temp");

    clear_directory(&temp_dir);

    let mut samples: HashMap<String, String> = HashMap::new();

    let mut i: f32 = 0.0;
    let len = read_dir(source_dir).unwrap().count() as f32;

    start(&instrument, &expression);

    for entry in read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        let path = path.as_path();
        let metadata = entry.metadata().unwrap();

        if metadata.is_file() == true && path.extension().unwrap() == "wav" {
            let src = path.to_str().unwrap();
            let pitch = get_pitch(&src).unwrap();
            let dest = temp_dir.join(format!("{}.mp3", pitch));
            let dest = dest.as_path();
            let dest = dest.to_str().unwrap();

            let result = Command::new("lame")
                .arg("-v")
                .args(&["-b", "8"])
                .args(&["-B", "64"])
                .arg(src)
                .arg(dest)
                .output();

            match result {
                Ok(_) => (),
                Err(_) => {
                    println!("Error converting. Is Lame installed?");
                    continue;
                }
            }

            let data = read(dest).unwrap();
            let data = encode(&data);
            samples.insert(pitch, format!("data:audo/mpeg;base64,{}", data));

            i = i + 1.0;
            next(&instrument, &expression, &len, &i);
        }
    }

    println!(
        "\r{:<20} {:<20} ✓{}  ",
        instrument,
        expression,
        (0..30 - 1).map(|_| " ").collect::<String>()
    );

    delete_directory(&temp_dir);

    Ok(samples)
}
