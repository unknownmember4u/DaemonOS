#!/usr/bin/env bash
# ==============================================================================
# DaemonOS Workspace Run Script
# Compiles and runs the daemon-cli controller.
# ==============================================================================

set -euo pipefail

echo "==> Running DaemonOS CLI Controller..."
cargo run --package daemon-cli
