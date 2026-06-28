# DaemonOS

DaemonOS is a modern, performance-oriented Linux-based desktop operating environment components written in Rust. It provides a modular, lightweight, and customizable system of window management, configuration utility, launcher, and system panel tools.

## Repository Structure

This repository is structured as a Cargo workspace containing the following crates:

- **`daemon-core`**: Core library containing shared utilities, protocols, and configuration parsing.
- **`daemon-cli`**: Command-line interface for controlling and interacting with DaemonOS services.
- **`daemon-wm`**: The core window manager for the desktop environment.
- **`daemon-panel`**: System status bar / panel providing clock, tray, workspaces switcher, and system monitors.
- **`daemon-launcher`**: Application launcher / runner.
- **`daemon-settings`**: GUI configuration center for system and theme settings.
- **`daemon-notify`**: Desktop notification daemon implementing the desktop notification specification.
- **`daemon-lock`**: Simple, secure screen locker.
- **`daemon-installer`**: System installation wizard/utility.

### Additional Folders

- **`docs/`**: Project documentation, architectural specs, and user manuals.
- **`assets/`**: Images, icons, wallpapers, and desktop entries.
- **`scripts/`**: Development and deployment utility scripts.
- **`daemon-packages/`**: Distribution-specific package files and recipes (e.g., PKGBUILD, debian control).
- **`daemon-iso/`**: Configuration files and build scripts for generating custom DaemonOS installation ISOs.

## Building and Running

### Prerequisites

You will need the Rust toolchain installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Make sure you have system dependencies installed (such as `x11`, `wayland`, `dbus`, `cairo`, `pango` development packages depending on the components build targets).

### Compile all workspace members

```bash
cargo build --release
```

### Running a specific component

```bash
cargo run --bin daemon-wm
```

## Contributing

Please review our contributing guidelines and use the provided issue/PR templates when submitting changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
