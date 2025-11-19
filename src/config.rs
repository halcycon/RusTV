use crate::matrix::Route;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// NDI discovery settings
    pub ndi: NdiConfig,
    /// Matrix routing configuration
    pub matrix: MatrixConfig,
    /// BirdDog camera configurations
    pub birddog: BirdDogConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NdiConfig {
    /// Enable automatic source discovery
    pub auto_discovery: bool,
    /// Discovery interval in seconds
    pub discovery_interval: u64,
    /// Static sources (if any)
    pub static_sources: Vec<StaticSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticSource {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixConfig {
    /// Predefined outputs
    pub outputs: Vec<String>,
    /// Saved routes
    pub routes: Vec<Route>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdDogConfig {
    /// BirdDog camera configurations
    pub cameras: Vec<CameraConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub name: String,
    pub ip_address: String,
    pub ndi_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ndi: NdiConfig {
                auto_discovery: true,
                discovery_interval: 5,
                static_sources: vec![],
            },
            matrix: MatrixConfig {
                outputs: vec![
                    "Monitor 1".to_string(),
                    "Monitor 2".to_string(),
                    "Monitor 3".to_string(),
                    "Monitor 4".to_string(),
                ],
                routes: vec![],
            },
            birddog: BirdDogConfig { cameras: vec![] },
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .context("Failed to read config file")?;
        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(path.as_ref(), content)
            .context("Failed to write config file")?;
        Ok(())
    }

    /// Create a default config file if it doesn't exist
    pub fn ensure_default_config<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            Self::from_file(path)
        } else {
            let config = Self::default();
            config.to_file(&path)?;
            Ok(config)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.ndi.auto_discovery);
        assert_eq!(config.matrix.outputs.len(), 4);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("[ndi]"));
        assert!(toml_str.contains("[matrix]"));
    }
}
