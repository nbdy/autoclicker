use std::sync::{Arc, RwLock, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use crate::settings::{Action, Key, MouseButton, Settings};

pub fn start_click_worker(
    settings: Arc<RwLock<Settings>>,
    active: Arc<RwLock<bool>>,
    rx_wake: mpsc::Receiver<()>,
) {
    thread::spawn(move || {
        #[cfg(feature = "clicking_enigo")]
        let mut enigo = enigo::Enigo::new(&enigo::Settings::default()).ok();

        let mut last_tick = Instant::now();
        loop {
            while rx_wake.try_recv().is_ok() {}

            let is_active = { *active.read().unwrap() };
            if !is_active {
                thread::sleep(Duration::from_millis(50));
                last_tick = Instant::now();
                continue;
            }

            let (action, interval_ms) = {
                let s = settings.read().unwrap();
                (s.action.clone(), s.interval_ms)
            };

            let interval = Duration::from_millis(interval_ms.max(1));
            let now = Instant::now();
            let elapsed = now.saturating_duration_since(last_tick);
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
            last_tick = Instant::now();

            #[cfg(feature = "clicking_enigo")]
            {
                if let Some(enigo) = enigo.as_mut() {
                    use enigo::{Button as EButton, Direction as EDir, Keyboard as _, Mouse as _};
                    match action {
                        Action::Mouse(MouseButton::Left) => {
                            let _ = enigo.button(EButton::Left, EDir::Click);
                        }
                        Action::Mouse(MouseButton::Right) => {
                            let _ = enigo.button(EButton::Right, EDir::Click);
                        }
                        Action::Mouse(MouseButton::Middle) => {
                            let _ = enigo.button(EButton::Middle, EDir::Click);
                        }
                        Action::Keyboard(k) => match k {
                            Key::Char(c) => {
                                let s = c.to_string();
                                let _ = enigo.text(&s);
                            }
                            Key::Space => {
                                let _ = enigo.text(" ");
                            }
                            Key::Enter => {
                                let _ = enigo.text("\n");
                            }
                            _ => {
                                if let Some(ek) = map_key_to_enigo(&k) {
                                    let _ = enigo.key(ek, enigo::Direction::Click);
                                }
                            }
                        },
                    }
                }
            }

            #[cfg(not(feature = "clicking_enigo"))]
            {
                let _ = action; // no-op when enigo disabled
            }
        }
    });
}

#[cfg(feature = "clicking_enigo")]
fn map_key_to_enigo(key: &Key) -> Option<enigo::Key> {
    match key {
        Key::Char(c) => Some(enigo::Key::Unicode(*c)),
        _ => None,
    }
}
