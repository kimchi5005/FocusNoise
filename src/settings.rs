use std::{fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::model::NoiseKind;

const MAX_OUTPUT_GAIN: f32 = 0.1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub noise_kind: NoiseKind,
    pub volume: u8,
    pub balance: u8,
    pub fade_seconds: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            noise_kind: NoiseKind::White,
            volume: 50,
            balance: 50,
            fade_seconds: 2,
        }
    }
}

impl Settings {
    pub fn output_gain(&self) -> f32 {
        (self.volume as f32 / 100.0) * MAX_OUTPUT_GAIN
    }

    pub fn balance_pan(&self) -> f32 {
        ((self.balance as f32 / 100.0) * 2.0) - 1.0
    }

    pub fn fade_seconds_f32(&self) -> f32 {
        self.fade_seconds as f32
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
