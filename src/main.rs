//! Main entry point for the application.

pub mod formats;

use crate::formats::wav_format::read_wav_from_file;

/// Main entry point for the application.
fn main() {
    let wave_data = read_wav_from_file("data/test.wav");
    match wave_data {
        Ok(data) => {
            println!("Sample rate: {}", data.sample_rate);
            println!("Number of samples: {}", data.data.len());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
