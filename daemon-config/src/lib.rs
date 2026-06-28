//! Shared configuration management library for DaemonOS.

/// Represents the global configurations for the DaemonOS desktop components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonConfig {
    /// Visual desktop theme name.
    pub theme: String,
    /// Logging verbosity level (e.g. "info", "debug").
    pub log_level: String,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            theme: "default-dark".to_string(),
            log_level: "info".to_string(),
        }
    }
}

impl DaemonConfig {
    /// Create a new default configuration instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load the configuration from a path.
    ///
    /// # Errors
    ///
    /// Returns a descriptive error message if the path is invalid or empty.
    pub fn load_from_path(path: &str) -> Result<Self, String> {
        if path.trim().is_empty() {
            return Err("Configuration path cannot be empty".to_string());
        }
        // Placeholder logic - returns default configuration for now
        Ok(Self::default())
    }
}
