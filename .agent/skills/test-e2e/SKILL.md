---
name: test-e2e
description: Führt einen vollständigen End-to-End-Systemtest über die gesamte Anwendungspipeline aus, prüft sBuffy/iBuffy-Konventionen sowie Fail-Fast und dokumentiert das Ergebnis in docs/e2e_test_report.md.
---

# SKILL: End-to-End (E2E) System- & Pipeline-Test (`test-e2e`)

> **Rolle & Einsatzzweck:**
> Dieser Skill wird vom **`tester`-Agenten** ausgeführt, um das Zusammenspiel aller Systemkomponenten (Eingabe/Ingest ➔ Verarbeitung/Inferenz ➔ Output/Persistenz/Clipboard ➔ UI/API-Reaktion) unter realitätsnahen Bedingungen Ende-zu-Ende zu verifizieren.

---

## 1. Testvorbereitung, Ausführung & Fail-Fast-Regel

1. **Referenz-Fixtures laden:** Stelle definierte Test-Eingabedaten (z. B. Referenz-Audiodatei, Referenz-Payload oder Test-Datenbank) bereit.
2. **System-Zustand sichern:** Falls System-Ressourcen wie die Zwischenablage (Clipboard) genutzt werden, sichere deren Inhalt vor dem Test und stelle ihn im `tearDown` wieder her.
3. **E2E-Suite starten:**
   ```bash
   pytest tests/e2e/ -v --tb=short
   ```
4. **Fail-Fast im 4-Agenten-Workflow:**
   Schlägt ein E2E-Test fehl, bricht der `tester`-Agent das parallel laufende Security-Audit des `security-specialist` **sofort ab (`manage_subagents(Action="kill")`)** und meldet den Fehler direkt an den `coder`-Agenten.

---

## 2. Verifikationspunkte entlang der E2E-Kette

- **Schritt 1 (Data Ingest & Buffering):** Sauberes Einlesen und Puffern der Eingangsdaten unter Einhaltung der Puffer-Konventionen (`sBuffy` für String-Puffer, `iBuffy` für Schleifenzähler).
- **Schritt 2 (Kernverarbeitung / Lokale Inferenz):** Korrekte, deterministische Verarbeitung ohne unerlaubte externe Netzwerkzugriffe.
- **Schritt 3 (Output, Persistenz / Clipboard & UI/API-Status):** Verifikation, dass das Endergebnis exakt mit der erwarteten Referenz übereinstimmt und der Zielzustand (`Ready` / `HTTP 200`) erreicht wird.

---

## 3. Verbindlicher E2E-Testbericht (`docs/e2e_test_report.md`)

Erstelle oder aktualisiere nach jedem Durchlauf zwingend die Datei `docs/e2e_test_report.md`.

> **Wichtig (Datenschutz & Public-Repo-Konformität):** In den Berichten dürfen **keine persönlichen Dateipfade** (`/home/<username>/...` oder `C:\Users\<username>\...`) aufgeführt werden. Verwende ausschließlich Komponenten- und Schrittbezeichnungen!

```markdown
# E2E System-Testbericht: whisper-pill

- **Datum & Uhrzeit:** [YYYY-MM-DD HH:MM:SS]
- **Plattform:** [Linux / Windows / macOS]
- **Konfiguration / Modell:** [Verwendete Test-Konfiguration]
- **Gesamtergebnis:** `[PASSED / FAILED]`

## 1. Pipeline-Schritte & Latenzen
| Schritt | Komponente | Status | Dauer (ms) |
| :--- | :--- | :--- | :--- |
| 1. Eingabe / Ingest | `[z. B. InputHandler / AudioRecorder]` | `OK / FAIL` | `...` |
| 2. Kernverarbeitung / Engine | `[z. B. CoreEngine / TranscriptionEngine]` | `OK / FAIL` | `...` |
| 3. Ausgabe / Persistenz / Clipboard | `[z. B. OutputService / ClipboardManager]` | `OK / FAIL` | `...` |
| 4. UI- / API-Statuswechsel | `[z. B. UIController / APIRouter]` | `OK / FAIL` | `...` |

## 2. Ergebnis-Genauigkeit & Validierung
- **Erwartete Referenz:** `"..."`
- **Tatsächliches Ergebnis:** `"..."`
- **Übereinstimmung / Qualität:** `... %`

## 3. Fazit & Fehleranalyse
`[Detaillierte Bestätigung der fehlerfreien E2E-Pipeline oder Ursachenanalyse bei Fehlern]`
```
