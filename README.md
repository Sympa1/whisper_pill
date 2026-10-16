# Whisper Pill

> **Schlankes, minimalistisches, schwebendes Desktop-Diktier-Utility für Linux und Windows – 100 % lokal, quelloffen und datenschutzfokussiert.**

`Whisper Pill` ist eine freie Open-Source-Alternative zu kommerziellen Diktierwerkzeugen wie Whisperbar. Die Anwendung dockt als schlanke, schwebende Kapsel („Pill“) am Bildschirmrand an und wandelt gesprochene Sprache per Tastendruck in präzisen Text um – inklusive automatischer Satzzeichen- und Kommasetzung. Das Transkript wird direkt in die System-Zwischenablage gelegt und steht sofort mit <kbd>Strg</kbd> + <kbd>V</kbd> zum Einfügen bereit.

---

## Highlights

* **100 % Offline & Datenschutz:** Die Spracherkennung läuft vollständig lokal auf deiner CPU via `whisper-rs` (`whisper.cpp`). Keine Cloud-Verbindung, keine API-Kosten, keine Datenübertragung.
* **Native Rust-Performance:** Minimaler Speicherverbrauch, rasanter Start und entkoppelte Thread-Architektur für vollkommen ruckelfreie UI-Reaktionen.
* **Autarkes Linux-AppImage:** Ein einzelnes, portables Bundle von nur **ca. 13 MB** – lauffähig auf nahezu jeder modernen Linux-Distribution ohne manuelle Abhängigkeitsinstallation.
* **Modernes Slint-UI:** Elegante, dunkle Kapseloberfläche mit Live-Audio-Pegelanzeige, Aufnahme-Timer und ausklappbarem Editor für Diktat-Reviews.
* **System-Tray-Integration:** Läuft unaufdringlich im Hintergrund; das Tray-Icon signalisiert aktiv laufende Transkriptionen in Echtzeit mit pulsierender Wellenform (`ılı`).
* **Globale Tastenkürzel:** Starten und Stoppen von überall aus per <kbd>Super</kbd> + <kbd>Strg</kbd> + <kbd>P</kbd>.
* **Wayland & X11 Unterstützung:** Native Integration unter KDE Plasma 6 (inklusive automatischer KWin-D-Bus-Regeln zur Vermeidung von Taskleisten-Einträgen), GNOME und XFCE.
* **MIT-Lizenz:** Maximal permissiv für private und kommerzielle Nutzung.

---

## Schnellstart

### 1. Als autarkes AppImage ausführen
Whisper Pill muss nicht installiert werden und kann direkt als AppImage gestartet werden:

```bash
# AppImage ausführen
./dist/Whisper_Pill-x86_64.AppImage

# Versionsprüfung im Terminal
./dist/Whisper_Pill-x86_64.AppImage --version

# Hilfe & Optionen
./dist/Whisper_Pill-x86_64.AppImage --help
```

### 2. KI-Modelle einrichten
Whisper Pill unterstützt offizielle GGML-Modelle (`base` und `small`). Das Modell kann entweder über das Einstellungs-Menü oder über das mitgelieferte Skript geladen werden:

```bash
# Standard-Modell (base, ca. 140 MB) herunterladen
./scripts/download_model.sh base

# Oder das präzisere Modell (small, ca. 460 MB) herunterladen
./scripts/download_model.sh small
```

---

## Build- & Packaging-Anleitung

### Voraussetzungen
* **Rust-Toolchain:** Rust 1.85+ (Edition 2024) via [rustup](https://rustup.rs/)
* **Build-Tools:** `cmake`, ein C++-Compiler (`gcc` oder `clang`)
* **System-Bibliotheken:** ALSA (`libasound2-dev` bzw. `alsa-lib`), Fontconfig (`libfontconfig1-dev` bzw. `fontconfig`)
* **Für AppImage-Erstellung:** `appimagetool`

### Aus dem Quellcode ausführen & testen
```bash
# Debug-Build direkt ausführen
cargo run --manifest-path app/Cargo.toml

# Unit- & Integrationstests ausführen
cargo test --manifest-path app/Cargo.toml

# Statische Code-Analyse (Clippy)
cargo clippy --manifest-path app/Cargo.toml --all-targets
```

### Optimierte Release-Binärdatei kompilieren
```bash
cargo build --release --manifest-path app/Cargo.toml
# Die fertige Binärdatei liegt unter: app/target/release/whisper_pill
```

### Linux-AppImage erstellen
Mit dem mitgelieferten Build-Skript wird die Release-Binärdatei gebaut, alle Freedesktop-Metadaten und Icons gebündelt und das finale AppImage in `dist/` abgelegt:

```bash
./scripts/build_appimage.sh
# Erzeugt: dist/Whisper_Pill-x86_64.AppImage
```

---

## Tastenkombinationen & Bedienung

| Funktion | Tastenkombination / Geste | Beschreibung |
|:---|:---|:---|
| **Aufnahme starten / stoppen** | <kbd>Super</kbd> + <kbd>Strg</kbd> + <kbd>P</kbd> | Globaler Hotkey zum Starten/Beenden der Sprachaufnahme. |
| **Text einfügen** | <kbd>Strg</kbd> + <kbd>V</kbd> | Fügt den erkannten Text in das aktive Textfeld ein. |
| **Pill verschieben** | Linke Maustaste gedrückt halten | Freies Drag & Drop; Fensterkoordinaten werden persistent gespeichert. |
| **Kontextmenü öffnen** | Rechtsklick auf Tray-Icon | Zugriff auf Aufnahme, Einstellungen, Fehlerlog und Beenden. |
| **Pill verbergen** | Klick auf <kbd>✕</kbd> oder Klick außerhalb | Minimiert das Fenster zurück in den System-Tray. |

---

## Weiterführende Dokumentation

* **[Endbenutzer-Handbuch (Markdown)](docs/USER_GUIDE.md):** Umfassender Benutzerleitfaden mit allen Funktionen und Praxistipps.
* **[Endbenutzer-Handbuch (Autarkes HTML)](docs/user_guide.html):** Grafisch gestaltetes, 100 % offline im Webbrowser nutzbares Benutzerhandbuch.
* **[Technische Entwickler- & Architektur-Dokumentation](docs/architecture/technical_documentation.md):** Klassenmodelle, Mermaid UML-Diagramme, Threading-Modelle und Datenfluss-Spezifikationen.
* **[Projekt-Journal](docs/JOURNAL.md):** Lückenlose, chronologische Dokumentation aller Architektur- und Entwicklungsentscheidungen.
