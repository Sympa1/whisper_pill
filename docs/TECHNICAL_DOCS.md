# Technische Dokumentation: Whisper Pill

> **Zentrale technische Entwickler- und Architektur-Referenz für Whisper Pill (Rust, Slint UI, C++ whisper.cpp Bindings).**

Eine ausführliche Schichten- und Modulspezifikation inklusive aller fünf Mermaid-Architektur- und UML-Diagramme findest du in der Hauptdokumentation:
👉 **[Technische Architektur- & Moduldokumentation](architecture/technical_documentation.md)**

---

## Schnellübersicht der Architektur

- **Sprachen & Kern-Frameworks:** Rust (Edition 2024), Slint UI (`1.18`), `whisper-rs` (`whisper.cpp`), `cpal` (Audio-Ingest), `arboard` (Clipboard).
- **Entkopplung:** Slint Event-Loop (Main Thread), Audio-Stream-Thread (`cpal`), Transkriptions-Worker-Thread (`whisper.cpp`), asynchrone D-Bus-Runtime (`tokio`).
- **RAM-Zeroing:** Unmittelbare Überschreibung aller aufgenommenen Audiodaten mit Nullen (`0.0`) nach der Transkription.
- **Zero-Download-Betrieb:** 100 % Offline-Inferenz; keine Netzwerkaufrufe während des regulären Diktats.

---

## Diagramm-Übersicht

1. **System- & Schichtenarchitektur:** `Presentation` (Slint AppWindow, InstallerWindow) ➔ `Orchestration` (AppOrchestrator) ➔ `Domain` (AudioRecorder, VadDetector, WhisperEngine, TextAccumulator) ➔ `Platform` (HotkeyManager, PillTray, KWinPlatform, AutostartPlatform, InstallerPlatform, ClipboardManager).
2. **UML-Klassendiagramm:** Kapselung und Schnittstellen aller Kernkomponenten.
3. **Sequenzdiagramm:** Hotkey-Auslösung, Pegel-Übertragung, VAD-Stilleerkennung, Inferenz, RAM-Bereinigung und Zwischenablage.
4. **Zustandsautomat (State Machine):** `Hidden` ➔ `Idle` ➔ `Recording` ➔ `Transcribing` ➔ `Ready` (8s Auto-Dismiss) ➔ `Expanded` / `Settings` / `Installer`.
5. **Daten- & Schemamodell:** `AppConfig`, `InstallOptions`, `AudioBuffer`, `TranscriptionResult`.

---

## Verifikations- und Test-Befehle

```bash
# Rust Unit- und Integrationstests (46 Tests)
cargo test --manifest-path app/Cargo.toml

# Rust Linter & Clippy (0 Warnungen)
cargo clippy --manifest-path app/Cargo.toml --all-targets -- -D warnings

# Python-Prototyp Tests (19 Tests)
./prototype/.venv/bin/pytest tests/ -v --tb=short

# Automatisierter Zero-Download & Security Scan (0 Befunde)
python3 .agent/skills/security-audit/scripts/check_downloads.py .
```
