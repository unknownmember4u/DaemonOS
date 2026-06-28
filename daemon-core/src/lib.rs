//! Core system orchestration library for DaemonOS.

pub mod config;
pub mod errors;
pub mod events;
pub mod ipc;
pub mod logging;
pub mod system;

// Re-expose primary items for standard usage
pub use config::SystemConfig;
pub use errors::{CoreError, Result};
pub use events::{Event, EventBus};
pub use ipc::{IpcBroker, LayoutMessage};
pub use logging::Logger;
pub use system::SystemInfo;

pub use daemon_config::DaemonConfig;
pub use daemon_ipc::{IpcChannel, IpcMessage};

/// Main orchestrator of the DaemonOS desktop components.
pub struct DaemonSystem {
    config: SystemConfig,
    events: EventBus,
    sys_info: SystemInfo,
}

impl DaemonSystem {
    /// Initialize the DaemonOS core system with a given configuration.
    pub fn new(config: SystemConfig) -> Self {
        Self {
            config,
            events: EventBus::new(),
            sys_info: SystemInfo::new(),
        }
    }

    /// Access the current configuration settings.
    pub fn config(&self) -> &SystemConfig {
        &self.config
    }

    /// Access the system event bus.
    pub fn events(&self) -> &EventBus {
        &self.events
    }

    /// Access the system information module.
    pub fn sys_info(&self) -> &SystemInfo {
        &self.sys_info
    }

    /// Placeholder service startup handler.
    ///
    /// # Errors
    ///
    /// Returns an error string on failure.
    pub fn start_services(&mut self, _socket_path: &str) -> std::result::Result<(), String> {
        Ok(())
    }
}

impl Default for DaemonSystem {
    fn default() -> Self {
        Self::new(SystemConfig::default())
    }
}
