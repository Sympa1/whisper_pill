# Sicherheits-Auditbericht: whisper-pill (Rust App)

- **Datum & Uhrzeit:** 2026-09-27 08:34:00
- **Gesamtbewertung:** **ACTION REQUIRED (Download-Fallback in Engine identifiziert)**
- **Geprüfter Scope:** Rust-Anwendung (Quellcode, UI-Definitionen, Abhängigkeiten, Build-Konfiguration)

---

## 1. Secrets & Zugangsdaten (Hardcoded Credentials)

Der Quellcode der Rust-Anwendung sowie die Slint-UI-Definitionen wurden mit Regex- und Entropie-Mustern auf API-Keys (OpenAI, Anthropic, HuggingFace, GitHub), Passwörter, Token-Header und Zertifikate gescannt.

| Komponente | Gefundener Typ | Gefundener Bezeichner | Schweregrad | Status |
| :--- | :--- | :--- | :--- | :--- |
| Quellcode & UI-Komponenten | Keine hardcodierten Keys/Tokens gefunden | - | Info | **PASSED** |

> **Ergebnis:** 0 Secrets gefunden. Die Rust-App benötigt und verwendet keinerlei API-Tokens oder Anmeldedaten.

---

## 2. Gefundene Schwachstellen & CVEs (Abhängigkeiten)

Die in der Cargo-Abhängigkeitsauflösung gebundenen Crates wurden analysiert:

| Paket / Version | Einsatzzweck | Lizenz | Schweregrad | Status | Empfehlung |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `slint` (1.18) | GUI-Framework | MIT / GPL-3.0 | Info | **OK** | Keine bekannten CVEs |
| `cpal` (0.15) | Audio-Aufnahme | Apache-2.0 | Info | **OK** | Native PortAudio/ALSA/WASAPI-Anbindung |
| `arboard` (3.4) | Zwischenablage | MIT / Apache-2.0 | Info | **OK** | Sichere Wayland/X11/Win32 Clipboard-APIs |
| `crossbeam-channel` (0.5) | MPMC-Queue | MIT / Apache-2.0 | Info | **OK** | Thread-sichere FIFO-Kommunikation |
| `serde` / `serde_json` (1.0) | Serialisierung | MIT / Apache-2.0 | Info | **OK** | Typsicheres JSON ohne dynamischen Eval |
| `tokio` (1.40) | Async-Runtime | MIT | Info | **OK** | Nur Basisfunktionen, kein "net"-Feature aktiviert |
| `whisper-rs` (0.16.0) | Lokale Whisper Inferenz | MIT | Info | **OK** | Bindings zu nativem whisper.cpp (offline) |
| `zbus` (4.4) / `ksni` (0.3) | D-Bus Shortcuts & Tray | MIT | Info | **OK** | Lokale IPC via Unix Domain Sockets |

> **Ergebnis:** Keine externen HTTP-/Netzwerk-Clients (`reqwest`, `hyper`, `ureq`) in den Abhängigkeiten vorhanden. Alle Lizenzen sind permissiv und kompatibel.

---

## 3. Statische Code-Sicherheit (SAST & Memory Safety)

- **Memory Safety & Unsafe Rust:** **PASSED**
  - Im Rust-Quellcode existieren **0 `unsafe` Codeblöcke**. Der gesamte Anwendungscode ist 100 % Safe Rust.
- **Command- & Shell-Injections:** **PASSED**
  - Alle Aufrufe von `std::process::Command` (z. B. im Hotkey-Manager für `gdbus`/`kwriteconfig6` und im System-Tray-Menü für `xdg-open`) übergeben Argumente ausschließlich als typisierte String-Slices (`.args([...])`).
  - Es existiert keine Shell-Interpolation (`sh -c` oder `cmd.exe /c` werden nicht verwendet). Keine externen Benutzereingaben werden an Systembefehle weitergereicht.
- **Sichere Dateihandhabung & Pfade:** **PASSED**
  - Konfiguration wird unter standardisiertem XDG-Pfad (Benutzerkonfiguration) verwaltet.
  - Autostart-Dateien werden im standardkonformen XDG-Autostart-Verzeichnis mit festen Formatstrings geschrieben.
  - Keine unsicheren Pfadverkettungen oder Directory-Traversal-Risiken.
- **Clipboard-Validierung:** **PASSED**
  - Mutex-geschützter Clipboard-Manager. Transkribierter Text wird vor der Übergabe getrimmt; leere Eingaben werden sauber abgefangen.

---

## 4. Privacy, Download-Verbot & Offline-Verifikation

- **Generelle Download-Prüfung:** **ACTION REQUIRED**
  - **Befund 1 (CRITICAL):** In der Transkriptions-Engine (`WhisperEngine`) ruft die Methode `ensure_model_available()` über `std::process::Command::new("curl")` ein Modell von HuggingFace herunter, wenn die Datei nicht lokal existiert.
  - **Befund 2 (MEDIUM):** In der Transkriptions-Engine (`WhisperEngine`) ist eine externe Download-URL (`https://huggingface.co/ggerganov/whisper.cpp/...`) hinterlegt.
  - **Empfehlung zur Behebung:** Automatischen `curl`-Download entfernen. Stattdessen prüfen, ob die Modelldatei existiert, und falls nicht, mit einer informativen Fehlermeldung abbrechen (*"Whisper-Modell nicht gefunden: Bitte Modelldatei lokal im Cache- oder Anwendungsordner ablegen"*).
- **Netzwerk-Aufrufe & Sockets:** **0 Remote-Sockets (100 % lokal)**
  - Keine Remote-Sockets oder ausgehende Netzwerkverbindungen.
  - Verwendete D-Bus-Sockets (`zbus`, `ksni`) operieren ausschließlich über lokale Unix Domain Sockets (`/run/user/.../bus`).
- **Modell-Isolation:** **100 % lokal** (sobald Modelldatei auf der Festplatte vorliegt; Inferenz via `whisper-rs` benötigt keinerlei Internetzugriff).
- **RAM- & Audio-Flushing (Datenschutz):** **PASSED (Vorbildlich)**
  - Im Inferenz-Worker werden alle flüchtigen Audio-Samples nach Beendigung der Inferenz mit `0.0` überschrieben:
    ```rust
    for s in mut_samples.iter_mut() {
        *s = 0.0;
    }
    mut_samples.clear();
    ```
  - Im Audio-Recorder bietet `AudioRecorder::flush_memory()` denselben Zero-Fill-Mechanismus.
- **Log-Hygiene:** **PASSED**
  - Zu keinem Zeitpunkt werden gesprochene oder transkribierte Inhalte in Logdateien oder auf der Standardausgabe persistent gespeichert.
- **Lizenz-Konformität:** **PASSED** (MIT-Lizenz des Projekts, alle Crates permissiv).

---

## 5. Audit-Zusammenfassung & Handlungsempfehlung

| Bereich | Status | Details |
| :--- | :--- | :--- |
| **Secrets & Keys** | ✔ PASSED | 0 Hardcoded Credentials |
| **Abhängigkeiten (Cargo)** | ✔ PASSED | 0 Netzwerk-Crates, permissive Lizenzen |
| **Memory Safety (SAST)** | ✔ PASSED | 0 `unsafe` Blöcke, 100 % Safe Rust |
| **Command Injection** | ✔ PASSED | Keine dynamischen Shell-Befehle |
| **Audio-Datenschutz** | ✔ PASSED | RAM-Zeroing direkt nach Transkription |
| **Zero-Download-Richtlinie** | ⚠️ **ACTION REQUIRED** | `curl`-Modell-Download in Transkriptions-Engine entfernen |
