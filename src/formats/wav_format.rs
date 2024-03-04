use std::path;

// use wav::header::Header;
use wav::bit_depth::BitDepth;

/// Struct to hold the data of a wave file
pub struct WaveData {
    pub data: Vec<f32>,
    pub sample_rate: u32
}

/// Function that reads a wave file from a file and returns a WaveData struct
pub fn read_wav_from_file(filepath: &str) -> Result<WaveData, String> {
    
    let path = path::Path::new(filepath);
    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    println!("Reading file: {}", filepath);
    // Open input file
    let mut inp_file = match std::fs::File::open(filepath) {
        Ok(file) => file,
        Err(_) => return Err("Error opening file".to_string()),
    };

    // Read data from file
    let (header, data) = match wav::read(&mut inp_file) {
        Ok((h, d)) => (h, d),
        Err(_) => return Err("Error reading file".to_string()),
    };

    let sample_rate = header.sampling_rate;
    // let num_channels = header.channel_count;

    let raw_data: Vec<f32> = match data {
        BitDepth::Eight(data) => {
            data.iter().map(|&x| (x as f32) / 128.0 - 1.0).collect()
        },
        BitDepth::Sixteen(data) => {
            data.iter().map(|&x| (x as f32) / 32768.0).collect()
        },
        BitDepth::TwentyFour(data) => {
            data.iter().map(|&x| (x as f32) / 8388608.0).collect()
        },
        BitDepth::ThirtyTwoFloat(data) => {
            data.iter().map(|&x| x).collect()
        },
        BitDepth::Empty => {
            return Err("Error reading file".to_string());
        }
    };

    println!("Successfully read file: {}", filepath);

    // Create a new WaveData struct and return it
    Ok(WaveData {
        data: raw_data,
        sample_rate: sample_rate
    })
}