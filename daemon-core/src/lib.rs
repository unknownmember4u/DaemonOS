//! Core system orchestration library for DaemonOS.

pub use daemon_config::DaemonConfig;
pub use daemon_ipc::{IpcChannel, IpcMessage};

/// Main orchestrator of the DaemonOS desktop components.
#[derive(Debug, Default)]
pub struct DaemonSystem {
    config: DaemonConfig,
    ipc: IpcChannel,
}

impl DaemonSystem {
    /// Initialize the DaemonOS core system with a given configuration.
    pub fn new(config: DaemonConfig) -> Self {
        Self {
            config,
            ipc: IpcChannel::new(),
        }
    }

    /// Establish the core inter-process communication channel.
    ///
    /// # Errors
    ///
    /// Returns a descriptive error message if connection to the socket path fails.
    pub fn start_services(&mut self, socket_path: &str) -> Result<(), String> {
        let channel = IpcChannel::connect(socket_path)?;
        self.ipc = channel;

        // Mock notification sent on successful startup
        let startup_event = IpcMessage::Event {
            sender: "daemon-core".to_string(),
            details: "Services started successfully".to_string(),
        };
        self.ipc.send(&startup_event)?;

        Ok(())
    }

    /// Access the current configuration settings.
    pub fn config(&self) -> &DaemonConfig {
        &self.config
    }
}
