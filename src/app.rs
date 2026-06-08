use std::{path::PathBuf, time::Duration};

use eframe::egui;

use crate::{
    audio::AudioEngine,
    model::{NoiseKind, PlaybackState},
    settings::{load_settings, save_settings, Settings},
    APP_NAME,
};

const SETTINGS_SAVE_INTERVAL_FRAMES: u32 = 20;

pub struct FocusNoiseApp {
    settings: Settings,
    settings_path: Option<PathBuf>,
    audio: Result<AudioEngine, String>,
    dirty_frames: u32,
}

impl FocusNoiseApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let (settings_path, settings) = load_settings();
        let audio = AudioEngine::new(&settings).map_err(|err| err.to_string());

        Self {
            settings,
            settings_path,
            audio,
            dirty_frames: 0,
        }
    }

    fn mark_dirty(&mut self) {
        self.dirty_frames = SETTINGS_SAVE_INTERVAL_FRAMES;
    }

    fn save_soon(&mut self) {
        if self.dirty_frames == 0 {
            return;
        }
        self.dirty_frames -= 1;
        if self.dirty_frames == 0 {
            let _ = save_settings(self.settings_path.as_ref(), &self.settings);
        }
    }

    fn audio(&self) -> Option<&AudioEngine> {
        self.audio.as_ref().ok()
    }

    fn playback(&self) -> PlaybackState {
        self.audio()
            .map(AudioEngine::playback)
            .unwrap_or(PlaybackState::Stopped)
    }

    fn noise_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let before = self.settings.noise_kind;
            for kind in NoiseKind::ALL {
                ui.selectable_value(&mut self.settings.noise_kind, kind, noise_label(kind));
            }
            if self.settings.noise_kind != before {
                if let Some(audio) = self.audio() {
                    audio.set_noise_kind(self.settings.noise_kind);
                }
                self.mark_dirty();
            }
        });
    }

    fn level_controls(&mut self, ui: &mut egui::Ui) {
        if ui
            .add(egui::Slider::new(&mut self.settings.volume, 0..=100).text("Volume"))
            .changed()
        {
            if let Some(audio) = self.audio() {
                audio.set_volume(self.settings.output_gain());
            }
            self.mark_dirty();
        }

        if ui
            .add(egui::Slider::new(&mut self.settings.balance, 0..=100).text("Balance"))
            .changed()
        {
            if let Some(audio) = self.audio() {
                audio.set_balance(self.settings.balance_pan());
            }
            self.mark_dirty();
        }

        if ui
            .add(egui::Slider::new(&mut self.settings.fade_seconds, 0..=100).text("Fade seconds"))
            .changed()
        {
            if let Some(audio) = self.audio() {
                audio.set_fade_seconds(self.settings.fade_seconds_f32());
            }
            self.mark_dirty();
        }
    }

    fn playback_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            if ui.button("Play").clicked() {
                if let Some(audio) = self.audio() {
                    audio.play();
                }
            }
            if ui.button("Pause").clicked() {
                if let Some(audio) = self.audio() {
                    audio.pause();
                }
            }
            if ui.button("Stop").clicked() {
                if let Some(audio) = self.audio() {
                    audio.stop();
                }
            }
        });
    }
}

fn noise_label(kind: NoiseKind) -> &'static str {
    match kind {
        NoiseKind::White => "White noise",
        NoiseKind::Brown => "Brown noise",
    }
}

fn state_label(state: PlaybackState) -> &'static str {
    match state {
        PlaybackState::Stopped => "Stopped",
        PlaybackState::Playing => "Playing",
        PlaybackState::Paused => "Paused",
        PlaybackState::FadingIn => "Fading in",
        PlaybackState::FadingOut => "Fading out",
    }
}

fn apply_responsive_style(ctx: &egui::Context) {
    let width = ctx.input(|input| input.screen_rect().width());
    let scale = (width / 420.0).clamp(0.85, 1.45);
    let mut style = (*ctx.style()).clone();

    style.visuals = egui::Visuals::dark();
    style.spacing.item_spacing = egui::vec2(8.0 * scale, 8.0 * scale);
    style.spacing.button_padding = egui::vec2(10.0 * scale, 6.0 * scale);
    style.spacing.slider_width = 220.0 * scale;
    style.text_styles = [
        (
            egui::TextStyle::Heading,
            egui::FontId::proportional(28.0 * scale),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::proportional(16.0 * scale),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::proportional(16.0 * scale),
        ),
        (
            egui::TextStyle::Small,
            egui::FontId::proportional(12.0 * scale),
        ),
        (
            egui::TextStyle::Monospace,
            egui::FontId::monospace(14.0 * scale),
        ),
    ]
    .into();
    ctx.set_style(style);
}

impl eframe::App for FocusNoiseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_responsive_style(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(12.0);
            ui.heading(APP_NAME);
            ui.add_space(8.0);

            if let Err(message) = &self.audio {
                ui.colored_label(egui::Color32::from_rgb(255, 150, 120), message);
                ui.add_space(12.0);
            }

            ui.label(format!("State: {}", state_label(self.playback())));

            ui.add_space(8.0);
            self.noise_controls(ui);

            ui.add_space(12.0);
            self.level_controls(ui);

            ui.add_space(16.0);
            self.playback_controls(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label("Offline. Noise is generated locally.");
            });
        });

        self.save_soon();
        ctx.request_repaint_after(Duration::from_millis(100));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let _ = save_settings(self.settings_path.as_ref(), &self.settings);
    }
}
