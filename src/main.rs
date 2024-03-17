//! Main entry point for the application.
#![warn(unused_extern_crates)]

// pub mod formats;
pub mod visualization;

use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.retroherna.cassie";

// use crate::formats::wav_format;
use crate::visualization::ui;

/// Main entry point for the application.
fn main() -> glib::ExitCode {
    // let wave_data = wav_format::read_wav_from_file("data/test.wav");
    // match wave_data {
    //     Ok(data) => {
    //         println!("Sample rate: {}", data.sample_rate);
    //         println!("Number of samples: {}", data.data.len());
    //     },
    //     Err(e) => {
    //         println!("Error: {}", e);
    //     }
    // }
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);

    // Run the application
    app.run()
}
