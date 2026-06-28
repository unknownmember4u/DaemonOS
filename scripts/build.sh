#!/usr/bin/env bash
# ==============================================================================
# DaemonOS Workspace Build Script
# Compiles workspace members.
# ==============================================================================

set -euo pipefail

MODE=${1:-"release"}

echo "==> Building DaemonOS workspace in '${MODE}' mode..."

if [ "$MODE" = "release" ]; then
    cargo build --workspace --release
else
    cargo build --workspace
fi

echo "==> Build complete!"
