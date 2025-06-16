pub mod config;
pub mod face_detect;
pub mod pipeline;

pub use pipeline::{
    process_folder_with_images_iter, process_folder_with_images_rayon, process_image, process_video,
};
