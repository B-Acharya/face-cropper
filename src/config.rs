use std::path::PathBuf; // Import PathBuf directly for clarity

use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "face_extractor",
    version = "0.1",
    about = "Extracts faces from images"
)]
pub struct AppConfig {
    #[command(flatten)]
    pub input: InputMode,

    /// Path to Haarcascade XML file
    #[arg(long)]
    pub cascade_path: PathBuf,

    /// Whether to compute mean intensity for each detected face
    #[arg(long)]
    pub mean: bool,

    // Optional output save DIR ?
    #[arg(short, long)]
    pub output_path: PathBuf,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct InputMode {
    /// Path to a single image
    #[arg(long, group = "input")]
    pub image: Option<PathBuf>,

    /// Path to a folder containing images
    #[arg(long, group = "input")]
    pub folder: Option<PathBuf>,

    #[arg(long, group = "input")]
    pub video: Option<PathBuf>,
}
