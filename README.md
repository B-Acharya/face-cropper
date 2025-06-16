# Face Detection CLI Tool

A high-performance, command-line application for face detection using Haar cascades. Developed in **Rust** for speed and robustness, with a reference **Python** implementation for benchmarking.

---

## 🔍 Features

- 🖼️ Process a **single image**
- 🗂️ Batch process **folders of images**
- 🎞️ Detect faces in **videos**
- 🚀 Optimized for performance and benchmarked
- 🐍 Python version available for comparative benchmarking

---

## 🛠️ Requirements

- **Rust** (install via [rustup](https://rustup.rs))
- Opencv-rust (Check https://github.com/twistedfall/opencv-rust)
- **Python 3.8+** (for benchmark comparisons)
- Python dependencies for benchmarking:
  ```bash
  pip install opencv-python click tqdm numpy
  ```

## Building the Rust application

```bash
cargo build --release
```

## Usage

The binaries are stored in /target/release/

| Option                  | Description                              | Required |
| ----------------------- | ---------------------------------------- | -------- |
| `--image <PATH>`        | Path to a single image                   | No       |
| `--folder <PATH>`       | Path to a folder containing images       | No       |
| `--video <PATH>`        | Path to a video file                     | No       |
| `--cascade-path <PATH>` | Path to Haar cascade XML file            | Yes      |
| `--output-path <PATH>`  | Directory where output results are saved | Yes      |

Note: Only one of --image, --folder, or --video should be provided at a time.

## Python implementation

uv run python-face-cropper/main.py \
 --video ./video.mp4 \
 --cascade-path ./assets/haarcascade_frontalface_default.xml \
 --output-path ./output

## Benchmarking

Uses hyperfine for benchmarking. Check the /benchmakr/benchmark.sh

## 🐳 Docker Support (Static Compilation)

You can build a **fully static binary** inside Docker with OpenCV and FFmpeg linked statically.

### Build Docker Image

```bash
docker build -t face-cropper-builder .
```

## 🙌 Acknowledgements

- Opencv-rust
