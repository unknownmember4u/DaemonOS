//! Async daemon runtime coordinator.

use crate::service::ServiceRegistry;
use tokio::sync::broadcast;
use tracing::info;

/// Manages startup, runtime lifecycle, and graceful shutdown.
pub struct DaemonRuntime {
    registry: ServiceRegistry,
    shutdown_tx: broadcast::Sender<()>,
}

impl DaemonRuntime {
    /// Create a new DaemonRuntime instance.
    pub fn new(registry: ServiceRegistry) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            registry,
            shutdown_tx,
        }
    }

    /// Run the daemon, waiting for SIGINT or SIGTERM to initiate graceful shutdown.
    ///
    /// # Errors
    ///
    /// Returns an error message if registering signal listeners fails.
    pub async fn run(self) -> Result<(), String> {
        info!("Daemon runtime starting services...");
        let shutdown_tx = self.shutdown_tx.clone();
        let handles = self.registry.start_all(&shutdown_tx);

        // Listen for system signals
        let sigint = tokio::signal::ctrl_c();

        #[cfg(unix)]
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .map_err(|e| format!("Failed to register SIGTERM listener: {}", e))?;

        #[cfg(not(unix))]
        let mut sigterm = std::future::pending::<()>();

        tokio::select! {
            _ = sigint => {
                info!("SIGINT received, initiating shutdown...");
            }
            _ = sigterm.recv() => {
                info!("SIGTERM received, initiating shutdown...");
            }
        }

        // Broadcast shutdown to all service tasks
        let _ = shutdown_tx.send(());

        // Wait for all background tasks to complete
        for handle in handles {
            let _ = handle.await;
        }

        info!("All services shut down gracefully.");
        Ok(())
    }
}
