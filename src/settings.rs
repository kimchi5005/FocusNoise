use std::{fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::model::{Language, NoiseKind};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub noise_kind: NoiseKind,
    pub volume: f32,
    pub balance: f32,
    pub fade_seconds: f32,
    pub language: Language,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            noise_kind: NoiseKind::White,
            volume: 0.35,
            balance: 0.0,
            fade_seconds: 2.0,
            language: Language::Japanese,
        }
    }
}

pub fn load_settings() -> (Option<PathBuf>, Settings) {
    let path = settings_path();
    let settings = path
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default();

    (path, settings)
}

pub fn save_settings(path: Option<&PathBuf>, settings: &Settings) -> Result<()> {
    let Some(path) = path else {
        return Ok(());
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_vec_pretty(settings)?)?;
    Ok(())
}

fn settings_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "FocusNoise", "FocusNoise")
        .map(|dirs| dirs.config_dir().join("settings.json"))
}
