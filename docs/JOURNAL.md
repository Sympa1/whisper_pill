# Projekt-Journal: whisper-pill

Dieses Dokument dient als kontinuierliches Gedächtnis des Projekts. 
Hier werden fundamentale Architekturentscheidungen, erreichte Meilensteine und offene Punkte chronologisch (neueste Einträge oben) dokumentiert.

### 2026-09-20 - Redundanten Symlink AGENT.md entfernt (Konsolidierung auf AGENTS.md)
- **Entscheidung / Änderung:**
  - Der redundante Symlink `AGENT.md` wurde entfernt. Die Projekt-Richtlinien verbleiben eindeutig und konsolidiert in der Standarddatei `AGENTS.md`.
  - Testsuite ausgeführt; alle 19 Tests PASSED.
- **Betroffene Komponenten:**
  - `AGENT.md` (gelöscht)
  - `docs/JOURNAL.md`

---

### 2026-09-20 - Vollständiger Sicherheits-, Secret- & Privacy-Audit durchgeführt (SECURE)
- **Entscheidung / Änderung:**
  - Audit nach den Vorgaben des Skills `security-audit` durchgeführt:
    1. **Secrets & Credentials:** Vollständiger Regex- und Entropiescan ergab 0 hardcodierte Tokens, API-Keys oder Passwörter im Quellcode.
    2. **SAST & Injection-Prüfung:** 0 unsichere `eval()`/`exec()`-Aufrufe, kein `shell=True` bei Subprozessen, kein unsicheres `pickle`/YAML.
    3. **Dateisystem:** Audioverarbeitung erfolgt zu 100 % flüchtig im RAM (NumPy-Array via FIFO-Queue); keine unsicheren `/tmp`-Dateien.
    4. **Offline-Garantie:** `WhisperTranscriber` erzwingt `local_files_only=True` zur Verhinderung von ungewollten HuggingFace-Netzwerkverbindungen.
    5. **RAM-Audio-Flushing:** `TranscriptionWorker` überschreibt das NumPy-Audioarray im RAM nach Inferenzende sofort mit Nullen (`audio_data.fill(0.0)`).
  - Neuer Sicherheitstest `test_transcription_worker_ram_audio_flushing` in `tests/test_transcribe.py` integriert; alle 19 Tests PASSED.
  - Ausführlicher Bericht in `docs/security_audit_report.md` generiert.
- **Betroffene Komponenten:**
  - `docs/security_audit_report.md`
  - `prototype/utils/transcriber.py`
  - `prototype/ui/transcription_worker.py`
  - `tests/test_transcribe.py`
  - `docs/JOURNAL.md`

---

### 2026-09-20 - Richtlinien für Test-Zwang, lückenlose Protokollierung und Workspace-Autonomie verankert
- **Entscheidung / Änderung:**
  - `AGENTS.md` um verbindliche Richtlinien erweitert:
    1. **Verpflichtende Testausführung nach jedem Schritt:** Nach jeder Code-Änderung wird automatisch der passende Test via `pytest` (`test-core`, `test-ui`, `test-e2e`) gestartet.
    2. **Lückenlose Protokollierung:** Jeder Arbeitsschritt wird kontinuierlich oben in `docs/JOURNAL.md` dokumentiert.
    3. **Autonome Workspace-Befehlsausführung:** Innerhalb des Repositories (`/home/sympa/Dev/whisper_pill/`) werden Datei- und Shell-Operationen (`ls -la`, `cat`, `pytest`, `python` etc.) selbstständig und ohne Unterbrechung ausgeführt. Rückfragen erfolgen nur bei Operationen außerhalb des Projektverzeichnisses.
  - Automatische Testsuite ausgeführt; alle 18 Tests erfolgreich PASSED.
- **Betroffene Komponenten:**
  - `AGENTS.md`
  - `docs/JOURNAL.md`

---

### 2026-09-20 - README.md mit visuellem Prototyp-Showcase aktualisiert
- **Entscheidung / Änderung:**
  - `README.md` um einen visuellen Live-Showcase der 4 echten Kapselzustände (`docs/assets/state_*.png`) erweitert.
  - Dokumentiert die feste Kapselbreite (420 px), den Schnellstart via Terminal & VS Code Play-Button, die Tastenkürzel-Tabelle (`Super+Strg+P` / `Win+Strg+P`) sowie direkte Verlinkungen auf die technische Entwicklerdokumentation und das Endnutzer-Handbuch.
- **Betroffene Komponenten:**
  - `README.md`
  - `docs/JOURNAL.md`

---

### 2026-09-20 - Endnutzer-Handbuch (autarkes HTML) & Technische Architektur-Dokumentation
- **Entscheidung / Änderung:**
  - **Technische Entwickler- und Architektur-Dokumentation (`docs/architecture/technical_documentation.md`):**
    - Umfassende Dokumentation des Gesamtsystems, aller Klassen (`PillWindow`, `AudioRecorder`, `WhisperTranscriber`, `HotkeyManager`, `PillTrayIcon`, `TranscriptionWorker`).
    - Drei native Mermaid-Diagramme: Zustandsautomat (`stateDiagram-v2`), Modul/Klassenhierarchie (`classDiagram`) und Sequenzfluss der Audio-Inferenz-Pipeline (`sequenceDiagram`).
    - Detaillierte Darstellung der Threading-Modelle (PortAudio Audio-Thread, Inferenz-Worker `QThread`, Qt-GUI-Thread).
  - **Autarkes Endbenutzer-Handbuch (`docs/user_guide.html`):**
    - Responsives HTML-Design im dunklen whisper-pill-Look ohne externe Abhängigkeiten (Zero-CDN, vollständiger Offline-Betrieb).
    - Automatisiertes Screenshot-Skript (`prototype/scripts/capture_ui_states.py`) erzeugt die 4 Kapselzustände (`docs/assets/state_*.png`).
    - Schritt-für-Schritt-Anleitung, Tastenkombinationen (`Super+Strg+P` / `Win+Strg+P`), Tray-Menü, Drag & Drop und Hinweise zum lokalen Datenschutz.
- **Betroffene Komponenten:**
  - `docs/architecture/technical_documentation.md`
  - `docs/user_guide.html`
  - `docs/assets/`
  - `prototype/scripts/capture_ui_states.py`
  - `docs/JOURNAL.md`

---

### 2026-09-20 - Shortcut-Analyse & Nativer KDE Plasma 6 KGlobalAccel D-Bus Fix
- **Entscheidung / Änderung:**
  - Ursachenanalyse für Shortcut-Fehlverhalten unter Wayland / KDE Plasma 6:
    - Analyse aller belegten Tastenkombinationen in `~/.config/kglobalshortcutsrc` durchgeführt (`Meta+Alt+P` war belegt durch `cycle-panels`, `Meta+P` durch KDE Screen Switch).
    - `Meta+Ctrl+P` (`Super+Strg+P`) wurde via D-Bus-Methode `isGlobalShortcutAvailable` geprüft und als vollständig frei verifiziert.
    - Ursache identifiziert: KWin fängt unter Wayland globale Hotkeys ab; die bloße Konfigurationszeile ohne KGlobalAccel-Komponentenbindung wurde nicht ausgewertet.
  - **Native D-Bus Lösung implementiert (`prototype/utils/hotkey.py`):**
    - Registriert `whisper_pill` direkt via D-Bus als offizielle KGlobalAccel-Komponente (`doRegister`).
    - Belegt Keycode `335544400` (`Meta+Ctrl+P`) via `setShortcut`.
    - Verbindet sich über `QDBusConnection.sessionBus().connect(...)` direkt mit dem Signal `globalShortcutPressed`.
    - Tastendruck auf `Super+Strg+P` schaltet die Pill nun verzögerungsfrei, ohne Subprozess-Start und nativ auf Wayland/X11 um.
  - Testsuite in `tests/test_ui.py` um `test_hotkey_manager_setup_and_signal` erweitert; alle 18 Tests PASSED.
- **Betroffene Komponenten:**
  - `prototype/utils/hotkey.py`
  - `tests/test_ui.py`
  - `docs/JOURNAL.md`

---

### 2026-09-20 - Globaler Shortcut auf Super+Strg+P (Linux) / Win+Strg+P (Windows) umgestellt
- **Entscheidung / Änderung:**
  - Der globale Shortcut zum Ein- und Ausblenden der Pill wurde von `Super+P` / `Win+P` auf **`Super+Strg+P` (Linux / KDE)** bzw. **`Win+Strg+P` (Windows)** umgestellt:
    - Verhindert Konflikte mit den systemweiten Standardbelegungen von Windows und Desktop-Environments (wo `Win+P` standardmäßig für die Umschaltung externer Bildschirme/Monitore reserviert ist).
    - **Linux (KDE Plasma 6 / Wayland / X11):** Shortcut in `kglobalshortcutsrc` auf `Meta+Ctrl+P` aktualisiert und im Desktop-File verankert.
    - **Windows:** Registrierung in `keyboard` auf `windows+ctrl+p` angepasst.
    - UI-Hinweise (Tooltips, Tray-Menü, Pill-Kontextmenü und Konsolenausgabe) synchronisiert.
- **Betroffene Komponenten:**
  - `prototype/utils/hotkey.py`
  - `prototype/ui/pill_window.py`
  - `prototype/ui/tray_icon.py`
  - `prototype/main.py`
  - `docs/JOURNAL.md`
- **Nächste Schritte / Offene Punkte:**
  - Konfigurationsdatei (`prototype/config.py` / `prototype/config.json`) für Modellgröße, Hotkey und Audioschnittstelle.
  - Technische Entwicklerdokumentation mit Mermaid-UML (`tech-docs`).
  - Endnutzer-Handbuch mit autarkem HTML-Build (`user-docs`).

---

### 2026-09-20 - Einheitliche Kapselbreite (420 px) über alle Zustände
- **Entscheidung / Änderung:**
  - Die Kapsel-Dimensionen wurden vereinheitlicht: Die Pill behält in **allen Zuständen** (`IDLE`, `RECORDING`, `PROCESSING`, `READY`, `EXPANDED`) eine **exakt konstante Breite von 420 px** (`PillWindow.PILL_WIDTH = 420`).
  - Verhindert jegliches horizontales Springen, Zucken oder Verschieben der Kapsel-Silhouette beim Zustandswechsel.
  - Ausbalanciertes Layout:
    - **Idle (420×48 px):** "🎙 whisper-pill" linksbündig, "● Aufnehmen" rechtsbündig.
    - **Recording (420×48 px):** "● REC" links, mittig zentrierte Audio-Waveform, Zeitstempel ("00:00") und "■ Stop" rechtsbündig.
    - **Processing (420×48 px):** "⟳ Transkribiere Audiosignal..." mit dynamischem Spacer.
    - **Ready (420×48 px):** "✔ In Zwischenablage kopiert!" links, Schnellzugriff `[⤢ Details / Edit]` und `[✕]` rechts.
    - **Expanded (420×240 px):** Kapsel dehnt sich nahtlos ausschließlich vertikal nach unten aus; Kopfzeile mit `[✕ Verbergen]`, `[⏻ Beenden]` und `[⎘ Text kopieren]`, darunter der Texteditor.
  - Neuer Geometrietest `test_pill_uniform_width_across_all_states` in `tests/test_ui.py` prüft alle 5 Zustände; alle 17 Tests PASSED.
- **Betroffene Komponenten:**
  - `prototype/ui/pill_window.py`
  - `tests/test_ui.py`
  - `docs/JOURNAL.md`
- **Nächste Schritte / Offene Punkte:**
  - Konfigurationsdatei (`prototype/config.py` / `prototype/config.json`) für Modellgröße, Hotkey und Audioschnittstelle.
  - Technische Entwicklerdokumentation mit Mermaid-UML (`tech-docs`).
  - Endnutzer-Handbuch mit autarkem HTML-Build (`user-docs`).

---

### 2026-09-20 - Hintergrundbetrieb, Globale Hotkeys (Super+P / Win+P) & Beenden-Funktionen
- **Entscheidung / Änderung:**
  - Hintergrundbetrieb (`app.setQuitOnLastWindowClosed(False)`) und System-Tray-Icon (`prototype/ui/tray_icon.py`) implementiert:
    - Kapsel-Symbol in der Desktop-Taskleiste.
    - Linksklick auf Tray schaltet Pill ein/aus.
    - Tray-Kontextmenü mit Schnellzugriff auf Ein-/Ausblenden, Aufnahme und Beenden.
  - Globale Hotkeys plattformübergreifend angebunden (`prototype/utils/hotkey.py`):
    - **Linux (KDE Plasma 6 / Wayland / X11):** D-Bus-Dienst `org.whisper_pill.Pill` auf `/Pill` und Single-Instance-Toggle (`--toggle`). Integration in KDE-Shortcuts (`Meta+P` / `Super+P`) via `kglobalshortcutsrc` und Desktop-Datei.
    - **Windows:** Registrierung von `Windows+P` via `keyboard` (aus der Whitelist).
  - Umfassende Beenden-Optionen integriert:
    1. System-Tray-Menü: "✕ whisper-pill beenden".
    2. Pill-Kontextmenü (Rechtsklick auf Kapsel): "⏻ whisper-pill beenden".
    3. Ausgeklappte Ansicht (State 3): Neuer roter Button `[⏻ Beenden]` neben `[✕ Verbergen]`.
    4. Terminal / VS Code: Sauberer Exit bei `Strg+C` ohne Traceback.
  - Testsuite in `tests/test_ui.py` um Sichtbarkeits-, Beendigungs- und Tray-Tests erweitert; alle 16 Tests PASSED.
- **Betroffene Komponenten:**
  - `prototype/ui/tray_icon.py`
  - `prototype/utils/hotkey.py`
  - `prototype/ui/pill_window.py`
  - `prototype/main.py`
  - `tests/test_ui.py`
  - `docs/JOURNAL.md`
- **Nächste Schritte / Offene Punkte:**
  - Konfigurationsdatei (`prototype/config.py` / `prototype/config.json`) für Modellgröße, Hotkey und Audioschnittstelle.
  - Technische Entwicklerdokumentation mit Mermaid-UML (`tech-docs`).
  - Endnutzer-Handbuch mit autarkem HTML-Build (`user-docs`).

---

### 2026-09-20 - VS Code Workspace-Integration & Play-Button-Support
- **Entscheidung / Änderung:**
  - VS Code-Konfiguration eingerichtet, damit die Anwendung nahtlos über den grünen Play-Button oben rechts bzw. F5 gestartet werden kann:
    - `.vscode/settings.json`: Setzt `python.defaultInterpreterPath` fest auf `${workspaceFolder}/prototype/.venv/bin/python` und bindet das Root-Verzeichnis in `extraPaths` ein.
    - `.vscode/launch.json`: Definiert das Launch-Profil `whisper-pill (Desktop-Pill starten)` mit gesetztem `PYTHONPATH` und `cwd`.
  - `prototype/main.py`: Fenster wird beim Start via `window.raise_()` und `window.activateWindow()` aktiv in den Vordergrund über das VS Code-Fenster gehoben.
- **Betroffene Komponenten:**
  - `.vscode/settings.json`
  - `.vscode/launch.json`
  - `prototype/main.py`
  - `docs/JOURNAL.md`
- **Nächste Schritte / Offene Punkte:**
  - Globaler System-Hotkey (z. B. via `keyboard` oder Qt-Shortcut) zum Ein-/Ausschalten der Aufnahme aus jeder Anwendung heraus.
  - Konfigurationsdatei (`prototype/config.py` / `prototype/config.json`) für Modellgröße, Hotkey und Audioschnittstelle.

---

### 2026-09-20 - Visuelles Polish & Kapsel-Design perfektioniert (48px)
- **Entscheidung / Änderung:**
  - Visuelles Rendering der Pill analysiert und optimiert:
    - Layout-Margins auf 0 gesetzt, Innenabstände auf 16×4 px kalibriert.
    - Feste Kapselhöhe von 48 px mit 24 px Border-Radius implementiert (vollständig abgerundete Stadionform).
    - Inaktive Widgets im `QStackedWidget` erhalten dynamisch `QSizePolicy.Ignored`, sodass die Pill im Ruhezustand (`290×48`), Aufnahme-Zustand (`360×48`) und Kopiert-Zustand (`410×48`) extrem schlank bleibt.
    - Im ausgeklappten Zustand (`EXPANDED`) öffnet sich die Pill auf `460×250` und schrumpft danach wieder zusammen.
    - Alle 4 Zustände visuell gerendert und pixelgenau verifiziert.
    - Alle 13 Tests in `tests/` weiterhin PASSED.
- **Betroffene Komponenten:**
  - `prototype/ui/pill_window.py`
  - `prototype/ui/waveform_widget.py`
  - `docs/JOURNAL.md`
- **Nächste Schritte / Offene Punkte:**
  - Globaler System-Hotkey (z. B. via `keyboard` oder Qt-Shortcut) zum Ein-/Ausschalten der Aufnahme aus jeder Anwendung heraus.
  - Konfigurationsdatei (`prototype/config.py` / `prototype/config.json`) für Modellgröße, Hotkey und Audioschnittstelle.

---

### 2026-09-20 - Wayland/Qt-Zwischenablagen-Integration & Clipboard-Test
- **Entscheidung / Änderung:**
  - Clipboard-Funktionalität auf der Zielplattform (Linux Wayland / `wayland-0`) diagnostiziert: `pyperclip` erfordert externe CLI-Werkzeuge (`wl-clipboard`/`xclip`), die standardmäßig fehlen können.
  - Robuste Dual-Clipboard-Architektur in `prototype/utils/transcriber.py` implementiert:
    - Primär: Natives Qt-Clipboard (`QGuiApplication.clipboard()`), das ohne externe Tools direkt mit dem Wayland-Compositor und X11 kommuniziert.
    - Sekundär: `pyperclip`-Fallback für Standalone-CLI-Aufrufe.
  - Live-Systemtest erfolgreich: Schreiben und Lesen via Qt-Clipboard verifiziert.
  - Zusätzlicher Unit-Test in `tests/test_transcribe.py` hinzugefügt; alle 13 Tests in `tests/` PASSED.
- **Betroffene Komponenten:**
  - `prototype/utils/transcriber.py`
  - `tests/test_transcribe.py`
  - `tests/test_ui.py`
- **Nächste Schritte / Offene Punkte:**
  - Globaler System-Hotkey (z. B. via `keyboard` oder Qt-Shortcut) zum Ein-/Ausschalten der Aufnahme aus jeder Anwendung heraus.
  - Konfigurationsdatei (`prototype/config.py` / `prototype/config.json`) für Modellgröße, Hotkey und Audioschnittstelle.

---

### 2026-09-20 - Schwebende PySide6 Pill-UI & State-Machine implementiert
- **Entscheidung / Änderung:**
  - Schwebende, rahmenlose Desktop-Kapsel (`prototype/ui/pill_window.py`) mit transluzentem Hintergrund, abgerundeten Ecken und Drag&Drop-Verschiebung implementiert.
  - Exakte Umsetzung der drei Kernzustände aus der ASCII-Vorgabe:
    - **State 1: Aufnehmen** (`● REC   ılılı·|·lılı   00:04`) mit animiertem Waveform-Widget (`WaveformWidget`), Aufnahme-Timer und Stop-Button.
    - **State 2: Fertig / Kopiert** (`✔ In Zwischenablage kopiert! [ ⤢ Details / Edit ]`) mit Auto-Reset-Timer.
    - **State 3: Ausgeklappt** (`[X] Schließen [ ⎘ Text kopieren ]`) mit direkt editierbarem `QTextEdit`.
  - Asynchroner Worker-Thread (`prototype/ui/transcription_worker.py`) verhindert jedes Einfrieren der GUI während der Whisper-Inferenz.
  - Thread-sichere Audio-Pegelbrücke (`AudioLevelBridge`) via Qt-Signale.
  - Haupteinstiegspunkt der Anwendung (`prototype/main.py`) erstellt.
  - Automatisierte UI- und State-Machine-Testsuite (`tests/test_ui.py`) mit 4/4 Tests erfolgreich (PASSED) headless durchgeführt.
  - Gesamt-Testsuite: 12/12 Tests PASSED.
  - UI-Testbericht unter `docs/ui_test_report.md` dokumentiert.
- **Betroffene Komponenten:**
  - `prototype/ui/state.py`
  - `prototype/ui/waveform_widget.py`
  - `prototype/ui/transcription_worker.py`
  - `prototype/ui/pill_window.py`
  - `prototype/main.py`
  - `tests/test_ui.py`
  - `docs/ui_test_report.md`
- **Nächste Schritte / Offene Punkte:**
  - Globaler System-Hotkey (z. B. via `keyboard` oder Qt-Shortcut) zum Ein-/Ausschalten der Aufnahme aus jeder Anwendung heraus.
  - Konfigurationsdatei (`prototype/config.py` / `prototype/config.json`) für Modellgröße, Hotkey und Audioschnittstelle.
  - Erstellung der technischen Dokumentation (`tech-docs`) und des Benutzerhandbuchs (`user-docs`).

---

### 2026-09-20 - WhisperTranscriber & End-to-End Pipeline implementiert
- **Entscheidung / Änderung:**
  - `WhisperTranscriber`-Klasse in `prototype/utils/transcriber.py` implementiert. Kapselt `faster-whisper` (Quantisierung CPU int8, VAD-Filterung) und `pyperclip` für automatische Zwischenablage.
  - Einhaltung der Variablen- und Pufferkonvention (`sBuffy_full_text`, `sBuffy_segment_text`).
  - Unit-Tests in `tests/test_transcribe.py` für Zwischenablage, Leereingaben und gemockte Whisper-Segmente hinzugefügt.
  - Gesamte `test-core` Testsuite erfolgreich ausgeführt: 8/8 Tests PASSED (0.47s).
  - Interaktives Demo- und Verifikationsskript `prototype/scripts/demo_pipeline.py` bereitgestellt.
  - Live-Audioaufnahme am System mit 47.759 Samples verifiziert.
- **Betroffene Komponenten:**
  - `prototype/utils/transcriber.py`
  - `tests/test_transcribe.py`
  - `prototype/scripts/demo_pipeline.py`
  - `docs/unit_test_report.md`
- **Nächste Schritte / Offene Punkte:**
  - Schwebende PySide6-Pill-Benutzeroberfläche (`prototype/ui/pill_window.py`) implementieren.
  - Drei Kernzustände nach ASCII-Art (Aufnehmen / Fertig / Ausgeklappt) visualisieren.
  - Globalen Hotkey / Klick-Interaktion zur Aufnahmesteuerung anbinden.

---

### 2026-09-20 - AudioRecorder & Unit-Tests implementiert
- **Entscheidung / Änderung:**
  - `AudioRecorder`-Klasse in `prototype/audio/recorder.py` implementiert. Verwendet `sounddevice.InputStream` zur Erfassung in 16 kHz Mono Float32 (Whisper-Standard).
  - Thread-sichere Pufferung mit `queue.Queue` und verzögerungsfreier Zusammenführung via `numpy.concatenate`.
  - RMS-Berechnung für Audio-Pegel zur späteren Visualisierung in der Pill-UI integriert (unter Beachtung der Zähl- und Puffervariablen-Konvention `iBuffy_rms`).
  - Umfassende Headless-Unit-Tests in `tests/test_audio.py` erstellt (Initialisierung, Stille/RMS, Thread-Pufferung, Stream-Lifecycle, Idle-Stop).
  - Testsuite via `pytest` ausgeführt: 5/5 Tests erfolgreich (PASSED).
  - Testbericht in `docs/unit_test_report.md` generiert.
- **Betroffene Komponenten:**
  - `prototype/audio/recorder.py`
  - `tests/__init__.py`
  - `tests/test_audio.py`
  - `docs/unit_test_report.md`
- **Nächste Schritte / Offene Punkte:**
  - Whisper-Inferenz-Wrapper (`prototype/utils/transcriber.py`) mit `faster-whisper` implementieren.
  - Headless-Unit-Test (`test-core`) für Transkription mit Test-Audiosample schreiben (`tests/test_transcribe.py`).
  - Integration von `pyperclip` für automatische Zwischenablage.

---

### 2026-09-20 - Prototype-Ordnerstruktur & Virtuelle Umgebung aufgesetzt
- **Entscheidung / Änderung:**
  - Ordnerstruktur in `prototype/` mit Paketen `audio/`, `ui/` und `utils/` sowie Modul-Initialisierern (`__init__.py`) angelegt.
  - Virtuelle Umgebung `prototype/.venv` initialisiert (Python 3.14).
  - Freigegebene Whitelist-Abhängigkeiten in `prototype/requirements.txt` definiert (`faster-whisper`, `PySide6`, `sounddevice`, `numpy`, `scipy`, `pyperclip`, `pyautogui`, `pytest`) und erfolgreich installiert.
  - Sanity-Check der Importe verifiziert.
- **Betroffene Komponenten:**
  - `prototype/audio/__init__.py`
  - `prototype/ui/__init__.py`
  - `prototype/utils/__init__.py`
  - `prototype/requirements.txt`
  - `prototype/.venv`
- **Nächste Schritte / Offene Punkte:**
  - Basisklasse für die Audio-Aufnahme via `sounddevice` implementieren (`prototype/audio/recorder.py`).
  - Headless-Unit-Test (`test-core`) für Audio-Pufferung und Datenformate schreiben (`tests/test_audio.py`).

---

### 2026-09-20 - Initiales Projekt-Setup & Harness-Definition
- **Entscheidung / Änderung:**
  - **Projektziel:** Lokales, datenschutzorientiertes Desktop-Diktier-Utility (schwebende Pill-UI) für Linux und Windows unter MIT-Lizenz.
  - **Architektur & Phasen:**
    - *Phase 1 (Aktuell):* Prototyp in Python (`faster-whisper`, `PySide6`, `sounddevice`, `pyperclip`, `pyautogui`, optional `keyboard`).
    - *Phase 2 (Produktion):* Nativer Port nach Rust (`whisper-rs`, `cpal`, `arboard`, `serde`, `toml`).
  - **Guardrails:** 100 % offline, keine Cloud, kein MCP-Server, keine unautorisierten externen Bibliotheken.
  - **Standards & Konventionen:** Deutsche Code-Kommentare, englische Bezeichner, strikte Puffer-Präfixe (`sBuffy` für Strings, `iBuffy` für Integer und Schleifenzähler).
  - **Harness & Skills:** 
    - `AGENTS.md` als Regelwerk aufgesetzt.
    - Automatisierte Skills für Security-Audit (CVEs, SAST, Hardcoded Secrets, Offline-Prüfung), technische Doku (Mermaid-UML), Endnutzer-Doku (HTML) sowie getrennte Testsuiten (`test-core`, `test-ui`) und `gitignore-analyzer` definiert.
    - Standardisiertes Markdown-Reporting im Verzeichnis `docs/` verankert.
- **Betroffene Komponenten:**
  - `AGENTS.md`
  - `.agents/skills/*`
  - `README.md`
  - `docs/JOURNAL.md`
- **Nächste Schritte / Offene Punkte:**
  - `prototype/`-Ordnerstruktur aufbauen (`audio/`, `ui/`, `utils/`).
  - Virtuelle Umgebung (`.venv`) anlegen und Whitelist-Abhängigkeiten installieren.
  - Basisklasse für die Audio-Aufnahme via `sounddevice` implementieren.
  - Ersten Headless-Unit-Test (`test-core`) für den Audio-Puffer schreiben.
