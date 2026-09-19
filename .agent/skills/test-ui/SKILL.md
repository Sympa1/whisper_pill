---
name: test-ui
description: Führt automatisierte GUI-, State-Machine- und Interaktionstests für die PySide6-Pill durch und dokumentiert die Ergebnisse in docs/ui_test_report.md.
---

# UI & State Machine Test Workflow

Führe diesen Workflow aus, um die grafische PySide6-Benutzeroberfläche, Fensterattribute, Signal-Slot-Verbindungen und Zustandsübergänge automatisiert abzusichern.

## 1. GUI-Testsuite ausführen
* Führe PyTest auf den UI-Tests (unter Nutzung von `pytest-qt` bzw. `QTest`) aus:
  ```bash
  pytest tests/test_ui.py -v --tb=short
  
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
