use std::io::{self, Write};

pub fn start(instrument: &str, expression: &str) {
    let width: u8 = 30;
    print!(
        "{:<20} {:<20} [{}]",
        instrument,
        expression,
        (0..width).map(|_| " ").collect::<String>()
    );
    io::stdout().flush().unwrap();
}

pub fn next(instrument: &str, expression: &str, len: &f32, i: &f32) {
    let width: u8 = 30;
    let progress = ((width as f32) * (i / len)).ceil() as u8;
    let remaining = width - progress;
    let done = (0..progress).map(|_| "=").collect::<String>();
    let pending = (0..remaining).map(|_| " ").collect::<String>();
    print!(
        "\r{:<20} {:<20} [{}{}]",
        instrument, expression, done, pending
    );
    io::stdout().flush().unwrap();
}
