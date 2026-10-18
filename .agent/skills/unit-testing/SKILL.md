---
name: unit-testing
description: Verbindlicher QA-Skill für den tester-Agenten (Core/Unit-Tests). Führt headless Unit- und Integrationstests sowie Coverage-Messungen aus, setzt die Fail-Fast-Abbruchregel für das parallele Security-Audit durch und generiert docs/unit_test_report.md.
---

# SKILL: Automatisierte Core- & Unit-Tests, Fail-Fast-Gate & Reporting (`unit-testing`)

> **Rolle & Einsatzzweck:**
> Dieser Skill wird vom **`tester`-Agenten** (sowie zur Pflicht-Selbstprüfung vom `coder`-Agenten nach jedem Schritt) ausgeführt, um Kernmodule, Services und Konfigurationen headless zu testen und das Ergebnis in `docs/unit_test_report.md` zu dokumentieren.

---

## 1. Fail-Fast-Protokoll im 4-Agenten-Workflow (KRITISCH)

Sobald der `coder`-Agent seine Arbeit im Branch beendet hat, starten der **`tester`-Agent** und der **`security-specialist`-Agent** parallel. Dabei gilt folgende **verbindliche Fail-Fast-Regel**:

1. **Sofortiger Abbruch des Security-Audits bei Testfehlern:**
   Stellt der `tester`-Agent fest, dass:
   - ein bestehender oder neuer Unit-/Integrationstest fehlschlägt (`Exit-Code != 0`),
   - ein Syntax-, Import- oder Typfehler die Testausführung verhindert, oder
   - für neu implementierte Logik Pflicht-Unit-Tests fehlen,
   dann wird das parallel laufende **Security-Audit des `security-specialist`-Agenten sofort abgebrochen** (`manage_subagents(Action="kill", ConversationIds=["<security_specialist_id>"])`).
2. **Direkte Rückgabe an den `coder`:**
   Der Fehlerbericht geht ohne Umwege **direkt zurück an den `coder`-Agenten**.
3. **Absolutes Verbot der Test-Manipulation:**
   Bestehende Tests dürfen niemals gelöscht, übersprungen (`@skip`) oder in ihren Assertions abgeschwächt werden.

---

## 2. Standard-Befehle zur Testausführung (Headless)

```bash
# A) Whisper-Pill Test-Befehle (Python-Prototyp & Rust-App):
./prototype/.venv/bin/pytest tests/ -v --tb=short --durations=5
cargo test --manifest-path app/Cargo.toml

# B) Fallback mit Python Standard-Unittest:
python3 -m unittest discover -s tests -p "test_*.py" -v

# C) Messung der Codeabdeckung (Code Coverage):
pytest --cov=prototype --cov-report=term-missing tests/
```

---

## 3. Qualitätskriterien für Unit-Tests

1. **AAA-Struktur (Arrange – Act – Assert):** Klare Dreiteilung jedes Tests mit sprechendem englischen Methodennamen und deutschem Docstring (fachliches *Warum*).
2. **Konventionsprüfung (`sBuffy` / `iBuffy`):** Verifiziere, dass temporäre String-Puffer (`sBuffy`) und Schleifenzähler (`iBuffy`) sowohl im Produktions- als auch im Testcode sauber eingehalten werden.
3. **Isolation & Mocking:** Externe Seiteneffekte (Netzwerk, Hardware, echte Datenbanken, System-Clipboard) über `unittest.mock.patch` oder In-Memory-Fakes isolieren.

---

## 4. Verbindlicher Testbericht (`docs/unit_test_report.md`)

Erstelle oder aktualisiere nach jedem erfolgreichen Durchlauf zwingend die Datei `docs/unit_test_report.md`.

> **Wichtig (Datenschutz & Public-Repo-Konformität):** In den Berichten dürfen **keine persönlichen Systempfade** (wie `/home/<username>/...` oder `C:\Users\<username>\...`) aufgeführt werden. Verwende stattdessen ausschließlich sprechende Modul- und Komponentennamen!

```markdown
# Unit- & Core-Testbericht: whisper-pill

- **Datum & Uhrzeit:** [YYYY-MM-DD HH:MM:SS]
- **Plattform:** [Linux / Windows / macOS]
- **Geprüfte Komponenten:** [Komponentenübersicht ohne Dateipfade]
- **Gesamtergebnis:** `[PASSED / FAILED]`

## 1. Ausgeführte Test-Suiten & Latenzen
| Schritt / Suite | Geprüfte Komponente | Status | Dauer (ms) |
| :--- | :--- | :--- | :--- |
| 1. Core- & Service-Logik | `[z. B. CoreService / DataBuffer]` | `OK / FAIL` | `...` |
| 2. Konfiguration & Validierung | `[z. B. ConfigManager]` | `OK / FAIL` | `...` |
| 3. Fehler- & Grenzwerte (Edge Cases) | `[z. B. ValidationEngine]` | `OK / FAIL` | `...` |

## 2. Abdeckung & Fazit
- **Code Coverage:** `... %`
- **Fail-Fast-Status:** `[Alle Tests grün – Security-Audit läuft regulär weiter / Fehler -> Security-Audit gestoppt]`
- **Fazit & Fehleranalyse:** `[Zusammenfassung des Testlaufs]`
```
