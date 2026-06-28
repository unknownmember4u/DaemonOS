#!/usr/bin/env bash
# ==============================================================================
# DaemonOS Workspace Bootstrap Script
# Enforces installation of correct compiler toolchains and components.
# ==============================================================================

set -euo pipefail

echo "==> Bootstrapping DaemonOS workspace..."

if ! command -v rustup &> /dev/null; then
    echo "Error: rustup is not installed. Please install it first from https://rustup.rs"
    exit 1
fi

echo "==> Verifying rustup toolchain..."
rustup toolchain install stable --component rustfmt --component clippy

echo "==> Installing workspace analysis utilities..."
# Check and notify or install optionally needed tools (e.g. cargo-deny)
if ! command -v cargo-deny &> /dev/null; then
    echo "Tip: Consider installing 'cargo-deny' to audit license/dependency graphs:"
    echo "     cargo install --locked cargo-deny"
fi

echo "==> Workspace bootstrapped successfully!"
