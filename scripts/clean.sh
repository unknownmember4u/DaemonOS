#!/usr/bin/env bash
# ==============================================================================
# DaemonOS Workspace Clean Script
# Cleans target build folders and log files.
# ==============================================================================

set -euo pipefail

echo "==> Cleaning workspace target files..."
cargo clean

echo "==> Removing temporary log files..."
rm -f *.log Cargo.lock.log

echo "==> Clean complete!"
