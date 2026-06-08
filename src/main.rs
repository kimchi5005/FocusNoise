#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod audio;
mod model;
mod settings;

use eframe::egui;

use crate::app::FocusNoiseApp;

const APP_NAME: &str = "Focus Noise";

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 520.0])
            .with_min_inner_size([360.0, 460.0])
            .with_title(APP_NAME),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Box::new(FocusNoiseApp::new(cc))),
    )
}
