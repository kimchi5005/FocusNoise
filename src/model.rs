use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoiseKind {
    White,
    Brown,
}

impl NoiseKind {
    pub const ALL: [Self; 2] = [Self::White, Self::Brown];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    FadingIn,
    FadingOut,
}
