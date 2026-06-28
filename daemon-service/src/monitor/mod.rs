//! Background system metrics and diagnostic monitoring.

use daemon_core::system::SystemInfo;
use tokio::time::{Duration, interval};
use tracing::info;

/// Background task to monitor system resource usages.
pub struct SystemMonitor {
    sys_info: SystemInfo,
}

impl SystemMonitor {
    /// Create a new SystemMonitor instance.
    pub fn new() -> Self {
        Self {
            sys_info: SystemInfo::new(),
        }
    }

    /// Run the monitoring loop.
    pub async fn run(&mut self, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        let mut ticker = interval(Duration::from_secs(5));
        info!("System monitor loop started.");

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    self.sys_info.refresh();
                    let total = self.sys_info.total_memory();
                    let free = self.sys_info.free_memory();
                    let used = self.sys_info.used_memory();

                    // Convert to MB for display
                    let total_mb = total / (1024 * 1024);
                    let used_mb = used / (1024 * 1024);
                    let free_mb = free / (1024 * 1024);

                    info!(
                        "Resource diagnostics - CPU count: {}, Memory: {} MB / {} MB used ({} MB free)",
                        self.sys_info.cpu_count(),
                        used_mb,
                        total_mb,
                        free_mb
                    );
                }
                _ = shutdown_rx.recv() => {
                    info!("System monitor shutting down...");
                    break;
                }
            }
        }
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}
