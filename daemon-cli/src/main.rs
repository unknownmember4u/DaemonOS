//! Command-line interface for controlling and interacting with DaemonOS.

use daemon_core::{DaemonConfig, DaemonSystem};

fn main() -> Result<(), String> {
    println!("DaemonOS CLI Controller v0.1.0");

    // Load configuration safely without unwraps
    let config = DaemonConfig::load_from_path("/etc/daemon/daemon.toml").unwrap_or_else(|err| {
        eprintln!("Warning: Failed to load config ({err}). Using default settings.");
        DaemonConfig::default()
    });

    // Initialize core system manager
    let mut system = DaemonSystem::new(config);
    system.start_services("/run/user/1000/daemon.sock")?;

    println!("Core systems successfully initialized.");
    Ok(())
}
