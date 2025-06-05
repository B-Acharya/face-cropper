use super::face_detect::load_image;
use super::face_detect::FaceDetector;
use indicatif::ProgressBar;
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

pub fn process_image(cascade_path: &PathBuf, image_path: &str, output_dir: PathBuf) {
    let face_detector_res = FaceDetector::new(cascade_path);
    let save_path = output_dir.join("facecrop_resutls");

    let _ = check_file_exists(&save_path);

    match face_detector_res {
        Ok(mut face_detector) => {
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

pub fn process_folder_with_images(cascade_path: &PathBuf, folder_path: &str, output_dir: PathBuf) {
    match FaceDetector::new(cascade_path) {
        Ok(mut face_detector) => {
            let dir = Path::new(folder_path);
            if let Ok(entries) = fs::read_dir(dir) {
                //TODO: hardcoded and needs to be chagned to the number of files in the directory
                let pb = ProgressBar::new(100);
                for entry in entries.flatten() {
                    let input_path = entry.path();
                    let filename = input_path.file_name().unwrap();
                    let save_path = output_dir.join("facecrop_resutls");
                    let _ = check_file_exists(&save_path);
                    let save_path = save_path.join(filename);

                    if input_path.is_file()
                        && input_path
                            .extension()
                            .map(|ext| ext == "png")
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
