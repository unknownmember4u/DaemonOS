//! Screen Locker for DaemonOS.

use daemon_config::DaemonConfig;
use daemon_ipc::{IpcChannel, IpcMessage};

fn main() -> Result<(), String> {
    println!("DaemonOS Secure Screen Locker v0.1.0");

    let config = DaemonConfig::load_from_path("/etc/daemon/lock.toml").unwrap_or_else(|err| {
        eprintln!("Warning: Failed to load config ({err}). Using default settings.");
        DaemonConfig::default()
    });

    println!("Starting screen lock. Active theme: {}", config.theme);

    let ipc = IpcChannel::connect("/run/user/1000/daemon.sock")?;
    ipc.send(&IpcMessage::Command("lock_init".to_string()))?;

    Ok(())
}
