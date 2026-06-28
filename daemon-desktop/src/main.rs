//! Desktop environment coordinator for DaemonOS.

pub mod hyprland;
pub mod startup;
pub mod theme;

use daemon_config::DaemonConfig;
use daemon_ipc::{IpcChannel, IpcMessage};
use hyprland::HyprlandManager;
use startup::StartupOrchestrator;
use theme::ThemeManager;

fn main() -> Result<(), String> {
    println!("DaemonOS Desktop Manager v0.1.0");

    let config = DaemonConfig::load_from_path("/etc/daemon/desktop.toml").unwrap_or_else(|err| {
        eprintln!("Warning: Failed to load config ({err}). Using default settings.");
        DaemonConfig::default()
    });

    println!("Starting desktop environment with theme: {}", config.theme);

    // Run modular integrations
    let _ = HyprlandManager::initialize();
    let _ = ThemeManager::apply_theme(&config.theme);
    let _ = StartupOrchestrator::launch();

    let ipc = IpcChannel::connect("/run/user/1000/daemon.sock")?;
    ipc.send(&IpcMessage::Command("desktop_init".to_string()))?;

    Ok(())
}
