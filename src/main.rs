mod config;
mod face_detect;
use clap::Parser;
use config::{AppConfig, InputMode};
use glob::glob;
use std::path::PathBuf; // Import PathBuf directly for clarity

use face_detect::FaceDetector;

fn add_cropped_suffix(path: &PathBuf) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| path.as_path());
    let stem = path.file_stem().unwrap_or_default();
    let ext = path.extension().unwrap_or_default();

    let new_file_name = format!(
        "{}_cropped.{}",
        stem.to_str().unwrap(),
        ext.to_str().unwrap()
    );

    parent.join(new_file_name)
}

fn main() {
    let config = AppConfig::parse();

    match &config.input {
        InputMode {
            image: Some(path), ..
        } => {
            println!("Processing single image: {}", path.display());
            let face_detector = FaceDetector::new(&config.cascade_path);
        }
        InputMode {
            folder: Some(path), ..
        } => {
            println!("Processing folder image: {}", path.display());
        }
        _ => unreachable!("Use one of the modes"),
    }
}
