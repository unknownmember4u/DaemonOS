//! Configuration schema and validators for the DaemonOS desktop environment.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Configuration schema for daemon-desktop environment setups.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DesktopConfig {
    /// Toggle to enable or disable Hyprland checks and setups.
    pub hyprland_enabled: bool,
    /// Desktop aesthetic theme name.
    pub theme: String,
    /// Absolute or relative path to desktop wallpaper image file.
    pub wallpaper: String,
    /// Selected system cursor theme.
    pub cursor_theme: String,
    /// Selected desktop typography font name.
    pub font_name: String,
    /// List of executable processes triggered during startup orchestration.
    pub startup_commands: Vec<String>,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            hyprland_enabled: true,
            theme: "default-dark".to_string(),
            wallpaper: "/usr/share/backgrounds/daemon-default.png".to_string(),
            cursor_theme: "Adwaita".to_string(),
            font_name: "Sans 10".to_string(),
            startup_commands: vec!["daemon-panel".to_string(), "daemon-launcher".to_string()],
        }
    }
}

impl DesktopConfig {
    /// Create a new DesktopConfig instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load the desktop configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error string if reading or parsing fails.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read desktop config: {e}"))?;
        let config =
            toml::from_str(&content).map_err(|e| format!("Failed to parse desktop TOML: {e}"))?;
        Ok(config)
    }

    /// Validate the configurations to ensure no blank settings are present.
    ///
    /// # Errors
    ///
    /// Returns an error string if any field validation checks fail.
    pub fn validate(&self) -> Result<(), String> {
        if self.theme.trim().is_empty() {
            return Err("Theme name cannot be empty".to_string());
        }
        if self.wallpaper.trim().is_empty() {
            return Err("Wallpaper path cannot be empty".to_string());
        }
        if self.cursor_theme.trim().is_empty() {
            return Err("Cursor theme cannot be empty".to_string());
        }
        if self.font_name.trim().is_empty() {
            return Err("Font name cannot be empty".to_string());
        }
        Ok(())
    }
}
