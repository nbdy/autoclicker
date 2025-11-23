use std::path::PathBuf;
use std::sync::{Arc, RwLock, mpsc};
use std::time::Duration;

use eframe::egui;

use crate::hotkey::start_hotkey_listener;
use crate::keymap::map_egui_key_to_key;
use crate::settings::{Action, Key, MouseButton, Settings, config_file_path, save_settings};
use crate::worker::start_click_worker;

#[derive(Debug)]
pub struct AutoClickerApp {
    pub(crate) settings: Arc<RwLock<Settings>>,
    pub(crate) config_path: PathBuf,
    pub(crate) active_flag: Arc<RwLock<bool>>,
    pub(crate) recording_hotkey: bool,
    pub(crate) recording_action_key: bool,
    pub(crate) last_save_error: Option<String>,
    pub(crate) tx_wake: mpsc::Sender<()>,
}

impl AutoClickerApp {
    pub fn new() -> Self {
        let config_path = config_file_path();
        let settings = Arc::new(RwLock::new(crate::settings::load_settings(&config_path)));
        let active_flag = Arc::new(RwLock::new(false));

        let (tx_wake, rx_wake) = mpsc::channel::<()>();

        start_click_worker(Arc::clone(&settings), Arc::clone(&active_flag), rx_wake);
        start_hotkey_listener(Arc::clone(&settings), Arc::clone(&active_flag));

        Self {
            settings,
            config_path,
            active_flag,
            recording_hotkey: false,
            recording_action_key: false,
            last_save_error: None,
            tx_wake,
        }
    }

    fn with_settings_mut<F: FnOnce(&mut Settings)>(&mut self, f: F) {
        {
            let mut s = self.settings.write().unwrap();
            f(&mut s);
        }
        let s_clone = self.settings.read().unwrap().clone();
        if let Err(e) = save_settings(&self.config_path, &s_clone) {
            self.last_save_error = Some(e);
        } else {
            self.last_save_error = None;
            let _ = self.tx_wake.send(());
        }
    }
}

fn last_pressed_key(input: &egui::InputState) -> Option<egui::Key> {
    input.events.iter().rev().find_map(|e| match e {
        egui::Event::Key {
            key, pressed: true, ..
        } => Some(*key),
        _ => None,
    })
}

impl eframe::App for AutoClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(6.0, 4.0);
        style.spacing.window_margin = egui::Margin::same(6);
        style.spacing.button_padding = egui::vec2(6.0, 4.0);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 4.0);

            egui::Grid::new("main_grid")
                .num_columns(2)
                .min_col_width(60.0)
                .striped(false)
                .show(ui, |ui| {
                    ui.label("Status");
                    self.ui_status_row(ui);
                    ui.end_row();

                    ui.label("Hotkey");
                    self.ui_hotkey_row(ui);
                    ui.end_row();

                    self.ui_action_rows(ui);

                    ui.label("Interval");
                    self.ui_interval_row(ui);
                    ui.end_row();

                    self.ui_error_row(ui);
                });
        });

        ctx.request_repaint_after(Duration::from_millis(80));
    }
}

impl AutoClickerApp {
    fn ui_status_row(&self, ui: &mut egui::Ui) {
        let is_active = { *self.active_flag.read().unwrap() };
        ui.horizontal(|ui| {
            let (dot, color) = if is_active {
                ("●", egui::Color32::GREEN)
            } else {
                ("●", egui::Color32::DARK_RED)
            };
            ui.label(egui::RichText::new(dot).color(color));
            ui.label(if is_active { "ON" } else { "OFF" });
        });
    }

    fn ui_hotkey_row(&mut self, ui: &mut egui::Ui) {
        let s = self.settings.read().unwrap().clone();
        let mut hot = s.hotkey.clone();
        if self.recording_hotkey {
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::YELLOW, "Recording…");
                if ui.add(egui::Button::new("Cancel").small()).clicked() {
                    self.recording_hotkey = false;
                }
            });

            let input = ui.input(|i| i.clone());
            let eg_ctrl = input.modifiers.ctrl;
            let eg_alt = input.modifiers.alt;
            let eg_shift = input.modifiers.shift;
            let eg_mac_cmd = input.modifiers.mac_cmd;
            if let Some(ev) = last_pressed_key(&input)
                && let Some(k) = map_egui_key_to_key(ev)
            {
                hot.ctrl = eg_ctrl;
                hot.alt = eg_alt;
                hot.shift = eg_shift;
                hot.meta = eg_mac_cmd;
                hot.key = k;
                self.recording_hotkey = false;
                self.with_settings_mut(|s| s.hotkey = hot);
            }
        } else {
            ui.horizontal(|ui| {
                ui.monospace(format!("{}", hot));
                if ui.add(egui::Button::new("Record").small()).clicked() {
                    self.recording_hotkey = true;
                }
            });
        }
    }

    fn ui_action_rows(&mut self, ui: &mut egui::Ui) {
        ui.label("Action");
        let current = self.settings.read().unwrap().action.clone();
        let mut action = current.clone();
        let mut action_is_mouse = matches!(action, Action::Mouse(_));
        self.ui_action_type_selector(ui, &mut action_is_mouse);
        ui.end_row();

        ui.label("Details");
        action = if action_is_mouse {
            self.ui_mouse_action_details(ui, action)
        } else {
            self.ui_keyboard_action_details(ui, action)
        };
        if action != current {
            self.with_settings_mut(|s| s.action = action.clone());
        }
        ui.end_row();
    }

    fn ui_action_type_selector(&mut self, ui: &mut egui::Ui, action_is_mouse: &mut bool) {
        ui.horizontal(|ui| {
            ui.radio_value(action_is_mouse, true, "Mouse");
            ui.radio_value(action_is_mouse, false, "Key");
        });
    }

    fn ui_mouse_action_details(&mut self, ui: &mut egui::Ui, action: Action) -> Action {
        let mut btn = match action {
            Action::Mouse(b) => b,
            _ => MouseButton::Left,
        };
        egui::ComboBox::from_id_salt("mouse_button")
            .selected_text(match btn {
                MouseButton::Left => "Left",
                MouseButton::Right => "Right",
                MouseButton::Middle => "Middle",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut btn, MouseButton::Left, "Left");
                ui.selectable_value(&mut btn, MouseButton::Right, "Right");
                ui.selectable_value(&mut btn, MouseButton::Middle, "Middle");
            });
        Action::Mouse(btn)
    }

    fn ui_keyboard_action_details(&mut self, ui: &mut egui::Ui, action: Action) -> Action {
        let mut k = match action {
            Action::Keyboard(k) => k,
            _ => Key::Space,
        };
        if self.recording_action_key {
            self.ui_keyboard_recording(ui, &mut k);
        } else {
            self.ui_keyboard_picker(ui, &mut k);
        }
        Action::Keyboard(k)
    }

    fn ui_keyboard_recording(&mut self, ui: &mut egui::Ui, k: &mut Key) {
        ui.horizontal(|ui| {
            ui.colored_label(egui::Color32::YELLOW, "Recording…");
            if ui.add(egui::Button::new("Cancel").small()).clicked() {
                self.recording_action_key = false;
            }
        });
        let input = ui.input(|i| i.clone());
        if let Some(ev) = last_pressed_key(&input)
            && let Some(newk) = map_egui_key_to_key(ev)
        {
            *k = newk;
            self.recording_action_key = false;
        }
    }

    fn ui_keyboard_picker(&mut self, ui: &mut egui::Ui, k: &mut Key) {
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("keyboard_key")
                .selected_text(k.to_str().to_string())
                .show_ui(ui, |ui| {
                    for f in [
                        Key::F6,
                        Key::F7,
                        Key::F8,
                        Key::F9,
                        Key::F10,
                        Key::F11,
                        Key::F12,
                    ]
                    .iter()
                    {
                        ui.selectable_value(k, f.clone(), f.to_str());
                    }
                    for c in [
                        'A', 'S', 'D', 'F', 'J', 'K', 'L', ';', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U',
                        'I', 'O', 'P',
                    ] {
                        if c.is_ascii_alphabetic() {
                            ui.selectable_value(k, Key::Char(c), c.to_string());
                        }
                    }
                    ui.selectable_value(k, Key::Space, "Space");
                    ui.selectable_value(k, Key::Enter, "Enter");
                    ui.selectable_value(k, Key::Escape, "Escape");
                });
            if ui.add(egui::Button::new("Record").small()).clicked() {
                self.recording_action_key = true;
            }
        });
    }

    fn ui_interval_row(&mut self, ui: &mut egui::Ui) {
        let s_clone = self.settings.read().unwrap().clone();
        let mut ms = s_clone.interval_ms as f64;
        let mut cps = if ms > 0.0 { 1000.0 / ms } else { 0.0 };
        ui.horizontal(|ui| {
            ui.label("ms");
            let ms_changed = ui
                .add_sized(
                    [90.0, 22.0],
                    egui::DragValue::new(&mut ms)
                        .speed(1.0)
                        .range(1.0..=10_000.0),
                )
                .changed();
            ui.add_space(8.0);
            ui.label("cps");
            let cps_changed = ui
                .add_sized(
                    [90.0, 22.0],
                    egui::DragValue::new(&mut cps)
                        .speed(0.1)
                        .range(0.1..=1000.0),
                )
                .changed();

            if ms_changed {
                ms = ms.clamp(1.0, 10_000.0);
                self.with_settings_mut(|s| s.interval_ms = ms.round() as u64);
                cps = (1000.0 / ms).clamp(0.1, 1000.0);
            }
            if cps_changed {
                cps = cps.clamp(0.1, 1000.0);
                let new_ms = (1000.0 / cps).round() as u64;
                self.with_settings_mut(|s| s.interval_ms = new_ms.max(1));
            }
        });
    }

    fn ui_error_row(&self, ui: &mut egui::Ui) {
        if let Some(err) = &self.last_save_error {
            ui.label("");
            ui.colored_label(egui::Color32::RED, format!("Save error: {err}"));
            ui.end_row();
        }
    }
}
