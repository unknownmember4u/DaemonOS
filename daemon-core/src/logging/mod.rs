//! Centralized logging initialization utilizing the tracing ecosystem.

use crate::errors::{CoreError, Result};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// Service to handle global logger initialization.
pub struct Logger;

impl Logger {
    /// Initialize the global system logging subscriber.
    ///
    /// # Errors
    ///
    /// Returns a `CoreError::System` if the subscriber has already been set or initialization fails.
    pub fn init(level_str: &str) -> Result<()> {
        let level = match level_str.to_lowercase().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        };

        let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

        // Note: global default can only be set once per process lifetime
        tracing::subscriber::set_global_default(subscriber).map_err(|e| {
            CoreError::System(format!("Failed to configure global default logger: {}", e))
        })?;

        Ok(())
    }
}
