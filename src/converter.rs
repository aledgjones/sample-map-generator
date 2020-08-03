use crate::progress::{next, start};
use crate::utils::{clear_directory, delete_directory, get_pitch};
use base64::encode;
use csv;
use regex::Regex;
use std::env;
use std::fmt;
use std::fs::read;
use std::path::PathBuf;
use std::process::Command;
use std::result::Result;

pub struct Sample {
    pitch: u8,
    attack: f32,
    release: f32,
    data: String,
}

impl fmt::Debug for Sample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "  [{},{:.2},{:.2},\"{}\"]",
            self.pitch, self.attack, self.release, self.data
        )
    }
}

pub struct SampleList {
    list: Vec<Sample>,
}
impl SampleList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn push(&mut self, item: Sample) {
        self.list.push(item);
    }
}
impl fmt::Debug for SampleList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: Vec<String> = Vec::new();
        for item in self.list.iter() {
            output.push(format!("{:?}", item));
        }
        write!(f, "[\n{}\n]", output.join(",\n"))
    }
}

pub fn get_samples(
    source_dir: &PathBuf,
    instrument: &str,
    expression: &str,
) -> Result<SampleList, String> {
    let root_dir = env::current_dir().unwrap();
    let temp_dir = root_dir.join("temp");

    clear_directory(&temp_dir);

    let mut samples = SampleList::new();

    let mut i: f32 = 0.0;

    start(&instrument, &expression);

    let meta_path = source_dir.join("meta.csv");
    let mut reader = match csv::Reader::from_path(&meta_path) {
        Ok(r) => r,
        Err(_) => {
            println!(
                "\r{:<20} {:<20} x{}  ",
                instrument,
                expression,
                (0..30 - 1).map(|_| " ").collect::<String>()
            );
            return Err(String::from("meta not found"));
        }
    };
    let len = reader.records().count() as f32;
    let mut reader = csv::Reader::from_path(&meta_path).unwrap();

    for result in reader.records() {
        let result = result.unwrap();

        let filename = result.get(0).unwrap();
        let pitch = get_pitch(filename);
        let attack: f32 = result.get(1).unwrap().parse::<f32>().unwrap();
        let release: f32 = result.get(2).unwrap().parse::<f32>().unwrap();

        let src = source_dir.join(filename);
        let src = src.to_str().unwrap();

        let dest = temp_dir.join(format!("{}.ogg", pitch));
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
                let diff = -9.0 - level;
                format!("{:.1}dB", diff)
            }
            Err(_) => {
                println!("Error converting. Is ffmpeg installed?");
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
                println!("Error converting. Is ffmpeg installed?");
                continue;
            }
        }

        let data = read(dest).unwrap();
        let data = encode(&data);
        samples.push(Sample {
            pitch,
            attack,
            release,
            data: format!("data:audo/ogg;base64,{}", data),
        });

        i = i + 1.0;
        next(&instrument, &expression, &len, &i);
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
