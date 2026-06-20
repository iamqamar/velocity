use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub volume: f64,
    pub muted: bool,
    pub playback_speed: f64,
    pub last_directory: Option<PathBuf>,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            volume: 1.0,
            muted: false,
            playback_speed: 1.0,
            last_directory: None,
            window_width: 1280,
            window_height: 720,
        }
    }
}

impl Config {
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("velocity").join("config.toml"))
    }

    /// Load from %APPDATA%/velocity/config.toml, falling back to defaults.
    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            tracing::warn!("couldn't determine config dir, using defaults");
            return Self::default();
        };

        match std::fs::read_to_string(&path) {
            Ok(contents) => toml::from_str(&contents).unwrap_or_else(|e| {
                tracing::warn!("bad config file, using defaults: {e}");
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    /// Persist to %APPDATA%/velocity/config.toml.
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path().context("can't determine config dir")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .context("failed to create config directory")?;
        }

        let toml_str = toml::to_string_pretty(self).context("serialize config")?;
        std::fs::write(&path, toml_str).context("write config file")?;

        Ok(())
    }
}
