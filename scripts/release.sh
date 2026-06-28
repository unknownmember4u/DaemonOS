#!/usr/bin/env bash
# ==============================================================================
# DaemonOS Workspace Release Packaging Script
# Compiles release binaries and bundles package distributions.
# ==============================================================================

set -euo pipefail

VERSION=${1:-"0.1.0"}

echo "==> Building and packaging release v${VERSION}..."

cargo build --workspace --release

# Placeholder packaging steps (e.g. creating tarballs or deb packages)
mkdir -p target/dist
tar -czf "target/dist/daemonos-cli-v${VERSION}.tar.gz" -C target/release daemon-cli

echo "==> Package distribution generated successfully at target/dist/"
