# whisper-pill

> **Minimalistisches, datenschutzfokussiertes Desktop-Diktier-Utility für Linux und Windows – 100 % lokal, quelloffen und kostenlos.**

`whisper-pill` ist eine freie Open-Source-Alternative zu kommerziellen Diktierwerkzeugen wie Whisperbar. Die Anwendung dockt als schlanke, schwebende Kapsel („Pill“) am Bildschirmrand an und wandelt gesprochene Sprache per Tastendruck in präzisen Text um – inklusive automatischer Satzzeichen- und Kommasetzung. Das Transkript wird direkt in die Zwischenablage gelegt und optional per Auto-Paste in das aktive Eingabefeld eingefügt.

---

## Highlights

* **100 % Offline & Datenschutz:** Die Spracherkennung läuft vollständig lokal auf deiner CPU oder GPU via Whisper. Keine Cloud-Verbindung, keine API-Kosten, keine Telemetrie.
* **Kein Server-Overhead:** Reine Desktop-Software – kein MCP-Server, kein Hintergrund-Daemon-Zwang, direkte Inferenz im Prozess.
* **Minimalistisches 3-Phasen-UI:**
  * **Status 1 (Aufnahme):** Schmale Pill mit dezentem REC-Indikator.
  * **Status 2 (Fertig / Kopiert):** Schnelle Rückmeldung über die erfolgreiche Übergabe an die Zwischenablage.
  * **Status 3 (Review & Edit):** Ausklappbarer Editor zum Korrigieren oder erneuten Kopieren längerer Texte.
* **Plattformübergreifend:** Entwickelt für Linux (Wayland & X11) und Windows mit freier Positionierung per Drag & Drop.
* **MIT-Lizenz:** Maximal offen für freie Nutzung, Anpassung und Forks.

---

## UI-Konzept (ASCII-Skizzen)

```text
1. State: Aufnehmen (Recording)
╭──────────────────────────────────────────────────────────╮
│  ● REC   ılılı·|·lılı   00:04                            │
╰──────────────────────────────────────────────────────────╯

2. State: Fertig / Kopiert (Ready / Copied)
╭──────────────────────────────────────────────────────────╮
│  ✔ In Zwischenablage kopiert!        [ ⤢ Details / Edit ]│
╰──────────────────────────────────────────────────────────╯

3. State: Ausgeklappt (Expanded / Review & Edit)
╭──────────────────────────────────────────────────────────╮
│  [X] Schließen                       [ ⎘ Text kopieren ] │
│ ┌──────────────────────────────────────────────────────┐ │
│ │ Hier steht der fertig transkribierte Text mit allen  │ │
│ │ Satzzeichen und Kommas, bereit zur Prüfung...        │ │
│ └──────────────────────────────────────────────────────┘ │
╰──────────────────────────────────────────────────────────╯
