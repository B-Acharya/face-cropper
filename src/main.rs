mod config;
mod pipeline;
use clap::Parser;
use config::{AppConfig, InputMode};
use glob::glob;
use pipeline::{process_folder_with_images, process_image};
use std::path::PathBuf; // Import PathBuf directly for clarity
mod face_detect;

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
            let cascade_path = config.cascade_path;
            let image_path = path.to_str().unwrap();
            process_image(&cascade_path, image_path, config.output_path);
        }
        InputMode {
            folder: Some(path), ..
        } => {
            println!("Processing folder image: {}", path.display());
            println!("Processing single image: {}", path.display());
            let cascade_path = config.cascade_path;
            let folder_path = path.to_str().unwrap();
            process_folder_with_images(&cascade_path, folder_path, config.output_path);
        }
        _ => unreachable!("Use one of the modes"),
    }
}
