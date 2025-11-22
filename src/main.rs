mod app;
mod hotkey;
mod keymap;
mod settings;
mod worker;

use eframe::egui;

fn main() -> eframe::Result<()> {
    setup_tracing();
    let app = app::AutoClickerApp::new();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size(egui::vec2(360.0, 160.0))
            .with_min_inner_size(egui::vec2(280.0, 120.0)),
        ..Default::default()
    };
    eframe::run_native("Autoclicker", native_options, Box::new(|_| Ok(Box::new(app))))
}

fn setup_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}
