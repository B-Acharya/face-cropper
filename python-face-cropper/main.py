import cv2
import numpy as np
import pathlib


def face_detection(frame, use_larger_box=False, larger_box_coef=1.0):
    """Face detection on a single frame.

    Args:
        frame(np.array): a single frame.
        backend(str): backend to utilize for face detection.
        use_larger_box(bool): whether to use a larger bounding box on face detection.
        larger_box_coef(float): Coef. of larger box.
    Returns:
        face_box_coor(List[int]): coordinates of face bouding box.
    """
    detector = cv2.CascadeClassifier("../assets/haarcascade_frontalface_default.xml")
    face_zone = detector.detectMultiScale(frame)

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


if "__main__" == __name__:

    # path_dir = pathlib.Path("./Lenna.png")
    path_dir = pathlib.Path("../test_images/")
    output_dir = pathlib.Path("./cropped_result")

    output_dir.mkdir(exist_ok=True)

    if path_dir.is_dir():
        png_list = path_dir.rglob("*.png")

        for path in png_list:

            frame = cv2.imread(str(path))
            face_region = face_detection(frame=frame)
            cropped_face = frame[
                max(face_region[1], 0) : min(
                    face_region[1] + face_region[3], frame.shape[0]
                ),
                max(face_region[0], 0) : min(
                    face_region[0] + face_region[2], frame.shape[1]
                ),
            ]
            output_save_path = output_dir / (str(path.stem) + ".png")
            print(output_save_path)
            cv2.imwrite(str(output_save_path), cropped_face)
    else:
        frame = cv2.imread(str(path_dir))
        face_region = face_detection(frame=frame)
        cropped_face = frame[
            max(face_region[1], 0) : min(
                face_region[1] + face_region[3], frame.shape[0]
            ),
            max(face_region[0], 0) : min(
                face_region[0] + face_region[2], frame.shape[1]
            ),
        ]

        output_save_path = output_dir / (str(path_dir.stem) + ".png")
        print(output_save_path)
        cv2.imwrite(str(output_save_path), cropped_face)
