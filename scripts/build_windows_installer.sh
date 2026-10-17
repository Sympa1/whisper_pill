#!/usr/bin/env bash
# Build-Skript fuer den Whisper Pill Installations-Assistenten (Nativer Slint-Wizard & Windows .exe Cross-Build)

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="${REPO_ROOT}/dist"
mkdir -p "${DIST_DIR}"

echo "======================================================================"
echo "Whisper Pill – Setup-Wizard & Windows .exe Builder"
echo "======================================================================"

# 1. Nativen Linux Slint-Installations-Wizard bauen
echo "[1/2] Kompiliere nativen Rust/Slint Installations-Wizard (Linux)..."
cargo build --release --manifest-path "${REPO_ROOT}/app/Cargo.toml"
cp "${REPO_ROOT}/app/target/release/whisper_pill" "${DIST_DIR}/whisper_pill_installer"
chmod +x "${DIST_DIR}/whisper_pill_installer"
echo "  ✔ Linux Setup-Wizard bereit: dist/whisper_pill_installer"

# 2. Windows .exe Cross-Build (via cargo-xwin im Benutzerverzeichnis oder mingw-w64)
XWIN_BIN=""
if [ -x "${DIST_DIR}/.tools/cargo-xwin" ]; then
    XWIN_BIN="${DIST_DIR}/.tools/cargo-xwin"
elif command -v cargo-xwin >/dev/null 2>&1; then
    XWIN_BIN="cargo-xwin"
fi

if [ -n "${XWIN_BIN}" ] && rustup target list --installed | grep -q "x86_64-pc-windows-msvc"; then
    echo "[2/2] Kompiliere Windows .exe via cargo-xwin (x86_64-pc-windows-msvc)..."
    "${XWIN_BIN}" xwin build --release --target x86_64-pc-windows-msvc --manifest-path "${REPO_ROOT}/app/Cargo.toml"
    cp "${REPO_ROOT}/app/target/x86_64-pc-windows-msvc/release/whisper_pill.exe" "${DIST_DIR}/whisper_pill.exe"
    cp "${REPO_ROOT}/app/target/x86_64-pc-windows-msvc/release/whisper_pill.exe" "${DIST_DIR}/Whisper_Pill_Setup_Wizard.exe"
    echo "  ✔ Windows Hauptanwendung erzeugt:  dist/whisper_pill.exe"
    echo "  ✔ Windows Setup-Wizard erzeugt:    dist/Whisper_Pill_Setup_Wizard.exe"
elif command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1 && rustup target list --installed | grep -q "x86_64-pc-windows-gnu"; then
    echo "[2/2] Kompiliere Windows .exe via MinGW (x86_64-pc-windows-gnu)..."
    cargo build --release --target x86_64-pc-windows-gnu --manifest-path "${REPO_ROOT}/app/Cargo.toml"
    cp "${REPO_ROOT}/app/target/x86_64-pc-windows-gnu/release/whisper_pill.exe" "${DIST_DIR}/whisper_pill.exe"
    cp "${REPO_ROOT}/app/target/x86_64-pc-windows-gnu/release/whisper_pill.exe" "${DIST_DIR}/Whisper_Pill_Setup_Wizard.exe"
    echo "  ✔ Windows Setup-Wizard erzeugt:    dist/Whisper_Pill_Setup_Wizard.exe"
fi
