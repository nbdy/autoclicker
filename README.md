# autoclicker

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
- GUI: `eframe`/`egui`
- Global hotkey/input listening: `rdev`
- Input simulation: `enigo`
- Settings: `serde` + `toml` + `directories`
