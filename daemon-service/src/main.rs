//! Async background daemon entry point.

use daemon_core::logging::Logger;
use daemon_service::config::ConfigWatcher;
use daemon_service::ipc::IpcServer;
use daemon_service::monitor::SystemMonitor;
use daemon_service::runtime::DaemonRuntime;
use daemon_service::service::ServiceRegistry;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), String> {
    // 1. Load initial configurations
    let config_path = Path::new("/etc/daemon/service.toml");
    let mut watcher = ConfigWatcher::new(config_path);
    let initial_config = watcher.current_config();

    // 2. Initialize tracing logging
    if let Err(err) = Logger::init(&initial_config.log_level) {
        eprintln!("Warning: Failed to initialize logger ({err}).");
    }

    tracing::info!("Starting DaemonOS Background Service...");

    // 3. Setup registry and register modular services
    let mut registry = ServiceRegistry::new();

    // Watcher service
    registry.register("config-watcher", move |mut rx| async move {
        watcher.watch_loop(&mut rx).await;
    });

    // UDS IPC Server service
    let ipc_server = IpcServer::new(initial_config.socket_path);
    registry.register("ipc-server", move |rx| async move {
        ipc_server.run(rx).await;
    });

    // Diagnostics System Monitor service
    let mut monitor = SystemMonitor::new();
    registry.register("system-monitor", move |rx| async move {
        monitor.run(rx).await;
    });

    // 4. Start runtime and block until graceful shutdown signals
    let runtime = DaemonRuntime::new(registry);
    runtime.run().await?;

    Ok(())
}
