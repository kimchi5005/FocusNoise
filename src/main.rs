#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod audio;
mod model;
mod settings;

use eframe::egui;

use crate::app::FocusNoiseApp;

const APP_NAME: &str = "Focus Noise";
const APP_ICON: &[u8] = include_bytes!("../assets/focus-noise.png");

fn main() -> eframe::Result<()> {
    let icon = eframe::icon_data::from_png_bytes(APP_ICON).ok();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 520.0])
            .with_min_inner_size([360.0, 460.0])
            .with_title(APP_NAME)
            .with_icon(icon.unwrap_or_default()),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Box::new(FocusNoiseApp::new(cc))),
    )
}
