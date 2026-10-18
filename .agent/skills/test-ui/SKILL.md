---
name: test-ui
description: Führt automatisierte Headless-GUI-, State-Machine- und Interaktionstests (z. B. PySide6/Qt oder Web-UI) durch, beachtet die Fail-Fast-Regel und dokumentiert die Ergebnisse in docs/ui_test_report.md.
---

# SKILL: Automatisierte UI- & State-Machine-Tests (`test-ui`)

> **Rolle & Einsatzzweck:**
> Dieser Skill wird vom **`tester`-Agenten** genutzt, um grafische Benutzeroberflächen (z. B. PySide6/Qt-Widgets, Fensterattribute, Signal-Slot-Verbindungen und UI-Zustandsautomaten) headless und reproduzierbar zu testen.

---

## 1. Headless GUI-Testsuite ausführen & Fail-Fast-Regel

1. **Testlauf starten (Headless via `offscreen`):**
   ```bash
   QT_QPA_PLATFORM=offscreen pytest tests/test_ui.py -v --tb=short
   ```
2. **Fail-Fast-Kopplung im 4-Agenten-Workflow:**
   Schlägt ein UI- oder State-Machine-Test fehl, bricht der `tester`-Agent das parallel laufende Security-Audit des `security-specialist`-Agenten **sofort ab (`manage_subagents(Action="kill")`)** und meldet den Fehler direkt an den `coder`-Agenten zurück!

---

## 2. Prüfschwerpunkte für UI-Komponenten

- **Fensterattribute & Initialzustand:** Korrekte Flags (z. B. Frameless, Always-On-Top, Transparenz) und definierter Startzustand (`IDLE`).
- **Zustandsübergänge (State Machine):** Deterministische Übergänge über alle UI-Zustände (z. B. `IDLE -> RECORDING/ACTIVE -> PROCESSING -> READY -> EXPANDED -> IDLE`).
- **Signal-Slot- & Thread-Entkopplung:** Langlaufende Operationen blockieren niemals den UI-Event-Loop; Kommunikation erfolgt sauber über Signale/Events.
- **Interaktion & Zwischenablage:** Korrekte Übernahme von Benutzereingaben, Editor-Änderungen und Kopieraktionen.

---

## 3. Verbindlicher UI-Testbericht (`docs/ui_test_report.md`)

Erstelle oder aktualisiere nach jedem Durchlauf zwingend die Datei `docs/ui_test_report.md`.

> **Wichtig (Datenschutz & Public-Repo-Konformität):** In den Berichten dürfen **keine persönlichen Dateipfade** (`/home/<username>/...` oder `C:\Users\<username>\...`) aufgeführt werden. Verwende ausschließlich Komponenten- und Testnamen!

```markdown
# UI & State-Machine Testbericht: whisper-pill

- **Datum & Uhrzeit:** [YYYY-MM-DD HH:MM:SS]
- **Plattform & Modus:** [Linux / Windows – Headless (`QT_QPA_PLATFORM=offscreen`)]
- **Geprüfte UI-Komponenten:** [z. B. MainWindow / PillWindow, StatusWidget]
- **Gesamtergebnis:** `[PASSED / FAILED]`

## 1. Ausgeführte UI-Test-Suiten
| Testfall | Status | Geprüftes Verhalten |
| :--- | :--- | :--- |
| `test_ui_initial_flags_and_state` | `PASSED / FAILED` | Fensterattribute & Initialzustand (`IDLE`) |
| `test_ui_state_machine_transitions` | `PASSED / FAILED` | Vollständiger Zustandszyklus der UI |
| `test_ui_user_interaction_and_copy` | `PASSED / FAILED` | Benutzerinteraktion, Editor & Clipboard-Übergabe |

## 2. Zusammenfassung
- **Zustandsmaschine:** Alle Zustandsübergänge und Signal-Slot-Verbindungen arbeiten fehlerfrei.
- **Fazit:** `[Zusammenfassung des UI-Testlaufs]`
```
