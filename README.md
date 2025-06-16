# Face Detection CLI Tool

A high-performance, command-line application for face detection using Haar cascades. Developed in **Rust** for speed and robustness, with a reference **Python** implementation for benchmarking.

---

## 🔍 Features

- 🖼️ Process a **single image**
- 🗂️ Batch process **folders of images**
- 🎞️ Detect faces in **videos**
- 🐍 Python version available for comparative benchmarking

---

## 🛠️ Requirements

- **Rust** (install via [rustup](https://rustup.rs))
- Opencv-rust (Check https://github.com/twistedfall/opencv-rust)

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

- **Python 3.8+** (for benchmark comparisons)
- Python dependencies for benchmarking:

  - Install uv [https://docs.astral.sh/uv/guides/install-python/](https://docs.astral.sh/uv/getting-started/installation/)

  ```bash
  cd python-face-cropper

  uv run python-face-cropper/main.py \
  --video ./video.mp4 \
  --cascade-path ./assets/haarcascade_frontalface_default.xml \
  --output-path ./output
  ```

## Benchmarking

Uses hyperfine for benchmarking. Check the /benchmakr/benchmark.sh

## 🐳 Docker Support (Static Compilation)

You can build a **static binary** inside Docker with OpenCV.

### Build Docker Image

The below command will compile and copy the binaries to your local system

```bash
docker build output=. .
```

## 🙌 Acknowledgements

- Opencv-rust
