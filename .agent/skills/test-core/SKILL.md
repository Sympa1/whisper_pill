---
name: test-core
description: Führt headless Unit- und Integrationstests für Audiopuffer, Inferenz-Engine und Konfiguration aus und generiert docs/unit_test_report.md.
---

# Test Core Workflow (Headless)

Führe bei Aktivierung folgende Schritte headless aus:

## 1. Testumgebung & Testläufe
* Führe PyTest auf den Core-Modulen mit Zeitmessung und kompaktem Trace aus:
  ```bash
  pytest tests/test_audio.py tests/test_transcribe.py tests/test_config.py -v --tb=short --durations=5
  
## 2. Dokumentations-Bericht generieren
Erstelle oder aktualisiere nach jedem Durchlauf zwingend die Datei `docs/e2e_test_report.md` nach folgendem Standard:

```markdown
# test-core System-Testbericht: whisper-pill

- **Datum & Uhrzeit:** [Aktueller Timestamp]
- **Plattform:** [Linux / Windows]
- **Modell:** Whisper [base / small]
- **Gesamtergebnis:** [PASSED / FAILED]

## Testschritte & Latenzen
| Schritt | Komponente | Status | Dauer (ms) |
| :--- | :--- | :--- | :--- |
| 1. Audio-Ingest | sounddevice / numpy | OK / FAIL | ... |
| 2. Lokale Transkription | faster-whisper | OK / FAIL | ... |
| 3. Zwischenablage | pyperclip | OK / FAIL | ... |
| 4. UI-Statuswechsel | PySide6 Pill | OK / FAIL | ... |

## Transkriptions-Genauigkeit
- **Referenz-Text:** "..."
- **Erkannter Text:** "..."
- **Match-Rate / WER:** ... %

## Fazit & Fehleranalyse
[Detaillierte Fehlermeldungen oder Bestätigung der fehlerfreien Pipeline]
