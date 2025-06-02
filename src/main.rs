use glob::glob;
use opencv::{core, prelude::CascadeClassifierTrait};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Image
    image_path: std::path::PathBuf,
}

fn add_cropped_suffix(path: &std::path::PathBuf) -> std::path::PathBuf {
    let parent = path.parent().unwrap_or_else(|| path.as_path());
    let stem = path.file_stem().unwrap_or_default();
    let ext = path.extension().unwrap_or_default();

    let new_file_name = format!(
        "{}_cropped.{}",
        stem.to_string_lossy(),
        ext.to_string_lossy()
    );

    parent.join(new_file_name)
}

fn main() {
    //for entry in glob("/media/**/*.jpg").expect("Failed to read glob pattern") {
    //    match entry {
    //        Ok(path) => println!("{:?}", path.display()),
    //        Err(e) => println!("{:?}", e),
    //    }
    let args = Cli::parse();

    let frame = opencv::imgcodecs::imread(&args.image_path.to_str().unwrap(), 0).unwrap();

    // Should be chnanged to detect_multi_scale3_def and can be used to get the best face crop
    let mut classifier =
        opencv::objdetect::CascadeClassifier::new("./haarcascade_frontalface_default.xml").unwrap();

    let mut faces: opencv::core::Vector<opencv::core::Rect> = Default::default();
    let _ = classifier.detect_multi_scale_def(&frame, &mut faces);

    println!("{:?}", faces.get(0));
    let roi = faces.get(0).unwrap();

    let save_name = add_cropped_suffix(&args.image_path);

    let cropedframe = opencv::core::Mat::roi(&frame, roi).unwrap();
    let _ = opencv::imgcodecs::imwrite_def(&save_name.to_str().unwrap(), &cropedframe);
}
