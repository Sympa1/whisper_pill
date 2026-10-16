#!/usr/bin/env bash
# ==============================================================================
# Whisper Pill - AppImage Build-Skript
# Erstellt ein autarkes, portables Linux-AppImage aus der Rust-Anwendung.
# ==============================================================================

set -euo pipefail

# Arbeitsverzeichnis ermitteln (Wurzelverzeichnis des Repositories)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "=== Whisper Pill AppImage Erstellung gestartet ==="
echo "Projekt-Verzeichnis: ${ROOT_DIR}"

# 1. Release-Kompilierung der Rust-Anwendung sicherstellen
echo "--> Baue optimierte Release-Binärdatei (cargo build --release)..."
cargo build --release --manifest-path "${ROOT_DIR}/app/Cargo.toml"

BINARY_SOURCE="${ROOT_DIR}/app/target/release/whisper_pill"
if [ ! -f "${BINARY_SOURCE}" ]; then
    echo "FEHLER: Binärdatei wurde nicht gefunden: ${BINARY_SOURCE}" >&2
    exit 1
fi

# 2. Ausgabe- und Staging-Verzeichnisse vorbereiten
DIST_DIR="${ROOT_DIR}/dist"
APP_DIR="${DIST_DIR}/AppDir"

echo "--> Bereite AppDir-Struktur vor..."
rm -rf "${APP_DIR}"
mkdir -p "${APP_DIR}/usr/bin"
mkdir -p "${APP_DIR}/usr/share/applications"
mkdir -p "${APP_DIR}/usr/share/icons/hicolor/scalable/apps"
mkdir -p "${APP_DIR}/usr/share/icons/hicolor/256x256/apps"

# 3. Binärdatei in AppDir kopieren
cp "${BINARY_SOURCE}" "${APP_DIR}/usr/bin/whisper_pill"
chmod +x "${APP_DIR}/usr/bin/whisper_pill"

# 4. Desktop-Entry kopieren
DESKTOP_SOURCE="${ROOT_DIR}/packaging/whisper-pill.desktop"
cp "${DESKTOP_SOURCE}" "${APP_DIR}/whisper-pill.desktop"
cp "${DESKTOP_SOURCE}" "${APP_DIR}/usr/share/applications/whisper-pill.desktop"

# 5. Icons sicherstellen und kopieren
SVG_ICON="${ROOT_DIR}/packaging/whisper-pill.svg"
PNG_ICON="${ROOT_DIR}/packaging/whisper-pill.png"

if [ ! -f "${PNG_ICON}" ]; then
    echo "--> Generiere PNG-Icons via packaging/generate_icons.py..."
    "${ROOT_DIR}/prototype/.venv/bin/python" "${ROOT_DIR}/packaging/generate_icons.py"
fi

cp "${SVG_ICON}" "${APP_DIR}/whisper-pill.svg"
cp "${SVG_ICON}" "${APP_DIR}/usr/share/icons/hicolor/scalable/apps/whisper-pill.svg"

cp "${PNG_ICON}" "${APP_DIR}/whisper-pill.png"
cp "${PNG_ICON}" "${APP_DIR}/usr/share/icons/hicolor/256x256/apps/whisper-pill.png"

# 6. AppRun-Starter-Skript erstellen
echo "--> Erstelle AppRun..."
cat << 'EOF' > "${APP_DIR}/AppRun"
#!/bin/sh
set -e

# Ermittle das Basisverzeichnis des gemounteten AppImages
HERE="$(dirname "$(readlink -f "${0}")")"

export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH:-}"
export XDG_DATA_DIRS="${HERE}/usr/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"

# Whisper Pill Hauptanwendung starten
exec "${HERE}/usr/bin/whisper_pill" "$@"
EOF
chmod +x "${APP_DIR}/AppRun"

# 7. appimagetool lokalisieren
APPIMAGETOOL_BIN=""
if [ -n "${APPIMAGETOOL:-}" ] && [ -x "${APPIMAGETOOL}" ]; then
    APPIMAGETOOL_BIN="${APPIMAGETOOL}"
elif command -v appimagetool >/dev/null 2>&1; then
    APPIMAGETOOL_BIN="$(command -v appimagetool)"
elif [ -x "${HOME}/.local/bin/appimagetool" ]; then
    APPIMAGETOOL_BIN="${HOME}/.local/bin/appimagetool"
elif [ -x "${HOME}/.local/bin/appimagetool-x86_64.AppImage" ]; then
    APPIMAGETOOL_BIN="${HOME}/.local/bin/appimagetool-x86_64.AppImage"
fi

if [ -z "${APPIMAGETOOL_BIN}" ]; then
    echo "FEHLER: appimagetool wurde auf dem System nicht gefunden!" >&2
    echo "Bitte installieren Sie appimagetool oder setzen Sie die Umgebungsvariable APPIMAGETOOL." >&2
    exit 1
fi

echo "--> Verwende appimagetool: ${APPIMAGETOOL_BIN}"

# 8. AppImage paketieren
OUTPUT_APPIMAGE="${DIST_DIR}/Whisper_Pill-x86_64.AppImage"
rm -f "${OUTPUT_APPIMAGE}"

echo "--> Paketierung wird durchgefuehrt..."
ARCH=x86_64 "${APPIMAGETOOL_BIN}" --no-appstream "${APP_DIR}" "${OUTPUT_APPIMAGE}"

chmod +x "${OUTPUT_APPIMAGE}"
echo "=== AppImage erfolgreich erstellt! ==="
echo "Zieldatei: ${OUTPUT_APPIMAGE}"
ls -lh "${OUTPUT_APPIMAGE}"
