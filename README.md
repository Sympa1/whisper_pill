# Whisper Pill

> **Schlankes, minimalistisches, schwebendes Desktop-Diktier-Utility für Linux und Windows – 100 % lokal, quelloffen und datenschutzfokussiert.**

`Whisper Pill` ist eine freie Open-Source-Alternative zu kommerziellen Diktierwerkzeugen wie Whisperbar. Die Anwendung läuft als native **Rust-Desktop-App** ressourcenschonend im System-Tray, blendet bei Bedarf eine schwebende Kapsel („Pill“) am Bildschirmrand ein und wandelt gesprochene Sprache per Tastendruck lokal in präzisen Text um – inklusive automatischer Satzzeichen- und Kommasetzung. Das fertige Transkript landet sofort in der System-Zwischenablage und steht direkt per <kbd>Strg</kbd> + <kbd>V</kbd> zum Einfügen bereit.

---

## Projektbeschreibung

Nachdem das Konzept zunächst als Prototyp validiert wurde, bildet die **native Rust-Implementierung (`app/`)** das Herzstück des Projekts. Sie wurde gezielt für minimale Latenz, geringsten RAM-Verbrauch und eine autarke Auslieferung ohne Laufzeit-Abhängigkeiten entwickelt:

* **Tray-First-Workflow:** Beim Start nistet sich Whisper Pill unaufdringlich im System-Tray ein, ohne Platz auf dem Desktop oder in der Taskleiste zu belegen.
* **Deklaratives Slint-UI (`Slint 1.18`):** Reaktionsschnelle, hardwarebeschleunigte Oberfläche ohne schweren Browser- oder Qt-Overhead.
* **Lokale KI-Inferenz (`whisper-rs` / `whisper.cpp`):** 100 % lokale CPU-Spracherkennung mit offiziellen GGML-Modellen (`base` und `small`). Kein MCP-Server, kein Cloud-Backend, keine Telemetrie.
* **Low-Latency Audio-Pipeline (`cpal`):** Echtzeit-Resampling auf 16 kHz Mono Float32, Live-Pegelberechnung (RMS), optionale Gain-Normalisierung und automatische Stille-Erkennung (VAD).
* **Native Desktop-Integration (`ksni`, `zbus`, `arboard`):** D-Bus StatusNotifierItem-Systemtray mit Live-Status-Icon, isolierte globale Shortcuts über KDE `KGlobalAccel`, automatische KWin-Fensterregeln sowie selbstheilender System-Autostart (auch für portable AppImages).

---

## UI-Zustände & Screenshots (Rust & Slint UI)

Die schwebende Kapsel behält über alle kompakten Zustände hinweg eine **konstante Breite von 520 px** bei. Sie bleibt stabil am Bildschirmrand verankert, springt beim Zustandswechsel nicht und dehnt sich nur beim Öffnen des Editors oder der Einstellungen aus.

### 1. Ruhezustand (Idle)
Minimalistisch und unaufdringlich am Bildschirmrand (`520 × 48 px`, 0 % CPU-Last im Standby):

![Ruhezustand](docs/assets/state_0_idle.png)

### 2. Aufnahme (Recording)
Aktiviert per Hotkey (<kbd>Super</kbd> + <kbd>Strg</kbd> + <kbd>P</kbd>) oder Klick; visualisiert den Live-Lautstärkepegel über 11 dynamische Wellenform-Balken samt Aufnahme-Timer und `[■ Stop]`-Button:

![Aufnahme laeuft](docs/assets/state_1_recording.png)

### 3. Fertig & Kopiert (Ready)
`whisper-rs` (`whisper.cpp`) transkribiert die Audiospur im Hintergrund-Thread und legt das Ergebnis direkt in die System-Zwischenablage. Bietet Schnellaktionen zum Anhängen weiterer Sätze (`[+ Weiter]`), für eine Neuaufnahme (`[● Neu]`) oder zum Öffnen des Editors (`[↗ Details]`) und blendet sich nach 8 Sekunden Inaktivität automatisch aus:

![In Zwischenablage kopiert](docs/assets/state_2_ready.png)

### 4. Ausgeklappt (Review, Edit & Beenden)
Über `[↗ Details]` öffnet sich der mehrzeilige, scrollbare Texteditor (`520 × 240 px`) mit den Aktions-Buttons `[▲ Zuklappen]`, `[– Verbergen]`, `[✕ Beenden]` und `[⎘ Text kopieren]`:

![Ausgeklappter Editor](docs/assets/state_3_expanded.png)

### 5. Integrierter Einstellungs-Dialog (Settings)
Über das System-Tray-Menü öffnet sich die Einstellungsansicht (`580 × 480 px`) zur Wahl des Sprachmodells (`Base` vs. `Small` inkl. Direkt-Download), der Suchstrategie (`Beam Search`), der Mikrofon-Normalisierung (`Gain`) und des System-Autostarts:

![Einstellungen](docs/assets/state_4_settings.png)

### 6. Grafischer Installations-Assistent (Setup-Wizard für Windows & Linux)
Mit `--installer` (oder als eigenständige `whisper_pill_installer` / `Whisper_Pill_Setup_Wizard.exe`) öffnet sich der 4-stufige Setup-Wizard (`640 × 480 px`) zur Einrichtung von Zielverzeichnis, Desktop-/Startmenü-Verknüpfungen, Autostart und Uninstaller:

![Installations-Assistent](docs/assets/state_5_installer.png)

---

## Highlights

* **100 % Offline & Datenschutz:** Die Spracherkennung läuft vollständig lokal auf deiner CPU via `whisper-rs` (`whisper.cpp`). Keine Cloud-Verbindung, keine API-Kosten, keine Datenübertragung.
* **Native Rust-Performance:** Minimaler Speicherverbrauch, rasanter Start und entkoppelte Thread-Architektur für vollkommen ruckelfreie UI-Reaktionen.
* **Autarkes Linux-AppImage:** Ein einzelnes, portables Bundle von nur **ca. 13 MB** – lauffähig auf nahezu jeder modernen Linux-Distribution ohne manuelle Abhängigkeitsinstallation.
* **Integrierter Einstellungsdialog:** Modellwechsel (`base` vs. `small`) inkl. Direkt-Download, Wahl der Suchstrategie (Beam Search vs. Greedy), Mikrofon-Gain und Autostart direkt über das Tray-Menü konfigurierbar.
* **System-Tray-Integration:** Läuft unaufdringlich im Hintergrund; das Tray-Icon signalisiert aktiv laufende Transkriptionen in Echtzeit mit pulsierender Wellenform (`ılı`).
* **Wayland & X11 Unterstützung:** Native Integration unter KDE Plasma 6 (inklusive automatischer KWin-D-Bus-Regeln zur Vermeidung von Taskleisten-Einträgen), GNOME und XFCE.
* **MIT-Lizenz:** Maximal permissiv für private und kommerzielle Nutzung.

---

## Schnellstart

### 1. Als autarkes AppImage ausführen
Whisper Pill benötigt keine Installation und kann direkt als AppImage gestartet werden:

```bash
# AppImage starten
./dist/Whisper_Pill-x86_64.AppImage

# Versionsprüfung im Terminal
./dist/Whisper_Pill-x86_64.AppImage --version

# Hilfe & CLI-Optionen anzeigen
./dist/Whisper_Pill-x86_64.AppImage --help
```

### 2. KI-Sprachmodell einrichten
Das benötigte GGML-Modell (`base` oder `small`) lässt sich direkt in der App über **Tray-Icon → Einstellungen...** auswählen oder vorab per Skript herunterladen:

```bash
# Standard-Modell (base, ca. 140 MB) herunterladen
./scripts/download_model.sh base

# Oder das präzisere Modell (small, ca. 460 MB) herunterladen
./scripts/download_model.sh small
```

---

## Tastenkombinationen & Bedienung

| Funktion | Tastenkombination / Geste | Beschreibung |
|:---|:---|:---|
| **Aufnahme starten / stoppen** | <kbd>Super</kbd> + <kbd>Strg</kbd> + <kbd>P</kbd> | Globaler Hotkey zum Starten und Beenden der Sprachaufnahme. |
| **Text einfügen** | <kbd>Strg</kbd> + <kbd>V</kbd> | Fügt den transkribierten Text in das aktive Fenster ein. |
| **Pill verschieben** | Linke Maustaste gedrückt halten | Freies Drag & Drop; die Position wird dauerhaft gespeichert. |
| **Kontextmenü öffnen** | Rechtsklick auf Tray-Icon | Zugriff auf Aufnahme, Einstellungen, Fehlerlog und Beenden. |
| **Pill verbergen** | Klick auf `[ – Verbergen ]` / `[ ✕ ]` oder Auto-Dismiss (8 s) | Minimiert die Kapsel zurück in den System-Tray. |
| **Anwendung beenden** | Klick auf `[ ✕ Beenden ]` im Editor oder via Tray-Menü | Beendet Whisper Pill vollständig. |

---

## Weiterführende Dokumentation & Lizenz

* **[Endbenutzer-Handbuch (Markdown)](docs/USER_GUIDE.md):** Umfassender Leitfaden für Einrichtung, Bedienung und Praxistipps.
* **[Endbenutzer-Handbuch (Autarkes HTML)](docs/user_guide.html):** Grafisch aufbereitetes Offline-Handbuch für den Browser.
* **[Technische Entwickler- & Architektur-Dokumentation](docs/architecture/technical_documentation.md):** Klassenmodelle, Mermaid-UML-Diagramme, Threading-Architektur und Datenflüsse.
* **[Projekt-Journal](docs/JOURNAL.md):** Chronologische Dokumentation aller Architektur- und Entwicklungsentscheidungen.
* **Lizenz:** Veröffentlicht unter der **[MIT-Lizenz](LICENSE)**.
