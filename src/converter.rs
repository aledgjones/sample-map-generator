use crate::progress::{next, start};
use crate::utils::get_pitch;
use base64::encode;
use std::collections::HashMap;
use std::fmt;
use std::fs::read;
use std::fs::read_dir;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

pub struct Entry {
    looped: bool,
    loop_start: u32,
    loop_end: u32,
    tune: i8,
    data: String,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"loop\": {}, \"loop_start\": {}, \"loop_end\": {}, \"tune\": {}, \"data\": \"{}\"}}",
            self.looped, self.loop_start, self.loop_end, self.tune, self.data
        )
    }
}

pub fn get_samples(
    source_dir: &PathBuf,
    build_dir: &PathBuf,
    instrument: &str,
    expression: &str,
) -> Result<HashMap<String, Entry>> {
    let mut samples: HashMap<String, Entry> = HashMap::new();

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
            let dest = build_dir.join(format!("{}.mp3", pitch));
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
                Err(_) => continue,
            }

            let data = read(dest).unwrap();
            let data = encode(&data);
            let sample = Entry {
                looped: false,
                loop_start: 0,
                loop_end: 0,
                tune: 0,
                data: data,
            };
            samples.insert(pitch, sample);

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

    Ok(samples)
}
