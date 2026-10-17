# Sicherheits-Auditbericht: Whisper Pill (Rust-App & AppImage)

- **Datum & Uhrzeit:** 2026-10-17 11:45:00
- **Gesamtbewertung:** **SECURE (Alle Sicherheits-, Datenschutz- und Offline-Prüfungen bestanden)**
- **Geprüfter Scope:** Native Rust-Anwendung (`AppOrchestrator`, `AudioRecorder`, `WhisperEngine`, `ModelDownloader`, `HotkeyManager`, `KWinPlatform`, `AutostartPlatform`, `PillTray`, `ClipboardManager`, `AppConfig`, `Logger`), Slint-UI, AppImage-Packaging & Desktop-Integration (KDE Plasma / XFCE)

---

## 1. Secrets & Zugangsdaten (Hardcoded Credentials)

Der gesamte Quellcode der Rust-Anwendung, die Slint-UI-Definitionen, Packaging-Skripte sowie die Konfigurations- und Testmodule wurden mittels Regex- und Entropie-Scans auf API-Schlüssel (OpenAI, Anthropic, HuggingFace, GitHub, AWS), Passwörter, Tokens und private Zertifikate geprüft.

| Komponente | Prüfumfang | Gefundene Secrets | Schweregrad | Status |
| :--- | :--- | :--- | :--- | :--- |
| `AppOrchestrator` & `Slint UI` | Quellcode, Callbacks, Properties | 0 | Info | **PASSED** |
| `WhisperEngine` & `ModelDownloader` | Inferenz-Pipeline & Modell-Cache | 0 | Info | **PASSED** |
| `Platform` (KDE / XFCE / Autostart) | D-Bus, X11, XDG-Desktop-Entries | 0 | Info | **PASSED** |
| `BuildScript` & `Packaging` | AppImage-Builder, `AppRun`, `.desktop` | 0 | Info | **PASSED** |

> **Ergebnis:** 0 Secrets gefunden. Die Anwendung arbeitet vollständig lokal ohne Cloud-Accounts, Telemetrie-Endpunkte oder API-Schlüssel.

---

## 2. Abhängigkeiten & Supply-Chain-Sicherheit (Cargo Crates)

Die in `Cargo.toml` und `Cargo.lock` verankerten Abhängigkeiten der Rust-Anwendung wurden auf bekannte Sicherheitslücken (CVEs), unnötige Netzwerk-Stacks und Lizenzkompatibilität geprüft:

| Crate / Version | Einsatzzweck | Lizenz | Netzwerk-Stack | Status |
| :--- | :--- | :--- | :--- | :--- |
| `slint` (1.18) | Deklaratives GUI-Framework (FemtoVG / Software) | MIT / GPL-3.0 | Nein | **OK** |
| `whisper-rs` (0.16.0) | Lokale C++ `whisper.cpp` CPU-Inferenz | MIT | Nein | **OK** |
| `cpal` (0.15) | Low-Latency Audio-Aufnahme (ALSA / PipeWire / PulseAudio) | Apache-2.0 | Nein | **OK** |
| `arboard` (3.4) | System-Zwischenablage (Wayland / X11 / Windows) | MIT / Apache-2.0 | Nein | **OK** |
| `crossbeam-channel` (0.5) | Lock-freie MPMC-Thread-Kanäle | MIT / Apache-2.0 | Nein | **OK** |
| `serde` / `serde_json` (1.0) | Typsichere JSON-Konfiguration | MIT / Apache-2.0 | Nein | **OK** |
| `tokio` (1.40) | Async-Runtime (`rt-multi-thread`, `sync`, `time`, `macros`) | MIT | Nein (`net` deaktiviert) | **OK** |
| `ksni` (0.3) | Freedesktop StatusNotifierItem System-Tray (D-Bus) | MIT | Nur lokaler Unix-Socket | **OK** |
| `zbus` (4.4) | Lokale D-Bus-IPC (KDE `KGlobalAccel`) | MIT | Nur lokaler Unix-Socket | **OK** |

> **Ergebnis:** In der gesamten Cargo-Abhängigkeitsstruktur ist **kein einziger HTTP- oder TCP/IP-Client** (`reqwest`, `hyper`, `ureq`, `curl-sys`) eingebunden. Selbst in `tokio` ist das `net`-Feature explizit deaktiviert.

---

## 3. Statische Code-Analyse (SAST), Memory Safety & Prozess-Sicherheit

### 3.1 Memory Safety & `unsafe`-Prüfung
- **0 `unsafe`-Blöcke im Rust-Anwendungscode:** Der gesamte eigene Rust-Code (`app/src/`) kommt vollständig ohne `unsafe`-Blöcke aus.
- **Compiler- & Linter-Verifikation:** `cargo clippy --all-targets -- -D warnings` läuft mit **0 Warnungen und 0 Fehlern** durch.
- **Kein externes PNG-Crate für Screenshots nötig:** Der integrierte Screenshot-Generator (`--capture-screenshots`) nutzt einen kompakten, speichersicheren Pure-Rust-Encoder auf Basis der Standardbibliothek (`std::fs`, CRC32/Adler32).

### 3.2 Schutz vor Command- & Shell-Injection (`std::process::Command`)
Sämtliche externen Prozessaufrufe wurden einzeln auditiert:
1. **`HotkeyManager` (KDE `KGlobalAccel`):** Ruft `gdbus` und `kwriteconfig6`/`kwriteconfig5` mit statischen, fest eincompilierten Argument-Arrays (`.args([...])`) auf. Keine Benutzereingaben fließen in den Aufruf ein.
2. **`KWinPlatform` (KDE Window Rules & X11/XFCE `xprop`):** Liest und setzt Fensterattribute über `kreadconfig6`/`kwriteconfig6`, `qdbus6`/`qdbus` sowie `xprop` (für `_NET_WM_STATE_SKIP_TASKBAR` unter XFCE/X11) ausschließlich ohne Shell-Interpolation (`sh -c` wird nirgendwo verwendet).
3. **`ModelDownloader` (Expliziter Modell-Download auf Nutzerwunsch):** Übergibt nur validierte, feste Modellnamen (`"base"` oder `"small"`) an `curl`/`wget` über `.args([...])` und schreibt zunächst atomar in eine `.tmp`-Datei, bevor diese nach erfolgreicher Prüfung umbenannt wird.
4. **`PillTray` / `AppOrchestrator` (Log-Anzeige):** Öffnet die lokale Logdatei über `xdg-open` als separates Prozess-Argument ohne Shell-Auswertung.

---

## 4. Desktop- & Plattform-Sicherheit (KDE Plasma, XFCE & AppImage)

### 4.1 D-Bus-Signal-Isolierung unter KDE Plasma (`HotkeyManager`)
- **Strikte Objektpfad- und Komponenten-Filterung:** Der D-Bus-Listener abonniert über `zbus::MatchRule` ausschließlich Signale vom Objektpfad `/component/whisper_pill` auf dem Interface `org.kde.kglobalaccel.Component` (`globalShortcutPressed`).
- **Payload-Verifikation (`is_whisper_pill_shortcut`):** Jedes eingehende Signal wird zusätzlich auf seine Tupel-Parameter `(component == "whisper_pill", action == "toggle_pill")` geprüft. Dadurch ist ausgeschlossen, dass fremde Desktop-Shortcuts (wie <kbd>F12</kbd> für Yakuake, die <kbd>Super</kbd>-Taste für das Anwendungsmenü oder Medien-Tasten) unbeabsichtigt eine Mikrofonaufnahme starten.

### 4.2 Fenstermanager-Isolation unter KDE (KWin) und XFCE (Xfwm4 / X11)
- **Kein Forced-Keep-Above-Hijacking:** In den KWin-Regeln (`KWinPlatform`) wird `aboverule=0` (`above=false`) gesetzt, damit die Kapsel keine sicherheits- oder systemkritischen Overlays (Dropdown-Terminals wie Yakuake, Bildschirmsperren oder Systemdialoge) verdeckt.
- **Auto-Dismiss im Ready-Zustand:** Nach abgeschlossener Transkription blendet sich die Kapsel nach 8 Sekunden Inaktivität automatisch aus.
- **XFCE / X11-Kompatibilität:** Über EWMH-Attribute (`_NET_WM_STATE_SKIP_TASKBAR`, `_NET_WM_STATE_SKIP_PAGER`) und das StatusNotifierItem-Protokoll fügt sich die Anwendung sauber in XFCE-Panels ein, ohne die Fensterliste zu überfrachten.

### 4.3 AppImage- & Autostart-Sicherheit (`AutostartPlatform`, `BuildScript`)
- **Persistente `$APPIMAGE`-Pfadauflösung:** Beim Betrieb als portables AppImage wird die Anwendung zur Laufzeit unter einem temporären FUSE-Mount (`/tmp/.mount_*`) eingehängt. `AutostartPlatform::get_persistent_executable_path()` wertet gezielt die Umgebungsvariable `$APPIMAGE` aus, sodass in der `.desktop`-Autostart-Datei niemals flüchtige `/tmp`-Pfade hinterlegt werden (Schutz vor kaputten Autostart-Einträgen und `/tmp`-Hijacking nach Neustarts).
- **Selbstheilende Pfadvalidierung:** Beim Start prüft die App, ob der in `whisper-pill.desktop` eingetragene `Exec=`-Pfad noch auf eine real existierende Datei verweist, und korrigiert veraltete Pfade automatisch.
- **Restriktive Dateiberechtigungen (`0o755`):** Ausführbare Rechte werden ausschließlich für die eigene `.desktop`-Datei im Benutzer-Autostart-Ordner und die Ziel-AppImage-Datei gesetzt.
- **Public-Repo-Hygiene im Build-Skript (`scripts/build_appimage.sh`):** Keine lokalen Entwicklerpfade im Build-Skript; Suche nach `appimagetool` erfolgt ausschließlich über `$APPIMAGETOOL`, `$PATH` und Standard-Benutzerpfade.

---

## 5. Datenschutz, Zero-Download-Prinzip & RAM-Hygiene

- **Automatisierter Offline-Check (`check_downloads.py`):** **PASSED**
  - `WhisperEngine::ensure_model_available()` führt **keinerlei automatische Netzwerk-Downloads** durch, sondern prüft rein lokal das Vorhandensein der Modelldatei.
  - Ein Download von HuggingFace erfolgt ausschließlich dann, wenn der Benutzer im Einstellungs-Dialog aktiv auf den Modell-Download klickt oder das Skript `scripts/download_model.sh` manuell aufruft.
- **RAM-Zeroing von Sprachdaten (Audio-Flushing):** **PASSED (Vorbildlich)**
  - Unmittelbar nach Abschluss jeder Whisper-Transkription werden sämtliche Audiosamples im Arbeitsspeicher explizit mit `0.0` überschrieben (`*s = 0.0; mut_samples.clear();`), bevor der Puffer freigegeben wird.
  - Ebenso überschreibt `AudioRecorder::flush_memory()` den Aufnahmepuffer vor dem Leeren vollständig mit Nullen.
- **Strikte Log-Hygiene (`Logger`):** **PASSED**
  - In die Diagnose-Datei (`error.log`) werden ausschließlich technische System- und Fehlermeldungen (Zeitstempel, Modulname, Fehlerursache) geschrieben. Transkribierte Texte oder Audioinhalte werden niemals protokolliert.

---

## 6. Zusammenfassung der Prüfergebnisse

| Prüfkategorie | Status | Ergebnisdetails |
| :--- | :--- | :--- |
| **Secrets & Credentials** | ✔ PASSED | 0 hardcodierte Schlüssel, Tokens oder Passwörter |
| **Cargo-Abhängigkeiten** | ✔ PASSED | 0 HTTP-/Netzwerk-Crates, alle Lizenzen MIT/Apache-2.0-kompatibel |
| **Memory Safety (SAST)** | ✔ PASSED | 0 `unsafe`-Blöcke im Anwendungscode, 0 Clippy-Warnungen |
| **Command Injection** | ✔ PASSED | Typsichere Argument-Arrays, keine Shell-Interpolation (`sh -c`) |
| **D-Bus & Hotkey-Isolation** | ✔ PASSED | Strikter Pfad- (`/component/whisper_pill`) & Payload-Filter |
| **AppImage & Autostart** | ✔ PASSED | Persistente `$APPIMAGE`-Auflösung, kein `/tmp`-Mount-Leaking |
| **Zero-Download & Offline-Check** | ✔ PASSED | `check_downloads.py` meldet 0 unerlaubte Downloads im Betrieb |
| **Audio-RAM- & Log-Datenschutz** | ✔ PASSED | Explizites Zero-Fill der Audio-Samples nach jeder Transkription |
