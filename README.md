# autoclicker

[![CI](https://github.com/nbdy/autoclicker/actions/workflows/ci.yml/badge.svg)](https://github.com/nbdy/autoclicker/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/nbdy/autoclicker?sort=semver)](https://github.com/nbdy/autoclicker/releases)
[![License](https://img.shields.io/github/license/nbdy/autoclicker)](LICENSE)
[![Rust 2024](https://img.shields.io/badge/rust-2024-orange?logo=rust)](Cargo.toml)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue)](#)

Minimal cross‑platform autoclicker with a tiny GUI (Rust + egui/eframe + rdev/enigo).

### What it does
- Global hotkey toggles the autoclicker on/off (works while the window is in the background)
- Repeatedly clicks the selected mouse button at the current cursor position, or simulates a keyboard key
- Lets you configure:
  - Toggle hotkey (modifiers + key)
  - Action: Mouse (Left/Right/Middle) or a keyboard key
  - Interval (milliseconds or CPS)
- Settings are saved automatically and reloaded on startup

### Build & run
```
cargo run --release
```

### Notes
- Default toggle hotkey: Ctrl+F8. Change it via the GUI (Hotkey → Record, then press your combination).
- On Linux:
  - Under Wayland, global input hooking/injection may be limited depending on compositor/security settings. X11 sessions typically work out‑of‑the‑box.
  - If clicks do not work, ensure you have the required input permissions and that your session/compositor allows global event listening.
- On macOS you may need to grant Accessibility permissions to the terminal/app for input simulation and global hotkeys to work.

### Dependencies
- GUI: `eframe & egui`
- Global hotkey/input listening: `rdev`
- Input simulation: `enigo`
- Settings: `serde` + `toml` + `directories`
