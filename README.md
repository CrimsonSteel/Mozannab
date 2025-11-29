# Mozannab

Mozannab is a desktop posture monitoring app using Rust, egui, and OpenCV. It detects your posture via webcam, gives feedback, and helps you calibrate for better ergonomics. Designed for privacy, speed, and minimal distraction.

## Installation

1. **Install Rust**: https://rustup.rs
2. **Install Cargo Make**: Run `cargo install cargo-make`
3. **Install OpenCV**: You need OpenCV 4.x DLLs (Windows) in your PATH. See the `opencv` crate docs.
4. **Clone the repo**:
   ```powershell
   git clone https://github.com/CrimsonSteel/Mozannab.git
   cd Mozannab
   ```
5. **Build and run**:
   ```powershell
   cargo make run-release
   ```

## Usage

- Launch the app: `cargo make run-release`
- Select your webcam from the dropdown.
- Click **Calibrate** and follow the on-screen instructions.
- The app will monitor your posture and show warnings if you slouch or lean.

## Features

- Fast posture detection using OpenCV DNN
- Calibration for personalized posture baseline
- Real-time feedback and warnings
- Hotkey support for quick actions
- Minimal, distraction-free UI
- Logging to file for diagnostics

## How to Build

- Use Cargo Make for reproducible builds:
  ```powershell
  cargo make build-release
  ```
- Binaries are in `target/release/`

## Default Shortcuts

- **Alt+W**: Toggle monitoring (default)
- **Alt+Q**: Start calibration (default)
- **Ctrl+Q**: Capture calibration sample during calibration (default)

---

## Notes

- Hotkeys are configurable via a `hotkeys.json` file created under `%APPDATA%/Mozannab` on Windows. The defaults are shown above.
- The posture check interval is configurable via a `config.json` file created under `%APPDATA%/Mozannab` on Windows. The default is 60000 ms

## Important files

- `src/camera_app.rs` — main UI and app glue.
- `src/core/calibrator.rs` — calibration logic.
- `src/core/pose.rs` — pose assessment logic.
- `src/hotkey.rs` — hotkey config and registration.

## TODOs:
- Add more error handling for pose capture failure
- Add a  UI for changing app settings
- Test on Linux & MacOS. MacOS is expected to have problems with shortcuts due to permissions