use opencv::objdetect::CascadeClassifier;
use opencv::objdetect::CascadeClassifierTrait;
use std::path::Path;

use anyhow::Result;

pub struct FaceDetector {
    classifier: CascadeClassifier,
}

pub fn load_image(image_path: &str) -> Result<opencv::core::Mat> {
    let frame = opencv::imgcodecs::imread(image_path, opencv::imgcodecs::IMREAD_UNCHANGED)
        .expect("Failed to load Image via OpenCV");
    Ok(frame)
}

impl FaceDetector {
    pub fn new(cascade_path: &Path) -> Result<Self> {
        let cascasde_path_str = cascade_path.to_str().unwrap();
        let classifier_result = opencv::objdetect::CascadeClassifier::new(cascasde_path_str);

        let classifier = match classifier_result {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "Failed to load classifier from '{}': {:?}",
                    cascade_path.display(),
                    e
                );
                return Err(e.into());
            }
        };
        Ok(Self { classifier })
    }

    pub fn detect_faces(
        &mut self,
        frame: &opencv::core::Mat,
    ) -> Result<opencv::core::Vector<opencv::core::Rect>> {
        let mut faces: opencv::core::Vector<opencv::core::Rect> = Default::default();
        let detect_result = self.classifier.detect_multi_scale_def(frame, &mut faces);

        if let Err(e) = detect_result {
            eprintln!("Error during face detection: {:?}", e);
            std::process::exit(1);
        }

        if faces.len() == 0 {
            eprintln!("No faces detected in the image.");
            std::process::exit(1);
        }

        Ok(faces)
    }

    pub fn detect_and_crop_face(&mut self, frame: &opencv::core::Mat) -> Result<opencv::core::Mat> {
        let roi = self.detect_faces(frame).unwrap();

        //This assumes that the face is present, needs a fallback if roi is empty
        let first_roi = roi.get(0).unwrap();

        let cropped_frame = opencv::core::Mat::roi(frame, first_roi).unwrap();

        //Shouldnt a reference be passes here ? costly to pass the whole cropped frame.
        Ok(cropped_frame.clone_pointee())
    }

    pub fn save_cropped(
        &mut self,
        cropped_frame: &opencv::core::Mat,
        save_path: &str,
    ) -> Result<()> {
        let imwrite_result = opencv::imgcodecs::imwrite_def(save_path, &cropped_frame);

        if let Err(e) = imwrite_result {
            eprintln!("Error saving cropped image to '{}': {:?}", save_path, e);
            std::process::exit(1);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Bring the items#[test]
                  //

    fn create_directory_with_images() {}
}
