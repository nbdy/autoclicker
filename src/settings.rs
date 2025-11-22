use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub hotkey: Hotkey,
    pub action: Action,
    pub interval_ms: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hotkey: Hotkey::default_toggle(),
            action: Action::Mouse(MouseButton::Left),
            interval_ms: 60,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Hotkey {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
    pub key: Key,
}

impl Hotkey {
    pub fn default_toggle() -> Self {
        Self { ctrl: true, alt: false, shift: false, meta: false, key: Key::F8 }
    }

    #[must_use]
    pub fn matches_combo(&self, ctrl: bool, alt: bool, shift: bool, meta: bool, key: &Key) -> bool {
        self.ctrl == ctrl && self.alt == alt && self.shift == shift && self.meta == meta && &self.key == key
    }
}

impl Display for Hotkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<&str> = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.meta {
            parts.push("Meta");
        }
        parts.push(self.key.to_str());
        write!(f, "{}", parts.join("+"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Key {
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Space,
    Enter,
    Escape,
    Char(char),
}

impl Key {
    pub fn to_str(&self) -> &str {
        match self {
            Key::F6 => "F6",
            Key::F7 => "F7",
            Key::F8 => "F8",
            Key::F9 => "F9",
            Key::F10 => "F10",
            Key::F11 => "F11",
            Key::F12 => "F12",
            Key::Space => "Space",
            Key::Enter => "Enter",
            Key::Escape => "Escape",
            Key::Char(c) => match c {
                'a' | 'A' => "A",
                'b' | 'B' => "B",
                'c' | 'C' => "C",
                'd' | 'D' => "D",
                'e' | 'E' => "E",
                'f' | 'F' => "F",
                'g' | 'G' => "G",
                'h' | 'H' => "H",
                'i' | 'I' => "I",
                'j' | 'J' => "J",
                'k' | 'K' => "K",
                'l' | 'L' => "L",
                'm' | 'M' => "M",
                'n' | 'N' => "N",
                'o' | 'O' => "O",
                'p' | 'P' => "P",
                'q' | 'Q' => "Q",
                'r' | 'R' => "R",
                's' | 'S' => "S",
                't' | 'T' => "T",
                'u' | 'U' => "U",
                'v' | 'V' => "V",
                'w' | 'W' => "W",
                'x' | 'X' => "X",
                'y' | 'Y' => "Y",
                'z' | 'Z' => "Z",
                '0' => "0",
                '1' => "1",
                '2' => "2",
                '3' => "3",
                '4' => "4",
                '5' => "5",
                '6' => "6",
                '7' => "7",
                '8' => "8",
                '9' => "9",
                _ => "?",
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Action {
    Mouse(MouseButton),
    Keyboard(Key),
}

pub fn config_file_path() -> PathBuf {
    let dirs = ProjectDirs::from("dev", "nbdy", "autoclicker")
        .expect("Failed to resolve project directories");
    let cfg_dir = dirs.config_dir();
    std::fs::create_dir_all(cfg_dir).ok();
    cfg_dir.join("settings.toml")
}

pub fn load_settings(path: &PathBuf) -> Settings {
    match std::fs::read_to_string(path) {
        Ok(s) => toml::from_str::<Settings>(&s).unwrap_or_else(|e| {
            error!("Failed to parse settings TOML: {}", e);
            Settings::default()
        }),
        Err(_) => Settings::default(),
    }
}

pub fn save_settings(path: &PathBuf, settings: &Settings) -> Result<(), String> {
    toml::to_string_pretty(settings)
        .map_err(|e| e.to_string())
        .and_then(|s| std::fs::write(path, s).map_err(|e| e.to_string()))
}
