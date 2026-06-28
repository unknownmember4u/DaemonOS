#!/usr/bin/env bash
# ==============================================================================
# DaemonOS Workspace Lint Script
# Runs clippy and static analysis checks on the workspace.
# ==============================================================================

set -euo pipefail

echo "==> Running clippy lints..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

if command -v cargo-deny &> /dev/null; then
    echo "==> Auditing license configuration..."
    cargo deny check licenses
fi

echo "==> Lints and audit checks complete!"
