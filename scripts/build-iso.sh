#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
WORK_DIR="${PROJECT_DIR}/daemon-iso/work"
OUT_DIR="${PROJECT_DIR}/out"
PROFILE_DIR="${PROJECT_DIR}/daemon-iso"

echo "==> Building DaemonOS workspace in release mode..."
cargo build --release --manifest-path="${PROJECT_DIR}/Cargo.toml"

echo "==> Copying compiled binaries to daemon-iso/airootfs/usr/bin/"
mkdir -p "${PROFILE_DIR}/airootfs/usr/bin"
find "${PROJECT_DIR}/target/release" -maxdepth 1 -type f -executable -name "daemon-*" \
    -exec cp {} "${PROFILE_DIR}/airootfs/usr/bin/" \;

echo "==> Cleaning stale work directory..."
if [ -d "${WORK_DIR}" ]; then
    sudo rm -rf "${WORK_DIR}"
fi

echo "==> Building bootable ISO with mkarchiso..."
echo "Note: This step requires root privileges."
sudo mkarchiso -v -w "${WORK_DIR}" -o "${OUT_DIR}" "${PROFILE_DIR}"

echo "==> Verifying live user 'daemonos' in built image..."
if sudo grep -q "^daemonos:" "${WORK_DIR}/x86_64/airootfs/etc/passwd" 2>/dev/null; then
    echo "  ✓ daemonos user found in passwd"
else
    echo "  ✗ WARNING: daemonos user NOT found in passwd"
fi

if sudo test -d "${WORK_DIR}/x86_64/airootfs/home/daemonos" 2>/dev/null; then
    echo "  ✓ /home/daemonos exists"
else
    echo "  ✗ WARNING: /home/daemonos does NOT exist"
fi

if sudo grep -q "autologin daemonos" "${WORK_DIR}/x86_64/airootfs/etc/systemd/system/getty@tty1.service.d/autologin.conf" 2>/dev/null; then
    echo "  ✓ autologin.conf targets daemonos"
else
    echo "  ✗ WARNING: autologin.conf does NOT target daemonos"
fi

echo "==> Build complete! The ISO is located in '${OUT_DIR}/'."
