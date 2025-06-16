mod config;
mod pipeline;
use clap::Parser;
use config::{AppConfig, InputMode};
use pipeline::{process_folder_with_images_iter, process_image, process_video};
mod face_detect;

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
            let cascade_path = config.cascade_path;
            let folder_path = path.to_str().unwrap();
            //process_folder_with_images(&cascade_path, folder_path, config.output_path);
            process_folder_with_images_iter(&cascade_path, folder_path, config.output_path);
        }

        InputMode {
            video: Some(path), ..
        } => {
            println!("Processing single video: {}", path.display());
            let cascade_path = config.cascade_path;
            let vidoe_path = path.to_str().unwrap();
            process_video(&cascade_path, vidoe_path, config.output_path);
        }
        _ => unreachable!("Use one of the modes"),
    }
}
