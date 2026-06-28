//! Configuration GUI Settings Center for DaemonOS.

use daemon_config::DaemonConfig;
use daemon_ipc::{IpcChannel, IpcMessage};

fn main() -> Result<(), String> {
    println!("DaemonOS Settings Control Panel v0.1.0");

    let config = DaemonConfig::load_from_path("/etc/daemon/settings.toml").unwrap_or_else(|err| {
        eprintln!("Warning: Failed to load config ({err}). Using default settings.");
        DaemonConfig::default()
    });

    println!("Loaded Settings Manager with theme: {}", config.theme);

    let ipc = IpcChannel::connect("/run/user/1000/daemon.sock")?;
    ipc.send(&IpcMessage::Command("settings_init".to_string()))?;

    Ok(())
}
