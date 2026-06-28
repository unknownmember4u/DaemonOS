//! Async configuration watcher for the background daemon.

use daemon_core::config::SystemConfig;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::sync::watch;
use tracing::{error, info};

/// Watches a configuration file and broadcasts updates to tasks.
pub struct ConfigWatcher {
    path: PathBuf,
    last_modified: SystemTime,
    tx: watch::Sender<SystemConfig>,
    rx: watch::Receiver<SystemConfig>,
}

impl ConfigWatcher {
    /// Create a new ConfigWatcher instance for a specified file path.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path_buf = path.as_ref().to_path_buf();
        let initial_config = SystemConfig::load_from_file(&path_buf).unwrap_or_default();
        let last_modified = std::fs::metadata(&path_buf)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let (tx, rx) = watch::channel(initial_config);

        Self {
            path: path_buf,
            last_modified,
            tx,
            rx,
        }
    }

    /// Retrieve the current configuration.
    pub fn current_config(&self) -> SystemConfig {
        self.rx.borrow().clone()
    }

    /// Retrieve a receiver channel to subscribe to future configurations.
    pub fn subscribe(&self) -> watch::Receiver<SystemConfig> {
        self.rx.clone()
    }

    /// Periodically poll the file and broadcast reloading events.
    pub async fn watch_loop(&mut self, shutdown_rx: &mut tokio::sync::broadcast::Receiver<()>) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(500));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if let Some(modified) = std::fs::metadata(&self.path)
                        .and_then(|m| m.modified())
                        .ok()
                        .filter(|&m| m > self.last_modified)
                    {
                        self.last_modified = modified;
                        info!("Configuration change detected, reloading...");
                        match SystemConfig::load_from_file(&self.path) {
                            Ok(config) => {
                                let _ = self.tx.send(config);
                                info!("Configuration successfully reloaded.");
                            }
                            Err(err) => {
                                error!("Failed to reload configuration: {}", err);
                            }
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    info!("Config watcher shutting down...");
                    break;
                }
            }
        }
    }
}
