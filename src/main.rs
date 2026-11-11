#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod commands;
mod db;
mod models;
mod radio;
#[cfg(feature = "voice")]
mod voice;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 560.0])
            .with_min_inner_size([560.0, 520.0])
            .with_title("TSchat")
            .with_decorations(false)
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "TSchat",
        options,
        Box::new(|_cc| Ok(Box::new(app::TsChatApp::default()))),
    )
}
