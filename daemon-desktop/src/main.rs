//! Desktop environment coordinator binary for DaemonOS.

use daemon_desktop::config::DesktopConfig;
use daemon_desktop::hyprland::HyprlandManager;
use daemon_desktop::session::SessionManager;
use daemon_desktop::startup::StartupOrchestrator;
use daemon_desktop::theme::ThemeManager;
use daemon_desktop::wallpaper::WallpaperManager;
use daemon_ipc::{IpcChannel, IpcMessage};
use std::path::Path;

fn main() -> Result<(), String> {
    println!("DaemonOS Desktop Manager v0.1.0");

    let config_path = Path::new("/etc/daemon/desktop.toml");
    let config = DesktopConfig::load_from_file(config_path).unwrap_or_else(|err| {
        eprintln!("Warning: Failed to load config ({err}). Using default settings.");
        DesktopConfig::default()
    });

    config.validate()?;

    // 1. Detect Hyprland installation
    if config.hyprland_enabled {
        if HyprlandManager::detect_installation() {
            println!("Hyprland installation detected.");
            let _gen_cfg = HyprlandManager::generate_config(&config)?;
            println!("Hyprland config generated successfully.");
        } else {
            eprintln!("Warning: Hyprland is enabled but executable could not be found.");
        }
    }

    // 2. Manage visual setups
    ThemeManager::apply_theme(&config)?;
    WallpaperManager::apply_wallpaper(&config)?;

    // 3. Session initialization
    let mut session = SessionManager::new();
    session.start_session(&config)?;
    println!("Desktop session successfully initialized.");

    // 4. Build and log startup list
    let startup_cmds = StartupOrchestrator::build_commands(&config);
    println!("Startup applications list: {startup_cmds:?}");

    // 5. Connect and initialize core communication
    let ipc = IpcChannel::connect("/run/user/1000/daemon.sock")?;
    ipc.send(&IpcMessage::Command("desktop_init".to_string()))?;

    Ok(())
}
