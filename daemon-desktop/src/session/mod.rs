//! Desktop environment session tracker and lifecycle management.

use crate::config::DesktopConfig;

/// Manages desktop environment session state.
pub struct SessionManager {
    is_active: bool,
}

impl SessionManager {
    /// Create a new SessionManager instance.
    pub fn new() -> Self {
        Self { is_active: false }
    }

    /// Start a desktop session after validating the configuration.
    ///
    /// # Errors
    ///
    /// Returns an error string if configuration validation fails.
    pub fn start_session(&mut self, config: &DesktopConfig) -> Result<(), String> {
        config.validate()?;
        self.is_active = true;
        Ok(())
    }

    /// Check if the desktop session is currently active.
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
