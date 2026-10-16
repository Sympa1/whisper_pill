#!/usr/bin/env bash
# ==============================================================================
# Whisper Pill - Modell-Download-Skript
# Laedt GGML-Modelle von HuggingFace sicher in den lokalen Cache (~/.cache/whisper_pill/).
# ==============================================================================

set -e

MODEL="${1:-small}"
CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/whisper_pill"
TARGET_FILE="${CACHE_DIR}/ggml-${MODEL}.bin"
DOWNLOAD_URL="https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-${MODEL}.bin"

echo "=================================================="
echo " Whisper Pill: Modell-Download"
echo " Modell:      ${MODEL} (ggml-${MODEL}.bin)"
echo " Zielordner:  ${CACHE_DIR}"
echo "=================================================="

# Zielverzeichnis erstellen falls nicht vorhanden
mkdir -p "${CACHE_DIR}"

if [ -f "${TARGET_FILE}" ]; then
    FILE_SIZE=$(du -h "${TARGET_FILE}" | cut -f1)
    echo "✔ Modell existiert bereits lokal: ${TARGET_FILE} (${FILE_SIZE})"
    read -p "Erneut herunterladen und ueberschreiben? (j/N): " -r CONFIRM
    if [[ ! $CONFIRM =~ ^[jJyY]$ ]]; then
        echo "Abgebrochen."
        exit 0
    fi
fi

echo "Lade ggml-${MODEL}.bin herunter..."

if command -v curl >/dev/null 2>&1; then
    curl -L --progress-bar -o "${TARGET_FILE}.tmp" "${DOWNLOAD_URL}"
elif command -v wget >/dev/null 2>&1; then
    wget --show-progress -O "${TARGET_FILE}.tmp" "${DOWNLOAD_URL}"
else
    echo "Fehler: Weder 'curl' noch 'wget' gefunden. Bitte installieren Sie eines der beiden Tools."
    exit 1
fi

# Temporäre Datei auf endgültigen Namen verschieben
mv "${TARGET_FILE}.tmp" "${TARGET_FILE}"

FINAL_SIZE=$(du -h "${TARGET_FILE}" | cut -f1)
echo "--------------------------------------------------"
echo "✔ Erfolgreich abgeschlossen!"
echo "  Datei: ${TARGET_FILE} (${FINAL_SIZE})"
echo "  Whisper Pill erkennt das Modell nun automatisch."
echo "=================================================="
