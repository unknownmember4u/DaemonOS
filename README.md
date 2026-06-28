# DaemonOS

DaemonOS is a modern, modular, performance-oriented open-source operating system environment built entirely in Rust. It utilizes a highly decoupled, message-driven micro-service architecture to coordinate window management, panels, app launchers, notification servers, and configuration settings.

## System Architecture

DaemonOS components are decoupled by design. No component directly depends on another user interface component; all coordination is processed via the shared inter-process communication layer (`daemon-ipc`).

```
          [ daemon-cli ] (CLI Controller)
                 │
                 ▼
          [ daemon-core ] (Core Orchestrator)
                 │
                 ▼
           [ daemon-ipc ] (IPC message definitions & broker)
           /     │      \
          /      │       \
         ▼       ▼        ▼
  [daemon-wm] [daemon-panel] [daemon-launcher] ... (Desktop Services)
```

## Workspace Crate Registry

This Cargo workspace includes the following crates:

*   **`daemon-config`**: Shared configuration library providing parsing and validation of system config.
*   **`daemon-ipc`**: Shared IPC protocol library establishing communication channels and message schemas.
*   **`daemon-core`**: Core library managing workspace component initialization and state.
*   **`daemon-cli`**: System command-line interface utility.
*   **`daemon-wm`**: Desktop window manager.
*   **`daemon-panel`**: System status bar and panel.
*   **`daemon-launcher`**: Application launcher overlay.
*   **`daemon-settings`**: Configuration dashboard GUI.
*   **`daemon-notify`**: Desktop notification daemon.
*   **`daemon-lock`**: System screen locker.
*   **`daemon-installer`**: OS installation wizard.

## Development Standards

To maintain production-grade quality, the project enforces:
*   **No Unsafe Code**: The entire workspace forbids unsafe blocks (`unsafe_code = "forbid"`).
*   **No Panics in Production**: The compiler denies use of `unwrap()` via clippy configuration (`unwrap_used = "deny"`).
*   **Consistent Styling**: Enforced automatically by `cargo fmt`.

## Getting Started

### Compile the entire workspace

```bash
cargo build
```

### Run check and linters

```bash
cargo fmt --check
cargo clippy --all-targets
```
