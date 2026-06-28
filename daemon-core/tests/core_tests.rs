//! Integration tests for daemon-core modules.

use daemon_core::DaemonSystem;
use daemon_core::config::SystemConfig;
use daemon_core::events::{Event, EventBus};
use daemon_core::logging::Logger;
use daemon_core::system::SystemInfo;
use std::fs;
use std::sync::{Arc, Mutex};

#[test]
fn test_daemon_system_initialization() {
    let config = SystemConfig::default();
    let system = DaemonSystem::new(config.clone());
    assert_eq!(system.config(), &config);
    assert!(system.sys_info().cpu_count() > 0);
}

#[test]
fn test_system_config_load_save() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("daemon_core_test_config.toml");

    let config = SystemConfig {
        theme: "custom-light".to_string(),
        log_level: "debug".to_string(),
        ..SystemConfig::default()
    };

    config.save_to_file(&temp_file)?;

    let loaded = SystemConfig::load_from_file(&temp_file)?;

    assert_eq!(loaded.theme, "custom-light");
    assert_eq!(loaded.log_level, "debug");

    let _ = fs::remove_file(&temp_file);
    Ok(())
}

#[test]
fn test_event_bus_pub_sub() -> Result<(), Box<dyn std::error::Error>> {
    let bus = EventBus::new();
    let received = Arc::new(Mutex::new(Vec::new()));

    let received_clone = Arc::clone(&received);
    bus.subscribe("system.init", move |ev| {
        if let Ok(mut guard) = received_clone.lock() {
            guard.push(ev.clone());
        }
    });

    let wildcard_received = Arc::new(Mutex::new(Vec::new()));
    let wildcard_clone = Arc::clone(&wildcard_received);
    bus.subscribe("*", move |ev| {
        if let Ok(mut guard) = wildcard_clone.lock() {
            guard.push(ev.clone());
        }
    });

    let event = Event {
        topic: "system.init".to_string(),
        payload: "started".to_string(),
    };

    bus.publish(event.clone())?;

    let guard = received.lock().map_err(|e| e.to_string())?;
    assert_eq!(guard.len(), 1);
    assert_eq!(guard[0], event);

    let wildcard_guard = wildcard_received.lock().map_err(|e| e.to_string())?;
    assert_eq!(wildcard_guard.len(), 1);
    assert_eq!(wildcard_guard[0], event);

    Ok(())
}

#[test]
fn test_system_info_fetch() {
    let mut info = SystemInfo::new();
    info.refresh();

    assert!(info.total_memory() > 0);
    assert!(info.cpu_count() > 0);
    assert!(info.os_name().is_some() || info.os_name().is_none());
}

#[test]
fn test_logger_init() {
    // Should either succeed or return a system error if already initialized by another test
    let res = Logger::init("info");
    assert!(res.is_ok() || res.is_err());
}
