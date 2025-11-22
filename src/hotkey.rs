use std::sync::{Arc, RwLock};

use rdev::{listen, Event, EventType, Key as RdevKey};
use tracing::{error, info};

use crate::keymap::map_rdev_to_key;
use crate::settings::{Settings};

pub fn start_hotkey_listener(settings: Arc<RwLock<Settings>>, active: Arc<RwLock<bool>>) {
    std::thread::spawn(move || {
        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut meta = false;
        let mut toggled_for_combo: bool = false;

        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    update_mods_on_key(key, true, &mut ctrl, &mut alt, &mut shift, &mut meta);

                    if let Some(main_key) = map_rdev_to_key(key) {
                        let hotkey = { settings.read().unwrap().hotkey.clone() };
                        if hotkey.matches_combo(ctrl, alt, shift, meta, &main_key) && !toggled_for_combo {
                            toggled_for_combo = true;
                            let mut a = active.write().unwrap();
                            *a = !*a;
                            info!("Toggled autoclicker: {}", if *a { "ON" } else { "OFF" });
                        }
                    }
                }
                EventType::KeyRelease(key) => {
                    update_mods_on_key(key, false, &mut ctrl, &mut alt, &mut shift, &mut meta);

                    if let Some(main_key) = map_rdev_to_key(key) {
                        let hot = { settings.read().unwrap().hotkey.clone() };
                        if hot.key == main_key {
                            toggled_for_combo = false;
                        }
                    }

                    if !ctrl && !alt && !shift && !meta {
                        toggled_for_combo = false;
                    }
                }
                _ => {}
            }
        };

        if let Err(e) = listen(callback) {
            error!("Global input listener failed: {:?}", e);
        }
    });
}

fn update_mods_on_key(
    key: RdevKey,
    is_down: bool,
    ctrl: &mut bool,
    alt: &mut bool,
    shift: &mut bool,
    meta: &mut bool,
) {
    match key {
        RdevKey::ShiftLeft | RdevKey::ShiftRight => *shift = is_down,
        RdevKey::ControlLeft | RdevKey::ControlRight => *ctrl = is_down,
        RdevKey::Alt | RdevKey::AltGr => *alt = is_down,
        RdevKey::MetaLeft | RdevKey::MetaRight => *meta = is_down,
        _ => {}
    }
}
