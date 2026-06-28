# DaemonOS Project Roadmap

This roadmap outlines the planned development phases for the DaemonOS desktop environment components.

## Phase 1: Workspace & Core Architecture (Current)
- [x] Configure Cargo workspace with modular crate boundaries.
- [x] Integrate workspace-wide lint controls (`forbid(unsafe_code)`, `deny(clippy::unwrap_used)`).
- [x] Provision shared crates: `daemon-core`, `daemon-config`, `daemon-ipc`.
- [x] Establish standard documentation and issue workflows.

## Phase 2: IPC & Protocol Layer
- [ ] Implement Unix domain socket broker in `daemon-ipc`.
- [ ] Develop serialize/deserialize protocols for window management and notification event payloads.
- [ ] Create mock tests simulating message dispatch and receipt.

## Phase 3: Desktop Environment & Window Integration
- [ ] Implement `daemon-desktop` integrating with Hyprland display server configurations and themes.
- [ ] Develop tiling/floating window placement desktop integrations.
- [ ] Connect `daemon-desktop` to `daemon-ipc` to accept focus/kill commands.

## Phase 4: UI Shell Components
- [ ] Build status-bar widgets in `daemon-panel` (clock, workspaces, network, battery).
- [ ] Implement application indexing and search in `daemon-launcher`.
- [ ] Design screen locker rendering loop and PAM authentication integration in `daemon-lock`.

## Phase 5: Distribution & Installer
- [ ] Package system components for standard distributions in `daemon-packages/`.
- [ ] Create system installation scripts in `daemon-installer`.
- [ ] Automate ISO generation process in `daemon-iso/`.
