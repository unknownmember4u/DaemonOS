#!/usr/bin/env bash
# ==============================================================================
# DaemonOS Workspace Format Script
# Formats all workspace members or checks formatting rules.
# ==============================================================================

set -euo pipefail

CHECK_ONLY=false
if [ "${1:-}" = "--check" ]; then
    CHECK_ONLY=true
fi

if [ "$CHECK_ONLY" = true ]; then
    echo "==> Verifying workspace formatting..."
    cargo fmt --all -- --check
else
    echo "==> Formatting workspace codebase..."
    cargo fmt --all
fi

echo "==> Format complete!"
