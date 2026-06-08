use std::{path::PathBuf, time::Duration};

use eframe::egui;

use crate::{
    audio::AudioEngine,
    localization::{language_label, noise_label, state_label, text},
    model::{Language, NoiseKind, PlaybackState},
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

    fn language_controls(&mut self, ui: &mut egui::Ui, lang: Language) {
        ui.horizontal(|ui| {
            ui.label(text(lang, "Language", "言語"));
            let before = self.settings.language;
            for language in Language::ALL {
                ui.selectable_value(
                    &mut self.settings.language,
                    language,
                    language_label(language),
                );
            }
            if self.settings.language != before {
                self.mark_dirty();
            }
        });
    }

    fn noise_controls(&mut self, ui: &mut egui::Ui, lang: Language) {
        ui.horizontal(|ui| {
            let before = self.settings.noise_kind;
            for kind in NoiseKind::ALL {
                ui.selectable_value(&mut self.settings.noise_kind, kind, noise_label(lang, kind));
            }
            if self.settings.noise_kind != before {
                if let Some(audio) = self.audio() {
                    audio.set_noise_kind(self.settings.noise_kind);
                }
                self.mark_dirty();
            }
        });
    }

    fn level_controls(&mut self, ui: &mut egui::Ui, lang: Language) {
        if ui
            .add(
                egui::Slider::new(&mut self.settings.volume, 0.0..=1.0)
                    .text(text(lang, "Volume", "音量")),
            )
            .changed()
        {
            if let Some(audio) = self.audio() {
                audio.set_volume(self.settings.volume);
            }
            self.mark_dirty();
        }

        if ui
            .add(
                egui::Slider::new(&mut self.settings.balance, -1.0..=1.0).text(text(
                    lang,
                    "Balance",
                    "左右バランス",
                )),
            )
            .changed()
        {
            if let Some(audio) = self.audio() {
                audio.set_balance(self.settings.balance);
            }
            self.mark_dirty();
        }

        if ui
            .add(
                egui::Slider::new(&mut self.settings.fade_seconds, 0.0..=10.0).text(text(
                    lang,
                    "Fade seconds",
                    "フェード秒数",
                )),
            )
            .changed()
        {
            if let Some(audio) = self.audio() {
                audio.set_fade_seconds(self.settings.fade_seconds);
            }
            self.mark_dirty();
        }
    }

    fn playback_controls(&mut self, ui: &mut egui::Ui, lang: Language) {
        ui.horizontal_wrapped(|ui| {
            if ui.button(text(lang, "Play", "再生")).clicked() {
                if let Some(audio) = self.audio() {
                    audio.play();
                }
            }
            if ui.button(text(lang, "Pause", "一時停止")).clicked() {
                if let Some(audio) = self.audio() {
                    audio.pause();
                }
            }
            if ui.button(text(lang, "Stop", "停止")).clicked() {
                if let Some(audio) = self.audio() {
                    audio.stop();
                }
            }
        });

        ui.horizontal_wrapped(|ui| {
            if ui.button(text(lang, "Fade in", "フェードイン")).clicked() {
                if let Some(audio) = self.audio() {
                    audio.fade_in();
                }
            }
            if ui
                .button(text(lang, "Fade out", "フェードアウト"))
                .clicked()
            {
                if let Some(audio) = self.audio() {
                    audio.fade_out();
                }
            }
        });
    }
}

impl eframe::App for FocusNoiseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            let lang = self.settings.language;

            ui.add_space(12.0);
            ui.heading(APP_NAME);
            ui.add_space(8.0);

            if let Err(message) = &self.audio {
                ui.colored_label(egui::Color32::from_rgb(255, 150, 120), message);
                ui.add_space(12.0);
            }

            self.language_controls(ui, lang);
            ui.separator();

            ui.label(format!(
                "{}: {}",
                text(lang, "State", "状態"),
                state_label(lang, self.playback())
            ));

            ui.add_space(8.0);
            self.noise_controls(ui, lang);

            ui.add_space(12.0);
            self.level_controls(ui, lang);

            ui.add_space(16.0);
            self.playback_controls(ui, lang);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(text(
                    lang,
                    "Offline. Noise is generated locally.",
                    "オフライン動作。ノイズはローカル生成です。",
                ));
            });
        });

        self.save_soon();
        ctx.request_repaint_after(Duration::from_millis(100));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let _ = save_settings(self.settings_path.as_ref(), &self.settings);
    }
}
