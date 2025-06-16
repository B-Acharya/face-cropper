#!/bin/bash
# Exit on error
set -e

# Always resolve paths relative to this script's directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Path to your Python script (assuming it's in main_app/python/)
PYTHON_SCRIPT="../python-face-cropper/main.py"
RUST_BINARY="../target/release/face-cropper"

# Face detector model path
CASCADE_PATH="../assets/haarcascade_frontalface_default.xml"

# Different modes of operation
IMAGE_PATH="../Lenna.png"
FOLDER_PATH="../test_images/"
VIDEO_PATH="../HR-P0.mp4"

# Save director
OUTPUT_BASE="./output"

OUTPUT_PY_IMAGE="$OUTPUT_BASE/python_image"
OUTPUT_RS_IMAGE="$OUTPUT_BASE/rust_image"
OUTPUT_PY_FOLDER="$OUTPUT_BASE/python_folder"
OUTPUT_RS_FOLDER="$OUTPUT_BASE/rust_folder"
OUTPUT_PY_VIDEO="$OUTPUT_BASE/python_video"
OUTPUT_RS_VIDEO="$OUTPUT_BASE/rust_video"

rm -rf $OUTPUT_BASE

# Clean previous outputs
mkdir -p "$OUTPUT_PY_IMAGE" "$OUTPUT_RS_IMAGE" \
	"$OUTPUT_PY_FOLDER" "$OUTPUT_RS_FOLDER" \
	"$OUTPUT_PY_VIDEO" "$OUTPUT_RS_VIDEO"

#######################################
#          IMAGE MODE BENCHMARK       #
#######################################

echo "== Benchmark: IMAGE mode =="
#
hyperfine --warmup 5 --runs 5 --show-output \
	"python $PYTHON_SCRIPT --image $IMAGE_PATH --cascade-path $CASCADE_PATH --output-path $OUTPUT_PY_IMAGE" \
	"$RUST_BINARY --image $IMAGE_PATH --cascade-path $CASCADE_PATH --output-path $OUTPUT_RS_IMAGE"

#######################################
#          FOLDER MODE BENCHMARK      #
#######################################

#echo -e "\n== Benchmark: FOLDER mode =="
#
#hyperfine --warmup 5 --runs 5 --show-output \
#	"python $PYTHON_SCRIPT --folder $FOLDER_PATH --cascade-path $CASCADE_PATH --output-path $OUTPUT_PY_FOLDER" \
#	"$RUST_BINARY --folder $FOLDER_PATH --cascade-path $CASCADE_PATH --output-path $OUTPUT_RS_FOLDER"

#######################################
#          VIDEO MODE BENCHMARK       #
#######################################

echo -e "\n== Benchmark: VIDEO mode =="

hyperfine --warmup 5 --runs 5 \
	"$RUST_BINARY --video $VIDEO_PATH --cascade-path $CASCADE_PATH --output-path $OUTPUT_RS_VIDEO" \
	"python $PYTHON_SCRIPT --video $VIDEO_PATH --cascade-path $CASCADE_PATH --output-path $OUTPUT_PY_VIDEO"

echo -e "\n✅ All benchmarks completed."
