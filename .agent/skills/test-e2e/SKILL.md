---
name: test-e2e
description: Führt einen vollständigen End-to-End-Systemtest (Audio-Input -> Whisper-Inferenz -> Clipboard -> UI) aus und dokumentiert das Ergebnis in docs/e2e_test_report.md.
---

# End-to-End System Test Workflow

Führe bei Aktivierung dieses Skills die vollständige Testkette unter realitätsnahen Bedingungen aus:

## 1. Testvorbereitung & Pipeline-Start
1. **Audio-Quelle bereitstellen:**
   - Lade eine definierte Referenz-Audiodatei (z. B. `tests/fixtures/sample_de.wav`) mit bekanntem deutschem Transkript.
2. **System-Clipboard sichern:**
   - Lies den aktuellen Inhalt der Zwischenablage aus und sichere ihn temporär.
3. **Ausführung der Integrationssuite:**
   - Starte den Testlauf via PyTest:
     ```bash
     pytest tests/e2e/test_full_pipeline.py -v --tb=short
     ```

## 2. Verifikationspunkte (Kette prüfen)
* **Audio-Ingest:** Verifiziere, dass `sounddevice` die Samples sauber puffert (Überprüfung auf `sBuffy` für Strings und `iBuffy` für Zähler in internen Schleifen).
* **Lokale Whisper-Inferenz:** Prüfe, ob `faster-whisper` ohne Netzwerkzugriff den erwarteten Text inklusive Kommasetzung liefert.
* **OS-Clipboard & UI:** Prüfe via `pyperclip`, ob der identische Text in der System-Zwischenablage ankommt und das Pill-Fenster den Status `Ready / Copied` anzeigt.

## 3. Dokumentations-Bericht generieren
Erstelle oder aktualisiere nach jedem Durchlauf zwingend die Datei `docs/e2e_test_report.md` nach folgendem Standard:

```markdown
# E2E System-Testbericht: whisper-pill

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
