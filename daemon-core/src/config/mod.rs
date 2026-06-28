//! Configuration management module for the DaemonOS core backend.

use crate::errors::{CoreError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Core configuration schema for DaemonOS services.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemConfig {
    /// Active UI and desktop theme name.
    pub theme: String,
    /// Logging verbosity level (e.g. "trace", "debug", "info", "warn", "error").
    pub log_level: String,
    /// Path to the Unix Domain Socket file for local IPC.
    pub socket_path: String,
    /// List of subsystem features enabled at startup.
    pub enabled_features: Vec<String>,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            theme: "default-dark".to_string(),
            log_level: "info".to_string(),
            socket_path: "/run/user/1000/daemon.sock".to_string(),
            enabled_features: vec!["window-manager".to_string(), "panel".to_string()],
        }
    }
}

impl SystemConfig {
    /// Create a new configuration instance with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load the configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns a `CoreError::Config` if reading or parsing the file fails.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| CoreError::Config(format!("Failed to read file: {}", e)))?;
        let config = toml::from_str(&content)
            .map_err(|e| CoreError::Config(format!("Failed to parse TOML: {}", e)))?;
        Ok(config)
    }

    /// Save the current configuration to a TOML file.
    ///
    /// # Errors
    ///
    /// Returns a `CoreError::Config` if serialization or writing fails.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| CoreError::Config(format!("Failed to serialize TOML: {}", e)))?;
        fs::write(path, content)
            .map_err(|e| CoreError::Config(format!("Failed to write file: {}", e)))?;
        Ok(())
    }
}
