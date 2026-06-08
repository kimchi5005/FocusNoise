use crate::model::{Language, NoiseKind, PlaybackState};

pub fn text<'a>(language: Language, english: &'a str, japanese: &'a str) -> &'a str {
    match language {
        Language::English => english,
        Language::Japanese => japanese,
    }
}

pub fn language_label(language: Language) -> &'static str {
    match language {
        Language::Japanese => "日本語",
        Language::English => "English",
    }
}

pub fn noise_label(language: Language, kind: NoiseKind) -> &'static str {
    match (language, kind) {
        (Language::English, NoiseKind::White) => "White noise",
        (Language::English, NoiseKind::Brown) => "Brown noise",
        (Language::Japanese, NoiseKind::White) => "ホワイトノイズ",
        (Language::Japanese, NoiseKind::Brown) => "ブラウンノイズ",
    }
}

pub fn state_label(language: Language, state: PlaybackState) -> &'static str {
    match (language, state) {
        (Language::English, PlaybackState::Stopped) => "Stopped",
        (Language::English, PlaybackState::Playing) => "Playing",
        (Language::English, PlaybackState::Paused) => "Paused",
        (Language::English, PlaybackState::FadingIn) => "Fading in",
        (Language::English, PlaybackState::FadingOut) => "Fading out",
        (Language::Japanese, PlaybackState::Stopped) => "停止中",
        (Language::Japanese, PlaybackState::Playing) => "再生中",
        (Language::Japanese, PlaybackState::Paused) => "一時停止中",
        (Language::Japanese, PlaybackState::FadingIn) => "フェードイン中",
        (Language::Japanese, PlaybackState::FadingOut) => "フェードアウト中",
    }
}
