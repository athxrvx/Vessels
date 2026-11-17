#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeTarget {
    Podman,
    Docker,
    Auto,
}

impl Default for RuntimeTarget {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub runtime_target: RuntimeTarget,
    pub custom_socket_path: Option<String>,
    pub refresh_interval_secs: u64,
    pub comfortable_density: bool,
    pub default_page: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            runtime_target: RuntimeTarget::Auto,
            custom_socket_path: None,
            refresh_interval_secs: 4,
            comfortable_density: true,
            default_page: "containers".to_string(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("vessels").join("config.toml"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str(&contents) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            if let Ok(serialized) = toml::to_string_pretty(self) {
                std::fs::write(path, serialized)?;
            }
        }
        Ok(())
    }
}
