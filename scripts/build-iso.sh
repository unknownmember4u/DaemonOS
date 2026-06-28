#!/usr/bin/env bash
set -e

echo "==> Building DaemonOS workspace in release mode..."
cargo build --release

echo "==> Copying compiled binaries to daemon-iso/airootfs/usr/bin/"
mkdir -p daemon-iso/airootfs/usr/bin
# Copy all executable daemon binaries (ignore extensions like .d or .rlib)
find target/release -maxdepth 1 -type f -executable -name "daemon-*" -exec cp {} daemon-iso/airootfs/usr/bin/ \;

echo "==> Building bootable ISO with mkarchiso..."
echo "Note: This step requires root privileges."
sudo mkarchiso -v -w /tmp/archiso-tmp -o out daemon-iso

echo "==> Build complete! The ISO is located in the 'out/' directory."
