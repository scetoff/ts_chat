#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod db;
mod models;
mod radio;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([980.0, 680.0])
            .with_min_inner_size([720.0, 480.0])
            .with_title("TSchat — Rust edition"),
        ..Default::default()
    };

    eframe::run_native(
        "TSchat",
        options,
        Box::new(|_cc| Ok(Box::new(app::TsChatApp::default()))),
    )
}
