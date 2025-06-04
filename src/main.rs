mod config;
mod face_detect;
use clap::Parser;
use config::{AppConfig, InputMode};
use glob::glob;
use std::path::PathBuf; // Import PathBuf directly for clarity

use face_detect::{load_image, FaceDetector};

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
            let face_detector_res = FaceDetector::new(&config.cascade_path);
            match face_detector_res {
                Ok(mut face_detector) => {
                    let frame = load_image(path.to_str().unwrap()).unwrap();
                    let cropped_face = face_detector.detect_and_crop_face(&frame).unwrap();
                    let _ = face_detector.save_cropped(&cropped_face, "./cropped.png");
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            };
        }
        InputMode {
            folder: Some(path), ..
        } => {
            println!("Processing folder image: {}", path.display());
        }
        _ => unreachable!("Use one of the modes"),
    }
}
