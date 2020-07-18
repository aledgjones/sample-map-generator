use crate::progress::{next, start};
use crate::utils::get_pitch;
use crate::utils::{clear_directory, delete_directory};
use base64::encode;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::read;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result;

pub fn get_samples(
    source_dir: &PathBuf,
    instrument: &str,
    expression: &str,
) -> Result<HashMap<String, String>, String> {
    let root_dir = env::current_dir().unwrap();
    let temp_dir = root_dir.join("temp");

    clear_directory(&temp_dir);

    let mut samples: HashMap<String, String> = HashMap::new();

    let mut i: f32 = 0.0;
    let len = read_dir(source_dir).unwrap().count() as f32;

    start(&instrument, &expression);

    for entry in read_dir(source_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path = path.as_path();
        let metadata = entry.metadata().unwrap();

        if metadata.is_file() == true && path.extension().unwrap() == "wav" {
            let src = path.to_str().unwrap();
            let pitch = get_pitch(&src).unwrap();
            let dest = temp_dir.join(format!("{}.ogg", pitch));
            let dest = dest.as_path();
            let dest = dest.to_str().unwrap();

            // https://trac.ffmpeg.org/wiki/AudioVolume

            // measure peaks -- first pass
            let result = Command::new("ffmpeg")
                .args(&["-i", src])
                .args(&["-af", "volumedetect", "-f", "null", "-"])
                .output();

            let diff = match result {
                Ok(log) => {
                    let pattern = Regex::new(r"max_volume: (.*) dB").unwrap();
                    let output = String::from_utf8(log.stderr).unwrap();
                    let level = match pattern.captures(&output) {
                        Some(cap) => match cap.get(1) {
                            Some(t) => t.as_str(),
                            None => "0.0",
                        },
                        None => "0.0",
                    };
                    let level: f64 = level.parse().unwrap();
                    let diff = -6.0 - level;
                    format!("{:.1}dB", diff)
                }
                Err(_) => {
                    println!("Error converting. Is Lame installed?");
                    continue;
                }
            };

            // apply filter with calulated params -- second pass
            let filter = format!("volume={}", diff);
            let result = Command::new("ffmpeg")
                .args(&["-i", src])
                .args(&["-af", &filter])
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
            samples.insert(pitch, format!("data:audo/ogg;base64,{}", data));

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
