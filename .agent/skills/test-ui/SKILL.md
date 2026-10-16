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
  ```

## 2. Dokumentations-Bericht generieren
Erstelle oder aktualisiere nach jedem Durchlauf zwingend die Datei `docs/ui_test_report.md` nach folgendem Standard.

> **Wichtig (Datenschutz & Public-Repo-Konformität):** In den Berichten dürfen **keine Dateipfade** aufgeführt werden. Verwende ausschließlich Komponenten- und Testnamen.

```markdown
# UI & State Machine Testbericht: whisper-pill

- **Datum & Uhrzeit:** [Aktueller Timestamp]
- **Plattform:** [Linux / Windows]
- **Komponente:** Pill-UI (PillWindow, WaveformWidget)
- **Gesamtergebnis:** [PASSED / FAILED]

## 1. Ausgefuehrte UI-Test-Suiten
| Test | Status | Beschreibung |
| :--- | :--- | :--- |
| `test_waveform_widget_levels` | PASSED | Animierte Lautstaerke-Balken |
| `test_pill_window_flags_and_initial_state` | PASSED | Fensterattribute (Frameless, Always-On-Top, Transparenz, IDLE) |
| `test_pill_window_state_transitions` | PASSED | State-Machine (IDLE -> REC -> PROCESSING -> READY -> EXPANDED -> IDLE) |
| `test_pill_recopy_edited_text` | PASSED | Text-Kopieren aus Editor |

## 2. Zusammenfassung
- **Test-Modus:** Headless via `QT_QPA_PLATFORM=offscreen`.
- **Zustandsmaschine:** Alle Übergänge arbeiten sauber mit Signalen und Slots.
- **Fazit:** [Zusammenfassung und Ergebnis]
```
