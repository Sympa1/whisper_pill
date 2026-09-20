# whisper-pill Projekt-Harness & Richtlinien

Diese Anweisungen gelten spezifisch für das Repository und die Entwicklung von **`whisper-pill`**.

## Projektziel & Lizenz
- **Ziel:** Schlankes, minimalistisches, schwebendes Desktop-Diktier-Utility (Pill-UI) für Linux und Windows.
- **Architektur:** Rein lokale Desktop-Software. **Kein MCP-Server**, kein externes Backend, keine Cloud-Anbindung.
- **Lizenz:** Das gesamte Projekt steht unter der **MIT-Lizenz** (maximal permissiv).

## Bibliotheken & Abhängigkeiten (Fokus: Python-Prototyp)
- **Standardbibliothek:** Alle mitgelieferten Python-Standardmodule (z. B. `json`, `os`, `sys`, `pathlib`, `time`, `typing`, `math` etc.) dürfen jederzeit frei und ohne Rückfrage eingesetzt werden.
- **Freigegebene externe Bibliotheken (Whitelist):** Ausschließlich die folgenden externen Pakete sind für die Implementierung zugelassen:
  - `faster-whisper`
  - `PySide6`
  - `sounddevice`
  - `numpy`
  - `scipy`
  - `pyperclip`
  - `pyautogui`
  - `keyboard` *(optional für globale Shortcuts)*
- **Verbot anderer externer Bibliotheken:** Alle weiteren Drittanbieter-Pakete sind standardmäßig untersagt. Externe Abhängigkeiten sind nach Möglichkeit immer zu vermeiden (Standardbibliothek bevorzugen).
- **Genehmigungspflicht:** Sollte ein zusätzliches externes Paket zwingend erforderlich sein, muss vor der Verwendung zwingend nachgefragt und eine nachvollziehbare Begründung geliefert werden.
- **Konfigurationsformat:** Für Konfigurationen im Prototyp wird ausschließlich `json` (Standardbibliothek) verwendet (kein YAML/INI).
- **Rust:** Spezifische Vorgaben und Whitelists für Rust-Crates (`whisper-rs`, `cpal`, `arboard`, `serde`, `toml`, `enigo`, `rdev`, `serde_json`) folgen zu einem späteren Projektzeitpunkt bei der Portierung.

## Code-Kommentare
- Alle Code-Kommentare ausschließlich in deutscher Sprache verfassen.
- Code-Kommentare ausgewogen halten: Kurz und präzise, aber mit ausreichendem Kontext (nicht zu langatmig erklären und keine Programmier-Grundlagen dokumentieren, sondern den fachlichen Sinn erklären).

## Clean Code & OOP-Prinzipien
- Klare, aussagekräftige Bezeichner verwenden.
- Selbstdokumentierender Code durch sprechende Namen.
- Objektorientierte Prinzipien konsequent anwenden (SOLID-Prinzipien beachten, Kapselung sinnvoll einsetzen).
- Simplen und modularen Code schreiben (Single Responsibility: kurze, fokussierte Methoden und überschaubare Klassen).

## Namensgebung & Variablen-Konventionen
- Bezeichner, Variablen-, Klassen- und Methodennamen grundsätzlich auf Englisch.
- **Buffer- und Zählvariablen (Typ-Präfix):**
  - Temporäre Puffer, Zwischenspeicher und Zähler müssen mit dem Datentyp-Präfix gekennzeichnet werden:
    - String-Puffer: `sBuffy` (oder `sBuffer...`)
    - Integer-Puffer / Zähler: `iBuffy` (oder `iBuffer...`)
  - Diese Konvention (`iBuffy` / `sBuffy`) ist verbindlich für Zwischenspeicherungen und Zählschleifen zu verwenden.
- Jede Klasse und jede Methode mit einer kurzen Dokumentation (Zweck und Verantwortlichkeit) versehen.

## Best Practices & Ermessensspielraum
- Innerhalb der gesetzten Grenzen und Vorgaben darf nach eigenem fachlichen Ermessen sinnvoll nach aktuellen Best Practices gearbeitet werden.
- Immer die Best Practices der jeweiligen Programmiersprache befolgen (idiomatischer Code).
- Framework- und Sprach-Konventionen einhalten.

## Workspace- & Ausfuehrungs-Richtlinie
- Artefakte, Pläne und Notizen dürfen ausschließlich im aktuellen Arbeitsverzeichnis (Workspace-Root) abgelegt werden, nicht in globalen Anwendungsdaten-Verzeichnissen (wie `~/.gemini/`).
- **Freie Befehlsausführung im Projekt:** Innerhalb des Projektverzeichnisses (`whisper_pill`) dürfen Dateien, Verzeichnisse und Befehle (`ls`, `cat`, `grep`, `pytest`, `python` etc.) vollkommen frei, eigenständig und ohne Rückfrage aufgerufen und gelesen werden.
- **Rückfragepflicht außerhalb:** Eine vorherige Rückfrage ist nur dann erforderlich, wenn Aktionen außerhalb des Projektverzeichnisses stattfinden sollen (z. B. systemweite Installationen oder globale OS-Dateien außerhalb des Workspaces).
- Für den Prototypen ein .venv im Verzeichnis `prototype` verwenden.

## Arbeitsweise, Test-Zwang & Projekt-Journal (`docs/JOURNAL.md`)
- **Autonome Arbeitsweise & durchgängige Ausführung:**
  - Logisch aufeinanderfolgende Schritte (Planen -> Implementieren -> Tests ausführen -> Fehler beheben -> Dokumentieren -> Commit/Push vorbereiten) sind **selbstständig, proaktiv und am Stück** durchzuführen.
  - Es muss nicht nach jedem einzelnen Zwischenschritt auf Bestätigung gewartet werden.
  - Bei klaren Anforderungen durcharbeiten und das Gesamtergebnis präsentieren. Nur bei echten Zielkonflikten oder unklaren Anforderungen rückfragen.
- **Test-Ausführung nach jedem Schritt (Pflicht):**
  - Nach jedem implementierten Schritt oder jeder Code-Änderung ist zwingend der passende Test aus den verfügbaren Test-Skills (`test-core`, `test-ui`, `test-e2e` bzw. `pytest`) eigenständig auszuführen.
  - Kein Schritt gilt als abgeschlossen, bevor nicht alle Tests grün durchgelaufen sind.
- **Lückenlose Dokumentation in `docs/JOURNAL.md` (Pflicht):**
  - Jeder durchgeführte Schritt, jede Architekturentscheidung und jede Änderung ist verbindlich und chronologisch oben in `docs/JOURNAL.md` zu vermerken.
- **Session-Start & Referenzierung (Pflicht):**
  - Lies zu Beginn jeder neuen Sitzung oder vor der Bearbeitung neuer Aufgaben zuerst `docs/JOURNAL.md` ein.
  - Nutze die dort dokumentierten Entscheidungen und offenen Punkte als primäre Referenz für den aktuellen Projektstand.

## Projektziel, Architektur & Phasen
- **Ziel:** Schlankes, minimalistisches, schwebendes Desktop-Diktier-Utility (Pill-UI) für Linux und Windows als freie, quelloffene Alternative zu Whisperbar.
- **Architektur:** Rein lokale Desktop-Software. **Kein MCP-Server**, kein externes Backend, keine Cloud-Anbindung.
- **Phasen-Modell:**
  - **Phase 1 (Aktuell – Prototyp):** Vollständiger Prototyp in **Python**. Dient zur schnellen Validierung von UI, Audio-Pipeline, Whisper-Inferenz und Zwischenablage.
  - **Phase 2 (Zukunft – Production):** Portierung auf **Rust** für minimale Latenz, native Binärdatei ohne Python-Laufzeitumgebung und geringsten RAM-Verbrauch.
- **Lizenz:** Das gesamte Projekt steht unter der **MIT-Lizenz** (maximal permissiv).

---

## UI-Konzept & ASCII-Art Design-Idee
Die Anwendung dockt als schwebende Kapsel („Pill“) am Bildschirmrand an und besitzt drei Kernzustände:

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
