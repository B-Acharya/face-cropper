use super::face_detect::load_image;
use super::face_detect::FaceDetector;
use indicatif;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressBar;
use opencv::mod_prelude::ToOutputArray;
use opencv::videoio::VideoCaptureTrait;
use opencv::videoio::VideoCaptureTraitConst;
use rayon::iter::{ParallelBridge, ParallelIterator};

use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn detect_and_save(face_detector: &mut FaceDetector, image_path: &str, save_path: &str) {
    let frame = load_image(image_path).unwrap();
    let cropped_face = face_detector.detect_and_crop_face(&frame).unwrap();
    let _ = face_detector.save_cropped(&cropped_face, save_path);
}

fn check_file_exists(path: &PathBuf) {
    if !path.exists() {
        let res = fs::create_dir(path);
        match res {
            Ok(_) => println!("Created directory: {}", path.display()),
            Err(e) => println!("Failed to create path {}", e),
        };
    }
}

/// Detects a face in a single image and saves the cropped result.
///
/// - `cascade_path`: Path to the Haar cascade XML file.
/// - `image_path`: Path to the input image.
/// - `output_dir`: Directory where the result will be saved.
pub fn process_image(cascade_path: &PathBuf, image_path: &str, output_dir: PathBuf) {
    let face_detector_res = FaceDetector::new(cascade_path);
    let save_path = output_dir.join("facecrop_resutls");

    let _ = check_file_exists(&save_path);

    match face_detector_res {
        Ok(mut face_detector) => {
            //TODO: path::From ? or path::new?
            let input_path = Path::new(image_path);
            let filename = input_path.file_name().unwrap();
            let save_path = save_path.join(filename);

            detect_and_save(&mut face_detector, image_path, save_path.to_str().unwrap());
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
}

fn number_of_files(image_dir: &str) -> usize {
    let entries = fs::read_dir(image_dir).unwrap();
    entries.count()
}

/// Processes all `.png` images in a folder one by one and saves cropped face results.
///
/// - `cascade_path`: Path to the Haar cascade XML file.
/// - `folder_path`: Folder containing `.png` images.
/// - `output_dir`: Directory to save cropped face images.
pub fn process_folder_with_images(cascade_path: &Path, folder_path: &str, output_dir: PathBuf) {
    let count = number_of_files(folder_path);
    let pb = ProgressBar::new(count as u64);
    match FaceDetector::new(cascade_path) {
        Ok(mut face_detector) => {
            let dir = Path::new(folder_path);
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let input_path = entry.path();
                    let filename = input_path.file_name().unwrap();
                    let save_path = output_dir.join("facecrop_results");
                    let _ = check_file_exists(&save_path);
                    let save_path = save_path.join(filename);

                    if input_path.is_file()
                        && input_path
                            .extension()
                            .map(|ext| ext == "png" || ext == "tiff")
                            .unwrap_or(false)
                    {
                        // Process the image
                        detect_and_save(
                            &mut face_detector,
                            input_path.to_str().unwrap(),
                            save_path.to_str().unwrap(),
                        );
                        pb.inc(1);
                    }
                }
                pb.finish_with_message("done");
            }
        }
        Err(e) => eprintln!("Error initializing face detector: {:?}", e),
    }
}

/// Processes all `.png` images in a folder using a standard iterator and saves cropped face results.
///
/// Similar to `process_folder_with_images`, but uses a `.for_each` style iteration.
///
/// - `cascade_path`: Path to the Haar cascade XML file.
/// - `folder_path`: Folder containing images.
/// - `output_dir`: Output directory for cropped faces.
pub fn process_folder_with_images_iter(
    cascade_path: &Path,
    folder_path: &str,
    output_dir: PathBuf,
) {
    let count = number_of_files(folder_path);
    let pb = ProgressBar::new(count as u64);
    let save_path = output_dir.join("facecrop_resutls");
    let _ = check_file_exists(&save_path);
    match FaceDetector::new(cascade_path) {
        Ok(mut face_detector) => {
            let dir = Path::new(folder_path);
            if let Ok(entries) = fs::read_dir(dir) {
                entries
                    .flatten()
                    .filter(|entry| {
                        entry.path().is_file()
                            && entry
                                .path()
                                .extension()
                                .map(|ext| ext == "png" || ext == "tiff")
                                .unwrap_or(false)
                    })
                    .for_each(|entry| {
                        let input_path = entry.path();
                        let filename = input_path.file_name().unwrap();
                        let save_path_image = save_path.join(filename);

                        // Process the image
                        detect_and_save(
                            &mut face_detector,
                            input_path.to_str().unwrap(),
                            save_path_image.to_str().unwrap(),
                        );
                        pb.inc(1);
                    });
                pb.finish_with_message("done");
            };
        }
        Err(e) => eprintln!("Error initializing face detector: {:?}", e),
    }
}

/// Processes images in parallel using Rayon to speed up face detection.
///
/// This function uses multiple threads to process `.png` images in a folder in parallel.
///
/// - `cascade_path`: Path to the Haar cascade XML file.
/// - `folder_path`: Folder containing images.
/// - `output_dir`: Output directory for cropped faces.
pub fn process_folder_with_images_rayon(
    cascade_path: &Path,
    folder_path: &str,
    output_dir: PathBuf,
) {
    //This is doubled and can be removed
    let count = number_of_files(folder_path);
    let save_path = output_dir.join("facecrop_resutls");
    let _ = check_file_exists(&save_path);
    let dir = Path::new(folder_path);
    opencv::core::set_num_threads(1).expect("Failed to set");
    if let Ok(entries) = fs::read_dir(dir) {
        entries
            .flatten()
            .filter(|entry| {
                entry.path().is_file()
                    && entry
                        .path()
                        .extension()
                        .map(|ext| ext == "png" || ext == "tiff")
                        .unwrap_or(false)
            })
            .par_bridge()
            .progress_count(count as u64)
            .for_each(|entry| {
                match FaceDetector::new(cascade_path) {
                    Ok(mut face_detector) => {
                        let input_path = entry.path();
                        let filename = input_path.file_name().unwrap();
                        let save_path_image = save_path.join(filename);

                        // Process the image
                        detect_and_save(
                            &mut face_detector,
                            input_path.to_str().unwrap(),
                            save_path_image.to_str().unwrap(),
                        );
                    }
                    Err(e) => eprintln!("Error initializing face detector: {:?}", e),
                }
            });
    };
}

/// Extracts frames from a video, detects faces, and saves cropped face images.
///
/// - `cascade_path`: Path to the Haar cascade XML file.
/// - `video_path`: Path to the input video file.
/// - `output_dir`: Directory where cropped face images will be saved.
pub fn process_video(cascade_path: &Path, video_path: &str, output_dir: PathBuf) {
    let face_detector_res = FaceDetector::new(cascade_path);
    let input_path = Path::new(video_path);
    //just get the stem
    let filename = input_path.file_stem().unwrap();
    let save_path = output_dir.join(filename);
    println!("{:?}", save_path);
    println!("{}", opencv::core::get_build_information().unwrap());

    let _ = check_file_exists(&save_path);
    //let pb = ProgressBar::new(count as u64);
    match face_detector_res {
        Ok(mut face_detector) => {
            let video_capture_res = opencv::videoio::VideoCapture::from_file_def(video_path);
            match video_capture_res {
                Ok(mut video_capture) => {
                    if video_capture.is_opened().unwrap() {
                        // loop till you get all the frames
                        let mut count = 0;
                        let number_of_frames = video_capture.get(7).unwrap();
                        let pb = ProgressBar::new(number_of_frames as u64);

                        loop {
                            let mut capture_image = opencv::core::Mat::default();

                            if video_capture.read(&mut capture_image).unwrap() {
                                let cropped_face =
                                    face_detector.detect_and_crop_face(&capture_image).unwrap();

                                let save_path_image = save_path.join(count.to_string() + ".png");

                                let _ = face_detector
                                    .save_cropped(&cropped_face, save_path_image.to_str().unwrap());
                                count += 1;
                                pb.inc(1);
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Video Capture faile to open video {}", e);
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
}

//pub fn process_cmbp(
//    cascade_path: &Path,
//    folder_path: &str,
//    output_dir: Pathbuf
//)

//Process Multiple folders with images ?
// folder_path
// -- Participant 1
// ---- Image_1.png
// ---- Image_2.png
// ---- Image_3.png
// ---- ...
// ---- Image_N.png
// -- Participant 3
// ---- Image_1.png
// ---- Image_2.png
// ---- ...
// ---- Image_N.png
// -- ..
// -- Participant 3

//TODO: replace all unwrap with expect or ?

//TODO: Process folder wth videos
