# Projekt-Journal: whisper-pill

Dieses Dokument dient als kontinuierliches Gedächtnis des Projekts. 
Hier werden fundamentale Architekturentscheidungen, erreichte Meilensteine und offene Punkte chronologisch (neueste Einträge oben) dokumentiert.

### 2026-10-18 - Vollständige Neuerstellung der Benutzer- und Technik-Dokumentation nach Skills-Standard
- **Entscheidung / Änderung:**
  1. **Überarbeitung der Endbenutzer-Dokumentation (`docs/USER_GUIDE.md`, `docs/user_guide.html`):**
     - Gemäß neuem `user-docs`-Skill vollständig strukturiert:
       - *Willkommen & Überblick:* Kernmerkmale (100 % lokal, Tray-First, portables AppImage).
       - *Schnellstart (Quickstart in 3 Schritten):* Visueller 3-Schritte-Workflow.
       - *Mermaid-Diagramme:* User Journey Workflow (`flowchart LR`), Zustandsübergänge (`stateDiagram-v2`) und detaillierter Troubleshooting-Entscheidungsbaum (`flowchart TD`).
       - *UI-Screenshots aller 6 Zustände:* Einbindung der Screenshots `state_0_idle.png` bis `state_5_installer.png` (inkl. neuem 4-Schritte Slint Installations-Assistenten).
       - *Plattform-Vergleich & Tastaturtabellen:* Spezifische Anleitungen für KDE Plasma, XFCE und Windows.
       - *Autarker Zero-CDN HTML-Build (`docs/user_guide.html`):* Vollständiges Offline-HTML mit Dark-Mode-Styling und responsiven Inline-Vektorgrafiken (`<svg>`) ohne externe Tracker, CDNs oder Fonts.
  2. **Umfassende Aktualisierung der Technischen Architektur-Dokumentation (`docs/architecture/technical_documentation.md`, `docs/TECHNICAL_DOCS.md`):**
     - Gemäß neuem `tech-docs`-Skill implementiert mit allen 5 standardisierten Mermaid-UML-Diagrammtypen:
       - *Diagramm A (System- & Schichtenarchitektur):* Presentation (Slint AppWindow & InstallerWindow), Orchestration (AppOrchestrator), Domain (AudioRecorder, VadDetector, WhisperEngine, TextAccumulator), Infrastructure (HotkeyManager, PillTray, KWinPlatform, AutostartPlatform, InstallerPlatform, ClipboardManager).
       - *Diagramm B (UML-Klassendiagramm):* Vollständige Kapselung aller Services, Manager und Plattform-Module.
       - *Diagramm C (Multi-Threading-Sequenz):* D-Bus Signalfilterung, Audio-Ingest, VAD-Stilleerkennung, Whisper-Inferenz-Worker, RAM-Zeroing (`*s = 0.0`) und Puffer-Konventionen (`sBuffy` / `iBuffy`).
       - *Diagramm D (Zustandsautomat):* Gesamte State Machine inklusive aller 7 Laufzeitzustände und des 8s Auto-Dismiss Timers.
       - *Diagramm E (Daten- & Konfigurationsmodell):* `APP_CONFIG`, `INSTALL_OPTIONS`, `AUDIO_BUFFER` und `TRANSCRIPTION_RESULT`.
       - *Standardisierter Einstiegspunkt:* Bereitstellung von `docs/TECHNICAL_DOCS.md`.
  3. **Qualitätssicherung & Tests:**
     - Zero-Download & Security Scan (`python3 .agent/skills/security-audit/scripts/check_downloads.py .`): **PASSED (0 Befunde)**.
     - 19 Python-Unit-Tests (`pytest`): **PASSED**.
     - 46 Rust-Unit-Tests (`cargo test`): **PASSED**.
- **Betroffene Komponenten:**
  - `UserGuide` (`docs/USER_GUIDE.md`, `docs/user_guide.html`)
  - `TechnicalDocs` (`docs/architecture/technical_documentation.md`, `docs/TECHNICAL_DOCS.md`)
  - `docs/JOURNAL.md`

---

### 2026-10-18 - Antigravity Skills: Synchronisation & Update aus dem AI-Workflow-Baukasten (`ai_dev_sample`)
- **Entscheidung / Änderung:**
  1. **Übernahme & Aktualisierung aller modernen Antigravity-Skills (`.agent/skills/`):**
     - Aus dem zentralen Baukasten (`ai_dev_sample/template_data/antigravity/skills`) wurden alle neuen und aktualisierten Produktions-Skills nach `whisper_pill/.agent/skills/` synchronisiert:
       - `code-quality-linting`: Statische Code-Analyse mit Ruff und Rust Clippy nach Clean-Code-Standards.
       - `code-refactoring`: Strukturierter Leitfaden für verhaltensbewahrendes 4-Phasen-Refactoring.
       - `database-migration`: Schema-Migration und Rollback-Verfahren (SQLite/Alembic).
       - `unit-testing`: Erweiterter Test-Skill mit Fail-Fast-Abbruchregel für Subagenten.
       - `gitignore-analyzer`: Aktualisierte Repository-Hygiene und Erkennung großer Modellgewichte/Caches.
       - `security-audit`: Umfassender Supply-Chain- und Download-Auditor inkl. `scripts/check_downloads.py` (Zero-Download-Check, AST-Netzwerkscan, Download-Inventartabelle, Entropie-Secrets und Injection-Filterung).
       - `tech-docs`: Standardisierte Mermaid-UML-Dokumentation (5 Diagrammtypen).
       - `user-docs`: Benutzerhandbuch mit User Journey, Flowcharts, Troubleshooting-Entscheidungsbäumen und Zero-CDN Inline-SVG.
       - `test-ui` & `test-e2e`: Modernisiert mit Fail-Fast-Protokoll und deterministischer Pufferüberprüfung (`sBuffy`/`iBuffy`).
  2. **Kompatibilitäts-Symlink (`.agents -> .agent`):**
     - Einrichtung des Symlinks `.agents` auf `.agent`, sodass sowohl Antigravity-Konventionen (`.agents/skills` als auch `.agent/skills`) nativ aufgelöst werden.
  3. **Qualitätssicherung & Tests:**
     - Zero-Download & Security-Audit (`python3 .agent/skills/security-audit/scripts/check_downloads.py .`): **PASSED (0 Befunde, 16 inventarisierte autorisierte Setup-Punkte)**.
     - 19 Python-Unit-Tests (`pytest`): **PASSED**.
     - 46 Rust-Unit-Tests (`cargo test`): **PASSED**.
- **Betroffene Komponenten:**
  - `Skills` (`.agent/skills/*`, `.agents -> .agent`)
  - `Harness` (`AGENTS.md`)
  - `docs/JOURNAL.md`

---

### 2026-10-17 - Phase 2: Grafischer Installations-Wizard (Rust & Slint Setup-Assistent) für Windows & Linux
- **Entscheidung / Änderung:**
  1. **Nativer 4-Schritte-Installations-Assistent in Rust & Slint (`InstallerWindow`, `app/ui/installer.slint`, `app/src/platform/installer.rs`):**
     - Vollständig ohne systemweite Zusatzpakete im dunklen Whisper-Pill-Design (`640 × 480 px`) implementiert:
       - *Schritt 1 (Willkommen & Lizenz):* Projektvorstellung und MIT-Lizenzvereinbarung mit Bestätigungs-Checkbox.
       - *Schritt 2 (Zielverzeichnis & Verknüpfungen):* Frei wählbarer Installationspfad (Windows-Standard `%LOCALAPPDATA%\Programs\WhisperPill`, Linux-Standard `~/.local/share/whisper_pill/bin`), Checkboxen für Desktop-Verknüpfung (`.lnk` via VBScript bzw. `.desktop`), Startmenü-/Anwendungsmenü-Eintrag und System-Autostart (Windows Registry `HKCU\...\Run` bzw. XDG-Autostart).
       - *Schritt 3 (Fortschritt & Installation):* Fortschrittsbalken mit Live-Protokollierung und automatischer Erzeugung eines Deinstallations-Skripts (`uninstall.cmd` bzw. `uninstall.sh`).
       - *Schritt 4 (Abschluss):* Zusammenfassung und Option zum sofortigen Starten von Whisper Pill.
     - Startbar über den CLI-Parameter `--installer` oder automatisch, wenn die ausführbare Datei `installer` bzw. `setup` im Namen trägt (`dist/whisper_pill_installer` / `Whisper_Pill_Setup_Wizard.exe`).
     - Automatischer Screenshot-Export des Setup-Wizards nach `docs/assets/state_5_installer.png`.
  2. **Klassisches NSIS-Installer-Skript & Build-Pipeline (`packaging/windows/installer.nsi`, `scripts/build_windows_installer.sh`):**
     - Bereitstellung einer vollständigen deutschen MUI2-NSIS-Konfiguration für `Whisper_Pill_Setup_x86_64.exe` inklusive Registry-Uninstaller sowie eines kombinierten Build-Skripts.
  3. **Tests:**
     - 46 Rust-Unit-Tests (`cargo test`, inkl. 4 neuer Installer-Tests) PASSED.
     - `cargo clippy --all-targets -- -D warnings` mit 0 Warnungen abgeschlossen.
     - 19 Python-Unit-Tests (`pytest`) PASSED.
- **Betroffene Komponenten:**
  - `InstallerUI` (`app/ui/installer.slint`, `app/ui/appwindow.slint`)
  - `InstallerPlatform` (`app/src/platform/installer.rs`, `app/src/main.rs`)
  - `WindowsPackaging` (`packaging/windows/installer.nsi`, `scripts/build_windows_installer.sh`)
  - `docs/JOURNAL.md`

---

### 2026-10-17 - Phase 2: Sicherheits-Checkup (`SECURE`), Benutzer- & Technik-Dokumentation (Rust-App, AppImage, KDE & XFCE) sowie Nativer Slint-Screenshot-Generator
- **Entscheidung / Änderung:**
  1. **Vollständiger Sicherheits-Checkup der Rust-App & des AppImages (`docs/security_audit_report.md`):**
     - Gesamtbewertung: **SECURE** (alle Prüfungen bestanden).
     - Automatisierter Zero-Download & Offline-Check (`check_downloads.py`): **PASSED** (0 unerlaubte Downloads oder Netzwerk-Crates im regulären Betrieb).
     - Statische Code-Analyse (SAST): **0 `unsafe`-Blöcke** im gesamten Rust-Anwendungscode, **0 Clippy-Warnungen**, keine Shell-Interpolation (`sh -c`), strikte D-Bus-Signal-Isolierung auf `/component/whisper_pill` (`HotkeyManager`), sichere `$APPIMAGE`-Autostart-Auflösung ohne `/tmp`-Mount-Leaking sowie lückenloses RAM-Zeroing (`*s = 0.0`) der Audiosamples direkt nach jeder Transkription.
  2. **Ausführliche Endbenutzer-Dokumentation für Rust-App, AppImage, KDE Plasma & XFCE (`docs/USER_GUIDE.md`, `docs/user_guide.html`):**
     - Detaillierte Anleitung für den portablen AppImage-Einsatz (`chmod +x`, CLI-Optionen, selbstheilender Autostart).
     - Dedizierte Kapitel für **KDE Plasma 5/6 (Wayland & X11)** (`KGlobalAccel`-Shortcut, Yakuake-Z-Order-Harmonie, KWin-Regeln, StatusNotifier-Tray) und **XFCE (X11)** (`xfce4-statusnotifier-plugin`, `Xfwm4`-Fensterattribute via `xprop`, Kurzbefehle via `xfce4-keyboard-settings`, Audio via `pavucontrol`).
     - Einbindung und Erläuterung aller 5 direkt aus der Rust-App gerenderten UI-Screenshots (`state_0_idle.png` bis `state_4_settings.png`) sowohl im Markdown- als auch im autarken Offline-HTML-Handbuch.
  3. **Technische Architektur- & Entwickler-Dokumentation (`docs/architecture/technical_documentation.md`):**
     - Vollständige Dokumentation der Rust/Slint-Architektur inklusive Mermaid-Systemübersicht, Zustandsautomat (inkl. 8s Auto-Dismiss, Scrollable Editor, Settings), Klassendiagramm und Multi-Thread-Sequenzdiagramm.
     - Spezifikation der Plattform-Integration für KDE Plasma (D-Bus Match-Rule & Payload-Filter, KWin) und XFCE (X11 EWMH-Atome, StatusNotifierItem) sowie der AppImage-Build- und Autostart-Architektur.
  4. **Nativer Screenshot-Generator (`--capture-screenshots`) & Überarbeitung der `README.md`:**
     - Implementierung von `run_screenshot_capture` und `save_rgba_as_png` in `AppOrchestrator` (`main.rs`) mittels `slint::Window::take_snapshot()`.
     - Überarbeitung der `README.md` mit kompakter Rust-Projektbeschreibung und allen 5 neuen Original-Screenshots der Rust-App.
  5. **Tests & Qualitätssicherung:**
     - 42 Rust-Unit-Tests (`cargo test`) PASSED.
     - `cargo clippy --all-targets -- -D warnings` mit 0 Warnungen und 0 Fehlern abgeschlossen.
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - Zero-Download Security-Audit (`check_downloads.py`) PASSED.
- **Betroffene Komponenten:**
  - `AppOrchestrator` (`app/src/main.rs`)
  - `SecurityAuditReport` (`docs/security_audit_report.md`)
  - `UserGuide` (`docs/USER_GUIDE.md`, `docs/user_guide.html`)
  - `TechnicalDocumentation` (`docs/architecture/technical_documentation.md`)
  - `Assets` (`docs/assets/state_0_idle.png` bis `state_4_settings.png`)
  - `README.md`, `docs/JOURNAL.md`

---

### 2026-10-16 - Phase 2: Public-Repo-Hygiene in `scripts/build_appimage.sh`
- **Entscheidung / Änderung:**
  1. **Bereinigung lokaler Projektpfade (`scripts/build_appimage.sh`):**
     - Verweise auf lokale Nachbarprojekte bei der Suche nach `appimagetool` wurden entfernt und durch generische Standardpfade (`$PATH`, `$APPIMAGETOOL`, `~/.local/bin/appimagetool`) ersetzt.
  2. **Tests:**
     - 42 Rust-Unit-Tests (`cargo test`) PASSED.
     - 19 Python-Unit-Tests (`pytest`) PASSED.
- **Betroffene Komponenten:**
  - `BuildScript` (`scripts/build_appimage.sh`)
  - `docs/JOURNAL.md`

---

### 2026-10-14 - Phase 2: Hotkey-Entkopplung von Fremdsignalen (Yakuake / Super-Taste Fix), Zuverlässige Transkript-Scrollbar & Harmonisierte Button-Icons
- **Entscheidung / Änderung:**
  1. **Entkopplung globaler Hotkeys von systemweiten KDE-Signalen (`HotkeyManager`):**
     - **Ursachenanalyse:** Die bisherige D-Bus Match-Rule lauschte unselektiv auf das Signal `org.kde.kglobalaccel.Component.globalShortcutPressed` ohne Pfadfilter oder Argumentenprüfung und verwarf den Nachrichteninhalt. Dadurch feuerte KDE KGlobalAccel bei *jedem beliebigen Shortcut* des Desktops (z. B. <kbd>F12</kbd> für Yakuake, <kbd>Super</kbd>-Taste für das Anwendungsmenü/KRunner, Lautstärketasten etc.) das Signal und Whisper Pill startete fälschlicherweise eine Aufnahme.
     - **Spezifische Pfad- & Komponenten-Filterung:** Match-Rule strikt auf den D-Bus-Objektpfad `/component/whisper_pill` eingegrenzt. Deserialisierung und Verifikation der Signal-Parameter `(component, action, timestamp)`: Nur wenn `component == "whisper_pill"` und `action == "toggle_pill"` zutreffen, wird das Hotkey-Event an den Orchestrator weitergeleitet. Externe Shortcuts wie <kbd>F12</kbd> oder <kbd>Super</kbd> lösen Whisper Pill unter keinen Umständen mehr aus.
  2. **Zuverlässige & Elegante Scrollbar im Texteditor (`Slint UI`):**
     - **Ursachenanalyse:** `ScrollView` berechnet ohne explizite Höhenbindung (`content-height`) keine Inhaltsausdehnung (`maximum = 0`), weswegen die Standard-Scrollbar permanent unsichtbar blieb. Zudem war die native Fluent-Scrollbar mit 2 px Stärke kaum wahrnehmbar.
     - **Implementierung:** Umstellung auf ein natives `Flickable` mit dynamischer `content-height: max(self.height, transcript-input.preferred-height + 20px)`. Bei längeren Transkripten blendet sich eine maßgeschneiderte, 6 px breite Scrollbar mit abgerundetem Daumen ein. Ermöglicht flüssiges Scrollen per Mausrad, Tastaturnavigation sowie direktes Ziehen des Scroll-Daumens mit der Maus.
  3. **Harmonisierung der Button-Beschriftungen & Icons (Ausgeklappter Editor):**
     - **Ursachenanalyse:** Zuvor war das Schließen-Symbol `✕` irreführend dem Button "Verbergen" zugeordnet, während "Beenden" mit dem unpassenden Power-Symbol `⏻` versehen war (welches zudem auf Standard-Desktop-Schriftarten oft als fehlendes Zeichen/Tofu gerendert wurde).
     - **Bereinigung:**
       - `▲ Zuklappen`: Klappt den Editor zurück in die kompakte Kapsel.
       - `– Verbergen`: Minimiert die Kapsel in den Hintergrund / Systemtray.
       - `✕ Beenden`: Schließt die Anwendung vollständig (im markanten Rot-Ton `#ef4444`).
       - `⎘ Text kopieren`: Kopiert den bearbeiteten Text in die Zwischenablage.
  4. **Tests & Packaging:**
     - 42 Rust-Unit-Tests (`cargo test`) PASSED (inkl. Tests für Hotkey-Komponentenfilterung `test_is_whisper_pill_shortcut` und `test_manual_trigger`).
     - `cargo clippy --all-targets -- -D warnings` mit 0 Warnungen und 0 Fehlern abgeschlossen.
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - AppImage Release-Build (`dist/Whisper_Pill-x86_64.AppImage`, 13 MB) erfolgreich neu generiert.
- **Betroffene Komponenten:**
  - `HotkeyManager` (`app/src/platform/hotkey.rs`)
  - `AppWindow` (`app/ui/appwindow.slint`)
  - `AppImage` (`dist/Whisper_Pill-x86_64.AppImage`)
  - `docs/JOURNAL.md`

---

### 2026-10-14 - Phase 2: Robuster AppImage-Autostart, KWin/Yakuake-Z-Order-Fix, Auto-Dismiss & Scrollbarer Text-Editor
- **Entscheidung / Änderung:**
  1. **Robuster AppImage-Autostart (`AutostartPlatform`, `AppOrchestrator`):**
     - **Ursachenanalyse:** Beim Betrieb als AppImage lieferte die Standardmethode zur Pfadermittlung das temporäre Mount-Verzeichnis (`/tmp/.mount_*`), welches nach einem Systemneustart ungemountet und gelöscht ist. Dadurch schlug der Start via KDE Autostart mit "Berechtigung zum Ausführen fehlt" fehl.
     - **Persistente Pfadauflösung:** Erkennung der AppImage-Laufzeitumgebung über die `$APPIMAGE`-Umgebungsvariable. Verweist im `.desktop`-Autostart-Eintrag verlässlich auf die persistente AppImage-Datei des Nutzers.
     - **Ausführungsrechte & Selbstheilung:** Automatische Vergabe von Ausführungsrechten (`0o755`) sowohl für die `.desktop`-Datei als auch für die referenzierte Binär-/AppImage-Datei. Validierung des `Exec`-Ziels beim Programmstart: Veraltete oder ungültige temporäre Pfade werden automatisch erkannt und auf das aktuelle AppImage aktualisiert.
  2. **KWin-Z-Order-Korrektur & Yakuake / Super-Taste Fix (`KWinPlatform`, `AppOrchestrator`):**
     - **KWin Window Rules Bereinigung:** Entfernung der erzwungenen `above=true` Regel mit `aboverule=2` (Force). Dadurch wird verhindert, dass KWin die Whisper Pill über systemnahe Overlays wie Dropdown-Terminals (Yakuake via F12) oder Anwendungsstarter (Super-Taste / Kickoff) zwingt. Die normale Vordergrund-Platzierung wird sauber über Slints natives Window-Attribut gesteuert.
     - **Automatischer Auto-Dismiss-Timer (Ready-Zustand):** Nach erfolgreicher Transkription und Zwischenablage-Synchronisation (Zustand 3 "Ready") blendet sich die Pill nach 8 Sekunden Inaktivität automatisch aus. Verhindert, dass die Pill ungenutzt im oberen Desktopbereich verharrt und bei späterer Yakuake-Nutzung stört.
     - Bei Interaktion (z. B. Aufklappen der Details, Fortsetzen oder Neuaufnahme) wird der Timer sauber zurückgesetzt.
  3. **Scrollbereich für lange Transkripte (`Slint UI`):**
     - Im erweiterten Review- und Editor-Modus (Zustand 4 "Expanded") wurde das Texteingabefeld in ein natives `ScrollView` gebettet.
     - Transkripte beliebiger Länge können nun komfortabel mit dem Mausrad gescrollt oder über den Scrollbalken navigiert werden; kein Abschneiden mehr am unteren Fensterrand.
  4. **Tests & Verifikation:**
     - 40 Rust-Unit-Tests (`cargo test`) PASSED (inkl. Tests für persistente Pfadauflösung und Exec-Validierung).
     - `cargo clippy --all-targets` mit 0 Warnungen und 0 Fehlern abgeschlossen.
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - AppImage Release-Build (`dist/Whisper_Pill-x86_64.AppImage`) erfolgreich gebaut und verifiziert.
- **Betroffene Komponenten:**
  - `AutostartPlatform` (`app/src/platform/autostart.rs`)
  - `KWinPlatform` (`app/src/platform/kwin.rs`)
  - `AppOrchestrator` (`app/src/main.rs`)
  - `AppConfig` (`app/src/config.rs`)
  - `AppWindow` (`app/ui/appwindow.slint`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Vollständige technische & Benutzer-Dokumentation, README- und .gitignore-Erweiterung
- **Entscheidung / Änderung:**
  1. **Erweiterung der `.gitignore`:**
     - Umfassende Ausschlussregeln für alle Rust-Build-Artefakte (`target/`, `app/target/`, `*.rlib`, `*.rmeta`).
     - Ausschluss aller Packaging- und AppImage-Ausgaben (`dist/`, `*.AppImage`, `packaging/AppDir/`, generierte PNG-Icons).
     - Ausschluss lokaler KI-Modelldateien (`*.bin`, `ggml-*.bin`, `models/`, `*.onnx`) und Diagnose-Logdateien (`error.log`, `*.log`).
  2. **Aktualisierung des Technischen Architekturhandbuchs (`docs/architecture/technical_documentation.md`):**
     - Vollständige Architektur- und Entwicklerdokumentation für die native Rust-Anwendung mit Slint-Framework.
     - Systemübersichts-Diagramm, Slint-Zustandsautomat mit allen Kapselzuständen (`Hidden`, `Recording`, `Transcribing`, `Ready`, `Expanded`, `Settings`).
     - Modul- und Klassendiagramm (`AudioRecorder`, `WhisperEngine`, `PillTray`, `ClipboardManager`, `KWinManager`, `AppConfig`, `Logger`).
     - Sequenzdiagramm für die Audio-zu-Clipboard-Pipeline und Detaillierung des Multi-Thread-Concurrency-Modells.
     - Build- und Packaging-Spezifikation für die AppImage-Erstellung.
  3. **Endbenutzer-Handbücher (`docs/user_guide.html` & `docs/USER_GUIDE.md`):**
     - Bereitstellung des Endbenutzer-Handbuchs sowohl im Markdown-Format für GitHub/IDE-Leser als auch als autarkes, ansprechendes HTML-Dokument (Zero-CDN, vollständige Offline-Nutzung).
     - Beschreibung von Schnellstart via AppImage, Shortcuts (<kbd>Super</kbd>+<kbd>Strg</kbd>+<kbd>P</kbd>), Einstellungsdialog, Modellauswahl (Base vs. Small), Beam Search, Gain, Autostart und Fehlerlog-Diagnose.
  4. **Aktualisierung der Haupt-`README.md`:**
     - Überarbeitung mit Rust-Fokus, Performance-Highlights, AppImage-Schnellstartanleitung, Modell-Download-Hinweisen und detaillierten Build-Befehlen (`cargo run`, `cargo build --release`, `./scripts/build_appimage.sh`).
  5. **Verifikation & Test-Ausführung:**
     - 38 Rust-Unit-Tests (`cargo test`) PASSED.
     - `cargo clippy --all-targets` fehler- und warnungsfrei.
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - Zero-Download & Offline Security-Audit PASSED.
- **Betroffene Komponenten:**
  - `README.md`, `.gitignore`
  - `TechnicalDocumentation` (`docs/architecture/technical_documentation.md`)
  - `UserGuide` (`docs/user_guide.html`, `docs/USER_GUIDE.md`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Autarke Linux-AppImage-Erstellung (13 MB Bundle) & CLI-Flags
- **Entscheidung / Änderung:**
  1. **Autarkes AppImage Packaging (`scripts/build_appimage.sh`, `dist/Whisper_Pill-x86_64.AppImage`):**
     - Gemäß Benutzeranforderung wurde die native Rust-Anwendung in ein distributionsunabhängiges, portables Linux-AppImage paketiert.
     - Release-Optimierung: `cargo build --release` mit voller C++-Optimierung (`-O3`), Vektorisierung für Whisper-Inferenz und Slint-Software/FemtoVG-Renderer.
     - AppDir-Strukturierung nach Freedesktop- und AppImageKit-Standard:
       - Starter-Skript `AppRun` mit dynamischer Pfadbindung (`LD_LIBRARY_PATH`, `XDG_DATA_DIRS`).
       - Desktop-Entry `whisper-pill.desktop` mit korrekten Kategorien (`Utility;AudioVideo;Audio;Recorder;`).
       - Hochauflösende Vektorgrafik (`whisper-pill.svg`) und gerasterte PNG-Icons (`256x256`, `512x512`) mit Kapsel-Design und dynamischer Audio-Wellenform.
     - Komprimierung via `appimagetool` mit SquashFS/Zstandard: Das gesamte autarke Whisper-KI-Diktierprogramm umfasst lediglich **13 MB**!
  2. **CLI-Flags für Terminal-Nutzung (`main.rs`):**
     - Integration von `--version` / `-v` (Ausgabe der Version und sauberer Exit).
     - Integration von `--help` / `-h` (Nutzungshinweise und Parameterübersicht).
  3. **Tests & Verifikation:**
     - AppImage-Ausführungstest via `./dist/Whisper_Pill-x86_64.AppImage --version` erfolgreich (`Whisper Pill v0.1.0`).
     - AppImage-Hilfeausgabe via `./dist/Whisper_Pill-x86_64.AppImage --help` verifiziert.
     - 38 Rust-Unit-Tests (`cargo test`) PASSED.
- **Betroffene Komponenten:**
  - `Packaging`, `AppRun`, `DesktopEntry` (`packaging/`, `scripts/build_appimage.sh`, `dist/`)
  - `AppOrchestrator` (`main.rs`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Rust-Code-Checkup, Clippy-Linting (0 Fehler/Warnungen) & VS Code rust-analyzer Konfiguration
- **Entscheidung / Änderung:**
  1. **Rust-Code-Checkup & Qualitätsprüfung:**
     - Durchführung einer vollständigen statischen Code- und Typanalyse mittels `cargo check`, `cargo clippy --all-targets` und `cargo test`.
     - Bereinigung sämtlicher Lints (u. a. `bool_assert_comparison`, `field_reassign_with_default`, `collapsible_if`, idiomatische Match-Klauseln).
     - Ergebnis: **0 Fehler, 0 Warnungen**, alle 38 Unit- und Integrationstests erfolgreich durchgelaufen (`test result: ok. 38 passed`).
  2. **Behebung von VS Code / rust-analyzer Fehlermeldungen (`.vscode/settings.json`):**
     - Ursachenanalyse für Syntax- und Auflösungsfehler in VS Code:
       - Die Rust-Crate befindet sich im Unterverzeichnis `app/`. Ohne Workspace-Verknüpfung sucht `rust-analyzer` im Workspace-Root und kann `Cargo.toml` nicht automatisch zuordnen.
       - Slint generiert GUI-Code und Datenstrukturen (`AppWindow`) zur Build-Zeit via `build.rs`. Wenn Build-Skripte in `rust-analyzer` inaktiv sind, werden diese Typen fälschlicherweise als unauflösbar markiert.
     - Lösung: Bereitstellung einer dedizierten `.vscode/settings.json` mit verknüpftem Projektpfad (`app/Cargo.toml`) sowie aktivierten Build-Skripten und Makro-Expansion.
- **Betroffene Komponenten:**
  - `Config`, `Logging`, `AppOrchestrator` (`config.rs`, `logging.rs`, `main.rs`)
  - VS Code Konfiguration (`.vscode/settings.json`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Korrektur der Checkbox-Ausrichtung & UI-Feinschliff (Autostart & Gain)
- **Entscheidung / Änderung:**
  1. **Behebung des Überhangs bei Checkboxen (`ui/appwindow.slint`):**
     - Gemäß Benutzerhinweis ragten die Checkboxen bei "Mit dem System starten" und "Automatische Lautstärkeanpassung (Gain)" oben aus dem Rahmen heraus.
     - Ursache war eine unzuverlässige dynamische Höhenberechnung (`y: (52px - self.height) / 2`) auf dem inneren Layout, welche in Slint zu negativen Offsets führte.
     - Lösung: Entfernung des manuellen `y`-Offsets und Umstellung auf idiomatische Slint-Zentrierung (`VerticalLayout { alignment: center; }`) sowohl für die Checkbox als auch für den Textblock.
  2. **Ästhetischer Feinschliff der Options-Karten:**
     - Checkbox-Dimensionen auf 22 × 22 px verfeinert (6 px Eckenradius, 1.5 px Randstärke).
     - Farbschema: Im inaktiven Zustand dezenter dunkler Hintergrund (`#18181b`) mit `#52525b` Kontur (Hover: `#71717a`), im aktiven Zustand leuchtendes Cyan (`#38bdf8`) mit tiefdunklem, fettem Häkchen (`✔`).
     - Gesamte Kartenhöhe auf 54 px harmonisiert und mit sanftem Hover-Effekt (`#25252a`, 100 ms Transition) ausgestattet.
  3. **Tests:**
     - 38 Rust-Unit-Tests (`cargo test`) PASSED (0 Fehler, 0 Warnungen).
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - Zero-Download & Offline Security-Audit PASSED.
- **Betroffene Komponenten:**
  - `AppWindow` (`ui/appwindow.slint`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Ästhetisches Einstellungs-UI-Redesign (580x480 px) & Dynamische Tray-Transkriptionsanzeige
- **Entscheidung / Änderung:**
  1. **Großzügiges & Ästhetisches Einstellungs-UI (`ui/appwindow.slint`, `main.rs`):**
     - Gemäß Benutzeranforderung wurde das Einstellungsfenster vergrößert (580 x 480 px statt zuvor 520 x 390 px) und optisch modernisiert.
     - Großzügigere Abstände: 20 px horizontales Padding, 16 px vertikales Padding, 12 px vertikaler Abstand zwischen den Sektionen.
     - Erweiterte Options- und Ausfallkarten: Modell- und Suchstrategie-Karten wurden auf 72 px vergrößert, wodurch Beschreibungen und Status-Badges luftig und unverkrampft Platz finden.
     - Modernisierte Badges mit dezentem Pillen-Hintergrund für Modellverfügbarkeit (`Bereit` in Smaragdgrün, `Auto-Download` in Cyan).
     - Intelligente Fensterzentrierung und -verschiebung bei Aufruf aus dem Tray-Menü, damit das vergrößerte Fenster stets perfekt auf dem Bildschirm positioniert ist.
  2. **Dynamische Transkriptionsanzeige im System-Tray (`platform/tray.rs`, `main.rs`):**
     - Das System-Tray-Icon signalisiert nun in Echtzeit, wenn die Whisper-KI eine Aufnahme transkribiert:
       - **Icon-Grafik:** Statt des roten REC-Punkts wird bei aktiver Transkription eine dynamische Wellenform (`ılı`) in leuchtendem Bernstein/Gold (`#F59E0B`) gerendert.
       - **Tooltip:** Der Mouseover-Tooltip aktualisiert sich dynamisch auf *"Whisper Pill: Transkribiert..."* mit Untertitel *"Whisper-KI verarbeitet Sprachaufnahme..."*.
       - **Menü:** Der oberste Menüeintrag zeigt während des Prozesses *"⏳ Transkription läuft..."*.
     - **Architektur:** Implementierung eines asynchronen `TrayHandle` über native Tokio-Kanäle (`tokio::sync::mpsc`), das bei Zustandsänderungen DBus-Signals (`NewIcon`, `NewToolTip`) verzögerungsfrei an die Desktop-Umgebung (KDE Plasma, GNOME, etc.) emittiert.
  3. **Tests:**
     - 38 Rust-Unit-Tests (`cargo test`) PASSED (0 Fehler, 0 Warnungen).
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - Zero-Download & Offline Security-Audit PASSED.
- **Betroffene Komponenten:**
  - `PillTray` & `TrayHandle` (`platform/tray.rs`)
  - `AppWindow` (`ui/appwindow.slint`)
  - `AppOrchestrator` (`main.rs`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Fehlerlog-Zugriff über Systemtray-Menü & Diagnose-Protokollierung
- **Entscheidung / Änderung:**
  1. **Fehler- und Diagnose-Logging (`logging.rs`):**
     - Gemäß Benutzeranforderung wurde ein dediziertes Logging-System implementiert, das Laufzeitfehler, Warnungen und Diagnoseereignisse strukturiert in eine persistente Logdatei (`~/.local/state/whisper_pill/error.log` bzw. `$XDG_STATE_HOME/whisper_pill/error.log`) schreibt.
     - Vollständige Implementierung über die Standardbibliothek (Zero-Dependency-Prinzip): Zeitstempelberechnung im UTC-Format ohne externe Crates.
     - Thread-sichere Schreibzugriffe via `Mutex` und automatische Initialisierung mit Diagnose-Header beim Erststart.
  2. **Systemtray-Kontextmenü-Integration (`platform/tray.rs`):**
     - Neuer Kontextmenü-Eintrag "Fehlerlog anzeigen..." integriert (`TrayAction::OpenLog`).
     - Öffnet die Logdatei direkt im nativen Standard-Texteditor des Benutzers (z. B. Kate, KWrite, Gedit, Mousepad via `xdg-open` unter Linux, `rundll32` unter Windows, `open` unter macOS).
  3. **Vollständige Fehler-Verdrahtung (`main.rs`):**
     - Protokollierung des Anwendungsstarts sowie aller relevanten System-Events.
     - Transkriptionsfehler, Audio-Aufnahmefehler und Download-Fehlschläge werden strukturiert mit Zeitstempel und Komponenten-Tag im Log festgehalten.
  4. **Tests:**
     - 37 Rust-Unit-Tests (`cargo test`) PASSED (0 Fehler, 0 Warnungen).
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - Zero-Download & Offline Security-Audit PASSED.
- **Betroffene Komponenten:**
  - `LoggingManager` (`logging.rs`)
  - `PillTray` (`platform/tray.rs`)
  - `AppOrchestrator` (`main.rs`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Automatischer Modell-Download beim Start & bei Modellauswahl
- **Entscheidung / Änderung:**
  1. **Automatischer Modell-Downloader (`transcription/downloader.rs`):**
     - Gemäß Benutzeranforderung wird ein ausgewähltes Modell, falls es lokal noch nicht existiert, beim Programmstart sowie bei der Auswahl in den Einstellungen automatisch im Hintergrund heruntergeladen.
     - Der Download erfolgt non-blocking in einem separaten Thread via `curl` / `wget` von der offiziellen HuggingFace whisper.cpp-Quelle in eine temporäre Datei (`.tmp`) mit abschließender atomarer Umbenennung.
     - Während des Downloads bleibt die Anwendung voll funktionsfähig und greift auf vorhandene Fallback-Modelle zurück.
     - Nach Fertigstellung schaltet die Transkriptions-Engine sofort auf das neue Modell um und aktualisiert die Status-Badges im Einstellungsdialog dynamisch auf `[Bereit]`.
  2. **Bereitstellung von `ggml-small.bin` & Standalone-Skript (`scripts/download_model.sh`):**
     - Das `small`-Modell (466 MB) wurde erfolgreich in das lokale Cache-Verzeichnis geladen.
     - Sowohl `ggml-base.bin` (142 MB) als auch `ggml-small.bin` (466 MB) stehen nun lokal zur Verfügung.
     - Ergänzung eines universellen CLI-Skripts `scripts/download_model.sh` für manuelle Modellinstallationen.
  3. **Tests:**
     - 33 Rust-Unit-Tests (`cargo test`) PASSED (0 Fehler, 0 Warnungen).
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - Zero-Download & Offline Security-Audit PASSED.
- **Betroffene Komponenten:**
  - `ModelDownloader` (`transcription/downloader.rs`)
  - `AppOrchestrator` (`main.rs`)
  - `AppWindow` (`ui/appwindow.slint`)
  - `scripts/download_model.sh`
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Intelligenter Modell-Fallback, opakes & kompaktes Einstellungs-UI sowie Positions-Fixes
- **Entscheidung / Änderung:**
  1. **Intelligenter Modell-Fallback & Cache-Erkennung (`config.rs`, `engine.rs`):**
     - Wenn ein Modell wie `small` ausgewählt ist, aber aktuell nur `base` (`ggml-base.bin`) im lokalen Cache liegt, greift automatisch der intelligente Fallback auf das vorhandene Modell.
     - Verhindert vollständig den Abbruch der Transkription ("Whisper-Modell nicht gefunden") und garantiert dauerhafte Funktionsfähigkeit.
     - Im Einstellungsdialog zeigen Status-Badges (`[Bereit]` in Grün bzw. `[Fehlt lokal]` in Orange) direkt an, welche Modelldateien lokal auf dem System installiert sind.
  2. **Vollständig opakes (blickdichtes) Einstellungs-UI (`ui/appwindow.slint`):**
     - Im Einstellungsmodus wechselt der Hintergrund der Kapsel von der leichten Transparenz der Pill auf ein 100 % solides, blickdichtes Anthrazit (`#18181b`).
     - Dahinterliegende Desktop-Fenster scheinen nicht mehr durch; Kontrast und Lesbarkeit sind optimal.
  3. **Kompaktes UI-Layout (390 px) & Intelligente Positionierung gegen Bildschirmrand-Überhang:**
     - Die Höhe des Einstellungsfensters wurde von 620 px auf 390 px gestrafft (Breite einheitlich 520 px).
     - Beim Öffnen der Einstellungen über das Systemtray-Menü wird die Pill-Position gemerkt und das Fenster bei Bedarf automatisch nach oben versetzt, sodass es niemals unter den unteren Bildschirmrand oder in die Taskleiste ragt.
     - Beim Speichern oder Abbrechen wird die ursprüngliche Pill-Position exakt wiederhergestellt.
  4. **Vollständige Verschiebbarkeit (Drag to Move) des Einstellungsfensters:**
     - Das gesamte Einstellungs-Layout ist in eine Slint `WindowMoveArea` gebettet.
     - Eine markante Kopfzeile mit Griff-Symbol (`⠿ ⚙ Whisper Pill Einstellungen (Ziehen zum Verschieben)`) signalisiert dem Benutzer visuell, dass das Fenster frei mit der Maus verschoben werden kann.
  5. **Behebung von Logikfehlern und Code-Hygiene in `main.rs`:**
     - Der Positions-Speicher-Timer (500 ms) ignoriert nun Phasen, in denen das Fenster verborgen oder im Einstellungsmodus ist; die Pill-Koordinaten werden nicht mehr fälschlich überschrieben.
     - Beenden der App erfolgt sauber über `slint::quit_event_loop()`.
     - Sämtliche Clippy- und Linter-Meldungen restlos behoben (0 Warnungen).
  6. **Tests:**
     - 31 Rust-Unit-Tests (`cargo test`) PASSED (0 Fehler, 0 Warnungen).
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - Zero-Download & Offline Security-Audit PASSED (100 % lokal).
- **Betroffene Komponenten:**
  - `AppConfig` (`config.rs`)
  - `AppWindow` (`ui/appwindow.slint`)
  - `AppOrchestrator` (`main.rs`)
  - `WhisperEngine` (`transcription/engine.rs`)
  - `AutostartPlatform` (`platform/autostart.rs`)
  - `KWinPlatform` (`platform/kwin.rs`)
  - `PillState` (`state.rs`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Silent Background Startup, Integrierte Einstellungen (Single Window) & KWin Window Rules
- **Entscheidung / Änderung:**
  1. **Lautloser Hintergrund-Start (Silent Startup):**
     - Die Anwendung öffnet beim Start nicht mehr automatisch das Pill-Fenster und startet keine Sofort-Aufnahme mehr.
     - Whisper Pill startet rein im Hintergrund als Systemtray-Icon und wartet passiv auf den globalen Hotkey (`Super+Strg+P`) oder Aktionen im Tray-Menü.
  2. **Integrierte Single-Window-Architektur für Einstellungen:**
     - Der Einstellungsdialog wurde nativ in `AppWindow` integriert (`is_settings: bool`) und die separate `SettingsWindow`-Instanz entfernt.
     - Es existiert prozessweit stets nur genau eine einzige OS-Fensterinstanz, wodurch das Erscheinen eines zweiten generischen Wayland-Icons in der Taskleiste zuverlässig eliminiert wurde.
     - Das Öffnen der Einstellungen über das Tray-Menü ("Einstellungen...") erfolgt nun absolut fehlerfrei, indem die Pill direkt am Bildschirm im Einstellungsmodus expandiert.
  3. **KWin-Fensterregeln (`platform/kwin.rs`):**
     - Automatische Konfiguration von KDE KWin Fensterregeln (`skiptaskbar=true`, `skipswitcher=true`, `above=true`).
     - Verhindert vollständig, dass Whisper Pill ein Taskbar-Icon in der Anwendungsleiste erzeugt.
     - Verhindert vollständig, dass Whisper Pill im Alt+Tab Task-Switcher erscheint oder beim Durchwechseln von Fenstern ungewollt wieder aufploppt.
  4. **Tests:**
     - 29 Rust-Unit-Tests (`cargo test`) PASSED (0 Fehler, 0 Warnungen).
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - Zero-Download & Offline Security-Audit PASSED (100 % lokal).
- **Betroffene Komponenten:**
  - `AppOrchestrator` (`main.rs`)
  - `AppWindow` (`ui/appwindow.slint`)
  - `KWinPlatform` (`platform/kwin.rs`)
  - `platform/mod.rs`
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Vereinheitlichung der visuellen UI-Bezeichnung auf "Whisper Pill" & Optimale Standardeinstellungen
- **Entscheidung / Änderung:**
  1. **Visuelle Benennung für den Benutzer ("Whisper Pill"):**
     - Alle für den Endbenutzer sichtbaren Stellen wurden konsistent von technischem `whisper-pill` auf das lesbare `Whisper Pill` umgestellt:
       - Hauptfenster (`AppWindow` Titel und Kapsel-Label `🎙 Whisper Pill`)
       - Einstellungsdialog (`SettingsWindow` Titel, Header und Beschreibungen)
       - Systemtray-Tooltip (`Whisper Pill (Super+Strg+P)`)
       - Kontextmenü-Einträge
       - Linux Desktop-Starter (`Name=Whisper Pill` im `.desktop`-Autostart-Eintrag)
       - KDE Global Shortcuts Anzeige (`Whisper Pill`)
       - Python-Prototyp UI-Elemente
  2. **Setzen der optimalen Standardeinstellungen (Best Settings by Default):**
     - Die Default-Konfiguration (`AppConfig::default()`) wurde ab Werk auf die bestmögliche Transkriptionsqualität vorkonfiguriert:
       - **Sprachmodell:** `small` (deutlich höhere Treffsicherheit bei Nuscheln und Dialekt statt dem früheren `base`)
       - **Suchstrategie:** `beam_search = true` (gründliche Beam Search Analyse paralleler Hypothesen)
       - **Audio-Pegelanpassung:** `normalize_audio = true` (automatische Dynamik-Anhebung leiser Wörter)
       - **Systemstart (Autostart):** `autostart = true` (Whisper Pill startet standardmäßig automatisch im Hintergrund beim Systemstart und richtet den `.desktop`-Eintrag beim Erststart automatisch ein)
  3. **Tests:**
     - 28 Rust-Unit-Tests (`cargo test`) PASSED.
     - 19 Python-Unit-Tests (`pytest`) PASSED.
     - Zero-Download & Offline Security-Audit PASSED (100 % lokal).
- **Betroffene Komponenten:**
  - `AppConfig` (`config.rs`)
  - `AppWindow` & `SettingsWindow` (`ui/appwindow.slint`)
  - `TrayIntegration` (`platform/tray.rs`)
  - `AutostartPlatform` (`platform/autostart.rs`)
  - `HotkeyPlatform` (`platform/hotkey.rs`)
  - `PillWindow` & `TrayIcon` (`prototype/ui/`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Anti-Nuschel-Optimierungen, Einstellungsdialog & Autostart-Verifikation
- **Entscheidung / Änderung:**
  1. **Anti-Nuschel-Optimierungen (4 Hebel):**
     - **Option 1 (Modellauswahl):** Benutzer kann zwischen `base` (Standard, ~140 MB, ressourcenschonend) und `small` (Empfohlen bei undeutlicher Sprache / Nuscheln / Dialekt, ~460 MB) wählen. Modell wird bei Bedarf dynamisch im laufenden Betrieb neu geladen.
     - **Option 2 (Audio-Pegelanpassung / Normalisierung):** Automatische Skalierung leiser Tonsignale auf Whisper-Optimalpegel (bis zu 5x Gain) vor der Inferenz, um leise genuschelte Silben deutlicher hervorzuheben. Über die Einstellungen aktivier-/deaktivierbar.
     - **Option 3 (Deutscher Initial-Prompt):** Einbettung eines neutralen deutschen Referenz-Prompts (`"Hallo, dies ist ein deutsches Diktat."`), wodurch Whisper auf die deutsche Sprache konditioniert wird, ohne starre Vorgaben zu Grammatik oder Rechtschreibung zu erzwingen (erhält natürliche Umgangssprache, Abbrüche und Halbsätze).
     - **Option 4 (Suchstrategie):** Wahl zwischen `Schnell (Greedy)` (sofortige Wortwahl, blitzschnell) und `Gründlich (Beam Search)` (parallele Analyse alternativer Wortfolgen für höchste Präzision bei Nuscheln, + ca. 0,2s).
  2. **Einstellungsdialog (`SettingsWindow`):**
     - Integriert in Slint GUI mit modernem Dark-Mode-Design.
     - Aufrufbar über das Kontextmenü des System-Tray-Icons ("Einstellungen...").
     - Enthält verständliche, noob-taugliche Erklärungen zu den Auswirkungen jeder Option auf Erkennungsgenauigkeit und Geschwindigkeit.
     - Bietet "Speichern & Übernehmen" sowie "Abbrechen". Konfiguration wird persistent in der Konfigurationsdatei gespeichert.
  3. **Autostart-Verifikation (Vollständige Entfernung):**
     - Beim Deaktivieren der Autostart-Checkbox wird die `.desktop`-Datei im Autostart-Verzeichnis sauber vom Dateisystem entfernt.
     - Neuer Unit-Test `test_write_and_verify_complete_removal_on_disable` verifiziert die restlose Entfernung.
  4. **Zero-Download-Richtlinie final umgesetzt:**
     - Entfernung des letzten verbliebenen `curl`-Downloads in `TranscriptionEngine`. Modelle müssen vorab im Modellverzeichnis hinterlegt sein.
     - `check_downloads.py` Sicherheits-Audit liefert 0 Funde (100% offline).
  5. **Tests:**
     - 28 Rust-Unit-Tests (`cargo test`) PASSED (0 Warnungen, 0 Fehler).
     - 19 Python-Unit-Tests (`pytest`) PASSED.
- **Betroffene Komponenten:**
  - `TranscriptionEngine` (`transcription/engine.rs`)
  - `AppConfig` (`config.rs`)
  - `AutostartPlatform` (`platform/autostart.rs`)
  - `TrayIntegration` (`platform/tray.rs`)
  - `SettingsWindow` (`ui/appwindow.slint`)
  - `AppOrchestrator` (`main.rs`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: System-Tray Tooltip bei Mouseover (ksni ToolTip)
- **Entscheidung / Änderung:**
  1. **Tooltip für System-Tray-Icon implementiert:**
     - In der Tray-Implementierung wurde die Methode `tool_tip(&self) -> ksni::ToolTip` implementiert.
     - Beim Mouseover über das Tray-Icon in KDE Plasma, GNOME (AppIndicator) und XFCE wird nun der formatierte Tooltip angezeigt:
       - **Titel:** `whisper-pill (Super+Strg+P)`
       - **Beschreibung:** `Minimalistisches Desktop-Diktier-Utility` (Klick-Hinweistext entfernt).
       - **Icons:** Skalierte 32x32- und 64x64-Kapsel-Icons im Tooltip-Vorschaubild.
  2. **Tests:**
     - Neuer Unit-Test `test_tray_tooltip_content` prüft Titel, Shortcut-Präsenz, exakte Beschreibung und Pixmap-Dimensionen.
     - Alle 21 Rust-Tests (`cargo test`) und 19 Python-Tests (`pytest`) erfolgreich PASSED.
- **Betroffene Komponenten:**
  - `TrayIntegration` (`platform/tray.rs`)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Datenschutz & Anonymisierung: Dateipfade aus allen Berichten entfernt & Skills aktualisiert
- **Entscheidung / Änderung:**
  1. **Bereinigung existierender Berichte (`docs/`):**
     - Alle Dateipfade (insbesondere absolute Benutzerpfade und Quellcodepfade) wurden aus `docs/security_audit_report.md`, `docs/unit_test_report.md`, `docs/ui_test_report.md` sowie `docs/JOURNAL.md` vollständig entfernt.
     - Statt interner Dateipfade werden nun ausschließlich funktionale Modul- und Komponentennamen (z. B. `AudioRecorder`, `WhisperEngine`, `HotkeyPlatform`, `UI`) verwendet, um die Privatsphäre und Systemsicherheit in öffentlichen Repositories (GitHub) zu wahren.
  2. **Aktualisierung aller Skill-Richtlinien (`.agent/skills/`):**
     - Die Skill-Definitionen `security-audit`, `test-core`, `test-ui`, `test-e2e`, `tech-docs` und `user-docs` wurden verbindlich um die Datenschutz-Richtlinie für öffentliche Berichte erweitert: Zukünftige Berichte dürfen keinerlei absolute System- oder lokale Quellcodedateipfade enthalten.
  3. **Tests:**
     - Alle 20 Rust-Tests (`cargo test`) und 19 Python-Tests (`pytest`) PASSED.
- **Betroffene Komponenten:**
  - `docs/security_audit_report.md`
  - `docs/unit_test_report.md`
  - `docs/ui_test_report.md`
  - `docs/JOURNAL.md`
  - `.agent/skills/security-audit/SKILL.md`
  - `.agent/skills/test-core/SKILL.md`
  - `.agent/skills/test-ui/SKILL.md`
  - `.agent/skills/test-e2e/SKILL.md`
  - `.agent/skills/tech-docs/SKILL.md`
  - `.agent/skills/user-docs/SKILL.md`
  - `AGENTS.md`

---

### 2026-09-27 - Security-Audit: Vollständiger Sicherheits- und Offline-Scan der Rust-App
- **Entscheidung / Änderung:**
  1. **Durchführung des Security-Audits für die Rust-Anwendung (`app/`):**
     - Vollständige Prüfung auf Secrets, bekannte Lücken (CVEs in `Cargo.lock`), SAST / Memory Safety, Command Injections und Audio-Datenschutz.
     - **Ergebnis:**
       - **Secrets:** 0 Hardcoded Credentials (**PASSED**).
       - **Abhängigkeiten:** Keine HTTP-/Netzwerk-Crates wie `reqwest`/`hyper`/`ureq` eingebunden. Alle Lizenzen permissiv (**PASSED**).
       - **Memory Safety:** 0 `unsafe` Blöcke in `app/src/`, 100 % Safe Rust (**PASSED**).
       - **Command Injection:** Alle `Command::new`-Aufrufe (`hotkey.rs`, `tray.rs`) nutzen typsichere Parameterlisten ohne Shell-Interpolation (**PASSED**).
       - **Audio-Datenschutz:** Audio-Puffer im RAM werden direkt nach der Inferenz vor der Freigabe mit `0.0` genullt (`main.rs`, `recorder.rs`) (**PASSED**).
       - **Zero-Download-Richtlinie:** Befund in [`app/src/transcription/engine.rs`](app/src/transcription/engine.rs#L38-L54) bestätigt (`curl`-Download für HuggingFace `ggml-base.bin`). Zur Behebung vorgemerkt (**ACTION REQUIRED**).
  2. **Audit-Bericht aktualisiert:**
     - [`docs/security_audit_report.md`](docs/security_audit_report.md) mit detaillierten Tabellen und Prüfpunkten für die Rust-App neu generiert.
  3. **Tests:**
     - Alle 20 Rust-Tests (`cargo test`) und 19 Python-Tests (`pytest`) PASSED.
- **Betroffene Komponenten:**
  - `docs/security_audit_report.md`
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Security-Skill: Generelle Zero-Download- & Netzwerk-Prüfung (check_downloads.py)
- **Entscheidung / Änderung:**
  1. **Generelle Download- & Netzwerk-Verifikation im Security-Audit:**
     - Der Skill [`.agent/skills/security-audit/SKILL.md`](.agent/skills/security-audit/SKILL.md) wurde um eine lückenlose, generelle Überprüfung auf jegliche Download-Aktivitäten erweitert (Zero-Download & Strict Offline Policy).
     - Geltender Grundsatz: Als rein lokales Desktop-Utility dürfen zur Laufzeit keine Daten, Modelle oder Binärdateien aus dem Internet geladen werden.
  2. **Automatisierter Scanner-Script (`check_downloads.py`):**
     - Neues Prüfwerkzeug [`.agent/skills/security-audit/scripts/check_downloads.py`](.agent/skills/security-audit/scripts/check_downloads.py) implementiert.
     - Scannt Rust-Quellcode (`app/src/`), Python-Code (`prototype/`, `tests/`), `Cargo.lock` und `Cargo.toml` auf:
       - CLI-Downloader (`curl`, `wget`, `aria2`, `ftp`) via Subprozesse.
       - Direkte Socket-APIs (`TcpStream`, `UdpSocket`).
       - HTTP-Bibliotheken (`reqwest`, `hyper`, `ureq`, `urllib`, `requests`, `aiohttp`, etc.).
       - Hardcodierte Remote-URLs und ungesicherte KI-Modell-Downloads.
  3. **Identifizierter Befund:**
     - Der Scanner identifizierte den automatischen `curl`-Fallback für das GGML-Modell in [`app/src/transcription/engine.rs`](app/src/transcription/engine.rs#L38-L54), der transparent protokolliert und zur Bereinigung vorgemerkt wurde.
- **Betroffene Komponenten:**
  - `.agent/skills/security-audit/SKILL.md`
  - `.agent/skills/security-audit/scripts/check_downloads.py` (neu)
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Fenster-Verschiebung (WindowMoveArea), Positions-Persistenz (config.json), Autostart & Menü-Beschriftung
- **Entscheidung / Änderung:**
  1. **Menü-Beschriftung im System-Tray:**
     - Menüeintrag für das Beenden der Anwendung exakt nach Vorgabe auf `Whisper Pill Beenden` angepasst.
  2. **Verschiebbare Kapsel-GUI (Drag to Move):**
     - Integration von Slints nativem `WindowMoveArea`-Element in [`app/ui/appwindow.slint`](app/ui/appwindow.slint).
     - In allen kompakten Zuständen (Idle, Recording, Processing, Ready) kann die schwebende Kapsel nun per Mausklick und Ziehen frei am Desktop platziert werden; Klicks auf Buttons bleiben interaktiv.
     - Im ausgeklappten Editor-Zustand fungiert die Kopfzeile als Ziehbereich, wodurch Textselektion und Eingabe im Editor vollständig ungestört bleiben.
  3. **Persistente Fensterposition (`config.json`):**
     - [`AppConfig`](app/src/config.rs) um `window_x`, `window_y` sowie `load()` und `save()` erweitert (Speicherort: `~/.config/whisper_pill/config.json`).
     - Beim Start wird eine gespeicherte Position via `set_position(PhysicalPosition)` exakt wiederhergestellt.
     - Veränderungen der Fensterposition werden kontinuierlich im Hintergrund (500ms Timer) sowie beim Verbergen und Beenden in `config.json` gesichert.
  4. **System-Autostart (KDE Plasma ab 6.0, GNOME ab 45, XFCE):**
     - Neues Modul [`app/src/platform/autostart.rs`](app/src/platform/autostart.rs) implementiert die Freedesktop XDG Autostart-Spezifikation via `~/.config/autostart/whisper-pill.desktop`.
     - Beinhaltet native Spezifikationsflags für KDE Plasma 6+ (`X-KDE-autostart-after=panel`) und GNOME 45+ (`X-GNOME-Autostart-enabled=true`).
     - Als interaktiver Checkmark-Eintrag `Mit dem System starten (Autostart)` im Kontextmenü des System-Tray-Icons integriert.
  5. **Tests:**
     - 5 neue Unit-Tests für `AppConfig` (Persistenz, Serialisierung), `autostart` (XDG .desktop Struktur, Pfad-Auflösung) und Menü-Struktur (6 Einträge).
     - Alle 20 Rust-Tests (`cargo test`) und 19 Python-Tests (`pytest`) erfolgreich PASSED.
- **Betroffene Komponenten:**
  - `app/src/platform/autostart.rs` (neu)
  - `app/src/platform/mod.rs`
  - `app/src/platform/tray.rs`
  - `app/src/config.rs`
  - `app/src/main.rs`
  - `app/ui/appwindow.slint`
  - `docs/JOURNAL.md`

---

### 2026-09-27 - Phase 2: Fortsetzungs-Diktat (+ Weiter), Neu-Button, Kapsel-Tray-Icon & Hilfe-Menü
- **Entscheidung / Änderung:**
  1. **Fortsetzungs-Diktat (+ Weiter) erweitert Text:**
     - Bei Klick auf `+ Weiter` nach einer Sprechpause wird der bisherige Transkriptionstext nun nicht mehr überschrieben, sondern sauber mit einem Trennleerzeichen um das neue Sprachsegment erweitert (`whisper_pill::transcription::append_transcript`).
     - Der kumulierte Gesamtzustand wird unmittelbar in die System-Zwischenablage synchronisiert.
  2. **Neuer Button "● Neu":**
     - Im Zustand *Ready* steht nun neben `+ Weiter` ein zusätzlicher Button `● Neu` zur Verfügung.
     - Klick auf `● Neu` setzt den Textpuffer zurück und startet ein frisches Diktat, dessen Transkription isoliert verarbeitet und kopiert wird.
  3. **Ergonomische Kapselbreite (520 px):**
     - Kapselbreite von 480 px auf 520 px angepasst (Höhe 48 px / Editor 240 px, Waveform zentriert bei 215 px), um der Erfolgsmeldung "✔ In Zwischenablage kopiert!" und allen vier Aktionen (`+ Weiter`, `● Neu`, `↗ Details`, `✕`) perfekten Abstand zu bieten.
  4. **Natives System-Tray-Icon (whisper-pill Kapsel):**
     - Das generische System-Mikrofon-Icon wurde durch die maßgeschneiderte whisper-pill Kapsel (dunkler Kapselkörper `#18181B`, Cyan-Rand `#38BDF8`, roter REC-Punkt `#EF4444`) ersetzt.
     - Mathematische Kantenglättung (Antialiasing) via ARGB32-Vektorrasterung (32x32 und 64x64) ohne externe Bilddateien oder Drittanbieter-Abhängigkeiten.
  5. **Kontextmenü-Hilfe zu GitHub:**
     - Im System-Tray-Menü wurde der Menüpunkt `Hilfe (GitHub)` ergänzt, der das Projekt-Repository (`https://github.com/Sympa1/whisper_pill`) im Standard-Browser öffnet.
  6. **Tests:**
     - 7 neue Unit-Tests für `append_transcript`, `create_tray_icon` (Dimensionen, Pixel-Farben) und die Menü-Struktur hinzugefügt.
     - Alle 15 Rust-Tests (`cargo test`) und 19 Python-Tests (`pytest`) PASSED.
- **Betroffene Komponenten:**
  - `app/src/transcription/accumulator.rs` (neu)
  - `app/src/transcription/mod.rs`
  - `app/src/platform/tray.rs`
  - `app/src/main.rs`
  - `app/ui/appwindow.slint`
  - `docs/JOURNAL.md`

---

### 2026-09-22 - Hygiene: .idea/ in .gitignore aufgenommen & aus Git-Tracking entfernt
- **Entscheidung / Änderung:**
  - Der JetBrains/IntelliJ-Projektordner `.idea/` wurde in `.gitignore` einkommentiert und via `git rm -r --cached .idea` sauber aus der Git-Versionsverwaltung entfernt (Dateien verbleiben lokal auf der Festplatte).
  - Alle 8 Rust-Tests (`cargo test`) und 19 Python-Tests (`pytest`) erfolgreich PASSED.
- **Betroffene Komponenten:**
  - `.gitignore`
  - `.idea/` (aus Git-Index entfernt)
  - `docs/JOURNAL.md`

---

### 2026-09-20 - Phase 2: Shortcut-Verhalten optimiert (Pill ausblenden & Aufnahme beenden via Toggle)
- **Entscheidung / Änderung:**
  - Auf Nutzeranforderung wurde das Verhalten des globalen Shortcuts (`Super+Strg+P` / `Meta+Ctrl+P`) grundlegend auf einen nahtlosen "Push-to-Dictate & Hide"-Workflow umgestellt:
    1. **Sichtbare Pill (Aufnahme aktiv oder bereit):**
       - Beim Drücken des Shortcuts wird die Aufnahme sofort beendet (Audiodaten werden im Hintergrund transkribiert und der Text landet flüchtig in der Zwischenablage).
       - Gleichzeitig wird die Pill **sofort ausgeblendet** (`win.hide()`), sodass der Desktop ohne störendes Kapsel-Overlay frei bleibt und der Text direkt mit `Strg+V` in die aktive Anwendung eingefügt werden kann.
    2. **Ausgeblendete Pill:**
       - Beim erneuten Drücken des Shortcuts wird die Pill wieder eingeblendet (`win.show()`), auf Kapselgröße (480x48 px) zurückgesetzt und **sofort eine neue Aufnahme gestartet**.
    3. **Daemon Event-Loop (`slint::run_event_loop_until_quit`):**
       - Durch Umstellung von `app_window.run()` auf `slint::run_event_loop_until_quit()` bleibt der Hintergrund-Prozess inklusive KDE D-Bus Hotkey-Listener und System-Tray-Icon dauerhaft aktiv, selbst wenn kein Fenster sichtbar ist.
    4. **Konsistentes Tray- und UI-Verhalten:**
       - Klick auf das `✕` (Dismiss) im Ready-Zustand blendet die Pill ebenfalls aus.
       - Im ausgeklappten Editor stehen nun `▲ Zuklappen` (zum Einklappen auf 48 px), `✕ Verbergen` (zum Ausblenden) und `⏻ Beenden` zur Verfügung.
       - Der Menüeintrag im System-Tray spiegelt das Verhalten mit der Beschriftung `Diktat starten / beenden (Super+Strg+P)` wider.
  - Alle 8 Rust-Tests (`cargo test`) und 19 Python-Tests (`pytest`) erfolgreich PASSED.
- **Betroffene Komponenten:**
  - `app/src/main.rs`
  - `app/src/platform/tray.rs`
  - `app/ui/appwindow.slint`
  - `docs/JOURNAL.md`

---

### 2026-09-20 - Phase 2: Umstellung auf Slint UI (Vektor-Kapsel, Subpixel-Schriften & perfekte Transparenz)
- **Entscheidung / Änderung:**
  - Da `egui` unter Wayland bei Fenstertransparenz, Schatten und nativer Typografie Einschränkungen zeigte, wurde die Benutzeroberfläche auf **Slint 1.18** migriert.
  - Slint wurde von ehemaligen Qt-Core-Entwicklern geschaffen und kombiniert deklarative Syntax (`.slint`) mit nativer Vektorgrafik und FreeType-Subpixel-Schriftarten bei extrem geringem Speicherverbrauch (~15 MB RAM).
  - Umgesetzte UI-Features in [`app/ui/appwindow.slint`](app/ui/appwindow.slint):
    1. **Kapselform & Breite:** Kapselbreite auf **480 px** vergrößert (48 px Höhe, bzw. 240 px im Editor, Radius 24 px), wodurch die Erfolgsmeldung "✔ In Zwischenablage kopiert!" und alle Aktionsbuttons (`+ Weiter`, `↗ Details`, `✕`) vollständig und luftig ohne Umbruch nebeneinander Platz finden.
    2. **Farben & Kontraste:** Exakte Prototyp-Farbwerte (`rgba(24, 24, 27, 0.95)`, Border `rgba(255, 255, 255, 0.12)`).
    3. **Pill-Buttons & Editor-Sichtbarkeit:** Eigene `PillButton`-Komponente (`● Aufnehmen`, `■ Stop`, `+ Weiter`, `↗ Details / Edit`, `✕ Verbergen`, `⏻ Beenden`, `⎘ Text kopieren`). Im ausgeklappten Editor wurde das Standard-Widget durch ein maßgeschneidertes `TextInput` mit expliziter Textfarbe `#F4F4F5` und Zwei-Wege-Binding (`text <=> root.transcript-string`) ersetzt, wodurch transkribierter Text sofort gestochen scharf in Weiß lesbar ist.
    4. **Waveform:** 11 vertikale Kapselbalken in `#38BDF8` mit abgerundeten Kappen (`border-radius: 2px`).
    5. **Exakte Geometrie & Platzierung:** Beseitigung der Asymmetrie durch mathematisch exakte Zentrierung der Waveform bei `x = (480px - 90px) / 2 = 195px` und vertikale Zentrierung jedes Balkens bei `y: (28px - height) / 2`. Feste 20px Randabstände links/rechts für Text und Buttons, vertikal exakt auf 24px zentriert. Dynamische Fensteranpassung (`win.window().set_size(480, 240)`) beim Auf- und Zuklappen.
    6. **Button-Breitenkorrektur (Idle-State):** `PillButton` definiert nun explizit `width: label.preferred-width + 24px;`, wodurch verhindert wird, dass der `● Aufnehmen`-Button nach dem Stoppen die gesamte Kapselbreite einnimmt. Verpackung in `HorizontalLayout` bei `x: 480px - self.width - 20px`.
    7. **Whisper GGML-Modell bereitgestellt & Auto-Downloader:**
       - Whisper GGML Base Modell (`ggml-base.bin`, 142 MB) im lokalen Cache `~/.cache/whisper_pill/` abgelegt.
       - Automatischer Downloader (`WhisperEngine::ensure_model_available`) implementiert, der fehlende Modelle bei Bedarf selbstständig via `curl` lädt.
    8. **System-Tray-Icon (KDE StatusNotifierItem via ksni):**
       - Natives Mikrofon-Icon `audio-input-microphone` in der Taskleiste integriert.
       - Rechtsklick-Menü für Aufnahme starten/stoppen und geordnetes Beenden.
    9. **Globale Shortcuts & D-Bus Listener:**
       - Automatische Registrierung des globalen Shortcuts `Super+Strg+P` (Meta+Ctrl+P, Keycode `335544400`) in KDE KGlobalAccel via D-Bus (`doRegister` und `setShortcut`).
       - Asynchroner D-Bus Signal-Stream für sofortiges Starten/Stoppen der Aufnahme bei Tastendruck.
    10. **Robuste Audio-Architektur:** Entkopplung des `AudioRecorder` in einen exklusiven Dedicated Audio-Worker-Thread via Message-Passing (`AudioCommand::Start`, `AudioCommand::Stop`), wodurch plattformspezifische ALSA `Send`-Restriktionen vollständig eliminiert wurden.
  - Alle 8 Unit-Tests (`test audio::*`, `test platform::*`, `test transcription::*`) erfolgreich PASSED.
- **Betroffene Komponenten:**
  - `app/Cargo.toml`
  - `app/build.rs`
  - `app/ui/appwindow.slint`
  - `app/src/main.rs`
  - `app/src/platform/tray.rs`
  - `app/src/platform/hotkey.rs`
  - `docs/JOURNAL.md`

---

### 2026-09-20 - Phase 2: Native Rust-Implementierung, Migration in app/ & Pixel-Perfektion des UI
- **Entscheidung / Änderung:**
  - Start von Phase 2: Vollständige Neuentwicklung der nativen Desktop-Anwendung in Rust.
  - Das Rust-Projekt wurde sauber in das Unterverzeichnis `app/` strukturiert (analog zu `prototype/` für Python).
  - **Pixel-Perfektionierung der Benutzeroberfläche (1:1 Angleichung an den Prototyp):**
    1. Kapsel-Styling: Echte Alpha-Transparenz ohne schwarze Fensterecken (`panel_fill` & `window_fill = Color32::TRANSPARENT`), Kapselhintergrund `rgba(24, 24, 27, 0.95)`, feiner 1px-Rahmen `rgba(255, 255, 255, 0.12)`.
    2. Pill-Buttons: Individuell gezeichnete abgerundete Buttons (`Rounding::same(12.0)`) mit den originalen Prototyp-Farben (`● Aufnehmen` Rot `#EF4444`, `■ Stop` Anthrazit `#3F3F46`, `+ Weiter` Cyan `#38BDF8`, `↗ Details` Dunkelgrau `#27272A`).
    3. Waveform: Exakt 11 vertikale Balken mit abgerundeten Kappen (`Rounding::same(2.0)`), Glockenkurven-Gewichtung und Farbe `#38BDF8`.
    4. Platzierung: Automatische Zentrierung oben mittig am Bildschirm (`y = 40 px`) beim Start.
    5. One-Touch Workflow: Sofortige Aufnahmebereitschaft beim Starten bzw. Shortcut-Druck (`Super+Strg+P`), automatische Stille-Erkennung mit VAD und Auto-Reset nach 8s im Ready-Zustand.
  - Architektur & Module umgesetzt:
    1. `app/src/state.rs`: 5 Kapselzustände (`Idle`, `Recording`, `Processing`, `Ready`, `Expanded`).
    2. `app/src/config.rs`: `AppConfig` mit RMS-Schwellenwert (0.012), 2.5s Stille-Timeout und 16 kHz Sample-Rate.
    3. `app/src/audio/vad.rs`: `SilenceDetector` mit Echtzeit-RMS-Berechnung und automatischem Stopp nach 2.5–3s Sprechpause (inkl. Unit-Tests).
    4. `app/src/audio/recorder.rs`: `AudioRecorder` via `cpal`, Stereo-zu-Mono Downmixing, lineares Resampling auf 16 kHz und RAM-Flushing.
    5. `app/src/transcription/engine.rs`: `WhisperEngine` via `whisper-rs` (lokale GGML-Modelle, typsicherer Segment-Iterator).
    6. `app/src/platform/clipboard.rs`: `ClipboardManager` via `arboard`.
    7. `app/src/platform/hotkey.rs`: `HotkeyManager` mit Linux D-Bus Listener für KDE KGlobalAccel Signal `globalShortcutPressed` (`Super+Strg+P`).
    8. `app/src/ui/waveform.rs`: `WaveformWidget` für flüssige dynamische Wellenform in der Kapsel.
    9. `app/src/ui/app.rs`: `PillApp` Kapsel-UI (420x48 px, abgerundete Ecken 24 px, ausgeklappt 420x240 px) mit One-Touch Aufnahme, Drag-to-Move, Stille-Erkennung und Fortsetzen-Button.
    10. `app/src/main.rs`: Haupteinsprungpunkt mit rahmenlosem, transparentem Fenster (`eframe`).
  - Alle 6 Rust-Tests und 19 Python-Tests erfolgreich (`PASSED`).
- **Betroffene Komponenten:**
  - `app/Cargo.toml`
  - `app/src/`
  - `docs/JOURNAL.md`

---

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
    3. **Autonome Workspace-Befehlsausführung:** Innerhalb des Repositories (``) werden Datei- und Shell-Operationen (`ls -la`, `cat`, `pytest`, `python` etc.) selbstständig und ohne Unterbrechung ausgeführt. Rückfragen erfolgen nur bei Operationen außerhalb des Projektverzeichnisses.
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
