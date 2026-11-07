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
            .with_inner_size([590.0, 437.0])
            .with_min_inner_size([590.0, 437.0])
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
