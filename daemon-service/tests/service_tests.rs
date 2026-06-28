//! Integration tests for daemon-service modules.

use daemon_service::config::ConfigWatcher;
use daemon_service::ipc::IpcServer;
use daemon_service::service::ServiceRegistry;
use std::sync::{Arc, Mutex};

#[test]
fn test_config_watcher_initialization() {
    let watcher = ConfigWatcher::new("non_existent_config.toml");
    let current = watcher.current_config();
    assert_eq!(current.theme, "default-dark");
}

#[tokio::test]
async fn test_service_registry_execution() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = ServiceRegistry::new();
    let counter = Arc::new(Mutex::new(0));

    let counter_clone = Arc::clone(&counter);
    registry.register("test-task", move |_rx| async move {
        if let Ok(mut val) = counter_clone.lock() {
            *val += 1;
        }
    });

    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);
    let handles = registry.start_all(&shutdown_tx);

    for h in handles {
        let _ = h.await;
    }

    let final_val = *counter.lock().map_err(|e| e.to_string())?;
    assert_eq!(final_val, 1);
    Ok(())
}

#[test]
fn test_ipc_server_instantiation() {
    let server = IpcServer::new("/tmp/test_socket.sock");
    // Simple path verification
    drop(server);
}
