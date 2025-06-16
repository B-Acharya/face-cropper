import cv2
import click
from pathlib import Path
import numpy as np
import pathlib
from tqdm import tqdm


def process_image(image_path: Path, output_dir: Path, cascade_path=None):
    frame = cv2.imread(str(image_path))
    if cascade_path == None:
        detector = cv2.CascadeClassifier(
            "../assets/haarcascade_frontalface_default.xml"
        )
    else:
        detector = cv2.CascadeClassifier(cascade_path)

    face_region = face_detection(frame, detector, cascade_path=cascade_path)
    cropped_face = frame[
        max(face_region[1], 0) : min(face_region[1] + face_region[3], frame.shape[0]),
        max(face_region[0], 0) : min(face_region[0] + face_region[2], frame.shape[1]),
    ]

    output_save_path = output_dir / (str(image_path.stem) + ".png")
    print(output_save_path)
    cv2.imwrite(str(output_save_path), cropped_face)


def process_video(video_path: Path, output_dir: Path, cascade_path=None):
    video_cap = cv2.VideoCapture(str(video_path))

    if cascade_path == None:
        detector = cv2.CascadeClassifier(
            "../assets/haarcascade_frontalface_default.xml"
        )
    else:
        detector = cv2.CascadeClassifier(cascade_path)

    total_frames = int(video_cap.get(cv2.CAP_PROP_FRAME_COUNT))
    count = 0
    if video_cap.isOpened():

        with tqdm(total=total_frames) as pbar:
            while True:
                ret, frame = video_cap.read()

                if ret:
                    face_region = face_detection(
                        frame, detector, cascade_path=cascade_path
                    )
                    cropped_face = frame[
                        max(face_region[1], 0) : min(
                            face_region[1] + face_region[3], frame.shape[0]
                        ),
                        max(face_region[0], 0) : min(
                            face_region[0] + face_region[2], frame.shape[1]
                        ),
                    ]
                    output_save_path = output_dir / (
                        str(video_path.stem) + "_" + str(count) + ".png"
                    )
                    cv2.imwrite(str(output_save_path), cropped_face)
                    count += 1
                    pbar.update(1)
                else:
                    break


def process_image_folder(folder_path: Path, output_dir: Path, cascade_path=None):
    png_list = list(folder_path.rglob("*.png"))

    if cascade_path == None:
        detector = cv2.CascadeClassifier(
            "../assets/haarcascade_frontalface_default.xml"
        )
    else:
        detector = cv2.CascadeClassifier(cascade_path)
    for path in tqdm(png_list, total=len(png_list)):

        frame = cv2.imread(str(path))
        face_region = face_detection(frame, detector, cascade_path=cascade_path)
        cropped_face = frame[
            max(face_region[1], 0) : min(
                face_region[1] + face_region[3], frame.shape[0]
            ),
            max(face_region[0], 0) : min(
                face_region[0] + face_region[2], frame.shape[1]
            ),
        ]
        output_save_path = output_dir / (str(path.stem) + ".png")
        cv2.imwrite(str(output_save_path), cropped_face)


def face_detection(
    frame, face_detector, use_larger_box=False, larger_box_coef=1.0, cascade_path=None
):
    """Face detection on a single frame.

    Args:
        frame(np.array): a single frame.
        backend(str): backend to utilize for face detection.
        use_larger_box(bool): whether to use a larger bounding box on face detection.
        larger_box_coef(float): Coef. of larger box.
    Returns:
        face_box_coor(List[int]): coordinates of face bouding box.
    """

    face_zone = face_detector.detectMultiScale(frame)

    if len(face_zone) < 1:
        print("ERROR: No Face Detected")
        face_box_coor = [0, 0, frame.shape[0], frame.shape[1]]
    elif len(face_zone) >= 2:
        max_width_index = np.argmax(face_zone[:, 2])  # Index of maximum width
        face_box_coor = face_zone[max_width_index]
        print(
            "Warning: More than one faces are detected. Only cropping the biggest one."
        )
    else:
        face_box_coor = face_zone[0]
    return face_box_coor


@click.command()
@click.option(
    "--image",
    type=click.Path(exists=True, dir_okay=False, path_type=Path),
    help="Path to a single image",
    required=False,
)
@click.option(
    "--folder",
    type=click.Path(exists=True, file_okay=False, path_type=Path),
    help="Path to a folder containing images",
    required=False,
)
@click.option(
    "--video",
    type=click.Path(exists=True, dir_okay=False, path_type=Path),
    help="Path to a video file",
    required=False,
)
@click.option(
    "--cascade-path",
    required=False,
    type=click.Path(exists=True, dir_okay=False, path_type=Path),
    help="Path to Haarcascade XML file",
)
@click.option(
    "--mean",
    is_flag=True,
    help="Whether to compute mean intensity for each detected face",
)
@click.option(
    "--output-path",
    required=True,
    type=click.Path(file_okay=False, path_type=Path),
    help="Directory to save output results",
)
def main(image, folder, video, cascade_path, mean, output_path):

    # path_dir = pathlib.Path("./Lenna.png")
    # path_dir = pathlib.Path("../test_images/")
    # output_dir = pathlib.Path("./cropped_result")
    #
    output_path = output_path / "cropped_result"

    output_path.mkdir(exist_ok=True)

    if image:
        print("Process Image")
        process_image(image, output_path, cascade_path)

    elif folder:
        print("Process Image folder")
        process_image_folder(folder, output_path, cascade_path)

    elif video:
        process_video(video, output_path, cascade_path)


if __name__ == "__main__":
    main()
