# ASCII Video Generator

This is a tool to generate ASCII art videos from a set of images or videos. The tool uses Rust and Python programming languages to create an output file that can be played in a terminal. The Rust code is responsible for processing the input files and creating ASCII frames, while the Python code is responsible for pulling the frames out of the given video.

## Installation

To use this tool, you will need to have Rust and Python 3 installed on your computer.

### Rust Installation

You can download Rust from the official [website](https://www.rust-lang.org/tools/install).

### Python Installation

You can download Python from the official [website](https://www.python.org/downloads).

Once you have both Rust and Python installed, you can clone the repository from GitHub:

```bash
git clone https://github.com/ymfyw/ascii-video-generator.git
```

Then, navigate to the project folder and run the following command:

```bash
cd ascii-video-generator && python3 src/frames.py && cargo run
```

You can change the video file in the Python file.



