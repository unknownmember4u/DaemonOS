//! Desktop environment startup sequence command building.

use crate::config::DesktopConfig;

/// Manages constructing lists of system startup commands.
pub struct StartupOrchestrator;

impl StartupOrchestrator {
    /// Build startup commands lists based on active settings.
    pub fn build_commands(config: &DesktopConfig) -> Vec<String> {
        config.startup_commands.clone()
    }
}
