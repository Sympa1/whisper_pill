---
name: security-audit
description: Führt einen vollständigen Sicherheits-, Schwachstellen- und Secret-Scan durch. Prüft Projekt-Code und Abhängigkeiten auf bekannte CVEs, statische Schwachstellen (SAST), hardcodierte Zugangsdaten sowie generelle Netzwerk- und Download-Aktivitäten (Zero-Download & Strict Offline Check).
---

# Comprehensive Security & Secret Audit Workflow

Führe bei Aktivierung dieses Skills folgende Schritte strikt und offline durch:

## 1. Secrets-, Token- & Credential-Scan (Hardcoded Secrets)
Scanne das gesamte Projektverzeichnis (`prototype/`, `app/`, `tests/`, Konfigurationsbeispiele) auf versehentlich fest hinterlegte sensible Zugangsdaten:
* **Musterbasierte Erkennung (Regex & Entropie):**
  * **API-Keys & Tokens:** OpenAI (`sk-...`), Anthropic, HuggingFace (`hf_...`), GitHub (`ghp_...`, `gho_...`), Generic Bearer Tokens.
  * **Login-Daten:** Variablen mit Zuweisungen wie `password = "..."`, `passwd = "..."`, `secret = "..."`, `api_key = "..."`.
  * **Zertifikate & Private Keys:** Header wie `-----BEGIN PRIVATE KEY-----` oder `-----BEGIN RSA PRIVATE KEY-----`.
* **Entropie-Check:** Suche nach langen Hexadezimal- oder Base64-Strings in Zuweisungen, die auf Passwörter oder kryptografische Schlüssel hinweisen.
* **Ausschlusskriterium:** Konfigurationen dürfen Credentials (falls je benötigt) nur aus Umgebungsvariablen oder sicheren OS-Keystores beziehen, niemals im Quellcode.

## 2. CVE- & Vulnerability-Check (Bekannte Lücken)
* Überprüfe die installierten Paketversionen in `.venv/` und `app/Cargo.lock` auf gelistete Sicherheitslücken:
  * Führe einen Abgleich via `pip-audit` / `cargo audit` oder lokaler Advisory-Datenbank durch.
  * Identifiziere bekannte Exploits (z. B. Pufferüberläufe in C-Extensions von NumPy/SciPy oder Schwachstellen in älteren PySide6-Versionen).

## 3. Statische Code-Analyse (SAST & Typische Schwachstellen)
Scanne `prototype/`, `app/` und `tests/` auf gängige Implementierungsrisiken:
* **Injections & Execution:** Keine Nutzung von `eval()`, `exec()` oder `subprocess.Popen(..., shell=True)`.
* **Unsichere Deserialisierung:** Ausschließlich `json` verwenden; `pickle` oder unsicheres YAML-Laden ist strikt verboten.
* **Dateisystem & Race Conditions:** Temporäre Audio-Dateien dürfen nicht mit vorhersehbaren Namen in `/tmp` geschrieben werden (`tempfile`-Modul mit sicheren Flags erzwingen).
* **Keyboard-/Paste-Injection:** Überprüfe, ob Eingaben über `pyautogui` vor der Ausführung gegen Steuerbefehle validiert werden.

## 4. Generelle Netzwerk- & Download-Prüfung (Zero-Download & Strict Offline Check)
Als rein lokales Desktop-Utility gilt der Grundsatz: **Es dürfen zur Laufzeit keine unerlaubten Daten, Modelle oder Binaries aus dem Internet heruntergeladen werden.**
* **Automatisierter Scan-Befehl:**
  ```bash
  python3 .agent/skills/security-audit/scripts/check_downloads.py
  ```
* **Quellcode-Scan auf Download-Mechanismen (`app/`, `prototype/`):**
  * Suche nach CLI-Downloadern oder Subprozess-Aufrufen: `curl`, `wget`, `aria2`, `ftp`.
  * Suche nach Netzwerk-Clients & HTTP-Bibliotheken: Python (`urllib`, `requests`, `aiohttp`, `httpx`), Rust (`reqwest`, `hyper`, `ureq`, `std::net::TcpStream`, `tokio::net::TcpStream` – mit Ausnahme lokaler IPC/Unix-Sockets wie D-Bus).
  * Suche nach versteckten Update-Checks, Auto-Update-Routinen oder dynamischem Nachladen von Skripten/Binärdateien.
* **Tiefenprüfung von Drittanbieter-Bibliotheken:**
  * **Rust (`Cargo.lock`):** Verifiziere, dass keine Networking-/HTTP-Crates gelistet sind und Build-Skripte (`build.rs`) beim Kompiliervorgang keine Netzwerkverbindungen aufbauen.
  * **Python (`site-packages`):** Durchsuche `faster-whisper`, `sounddevice`, `pyperclip` nach Sockets, HTTP-Clients, Analytics oder verdeckter Telemetrie.
  * **KI-Modell-Downloads:** Prüfe explizit, ob KI-Frameworks beim Initialisieren ungefragt Verbindung zu externen Servern (z. B. HuggingFace) aufbauen. In Python muss `local_files_only=True` erzwungen sein; in Rust muss `whisper-rs` rein lokale Modellpfade verwenden.
* **RAM & Audio-Flushing:** Sicherstellen, dass Audio-Arrays nach Transkriptionsende überschrieben und freigegeben werden.

## 5. Audit-Bericht (`docs/security_audit_report.md`)
Erstelle oder überschreibe nach jedem Scan zwingend die Datei `docs/security_audit_report.md` nach folgendem Standard.

> **Wichtig (Datenschutz & Public-Repo-Konformität):** In den Berichten dürfen **keine Dateipfade** (weder absolute Systempfade noch interne Quellcodedateipfade) aufgeführt werden, da die Berichte öffentlich einsehbar sind. Verwende stattdessen ausschließlich sprechende Modul- und Komponentennamen (z. B. `AudioRecorder`, `TranscriptionEngine`, `HotkeyPlatform`, `UI`).

```markdown
# Sicherheits-Auditbericht: whisper-pill

- **Datum & Uhrzeit:** [YYYY-MM-DD HH:MM:SS]
- **Gesamtbewertung:** [SECURE / ACTION REQUIRED]
- **Geprüfter Scope:** [Komponentenübersicht ohne Dateipfade]

## 1. Secrets & Zugangsdaten (Hardcoded Credentials)
| Komponente | Gefundener Typ | Gefundener Bezeichner | Schweregrad | Status |
| :--- | :--- | :--- | :--- | :--- |
| Quellcode & UI-Komponenten | Keine hardcodierten Keys/Tokens gefunden | - | Info | PASSED |

## 2. Gefundene Schwachstellen & CVEs
| Paket / Komponente | CVE-ID / Schwachstelle | Schweregrad | Status | Empfehlung |
| :--- | :--- | :--- | :--- | :--- |
| - | Keine bekannten CVEs gefunden | - | OK | - |

## 3. Statische Code-Sicherheit (SAST)
- **Injection-Prüfung:** [PASSED / FAILED]
- **Sichere Dateihandhabung:** [PASSED / FAILED]
- **Clipboard- & Input-Validierung:** [PASSED / FAILED]

## 4. Privacy, Download-Verbot & Offline-Verifikation
- **Generelle Download-Prüfung:** [0 Downloads / Unerlaubte Downloads gefunden]
- **Netzwerk-Aufrufe & Sockets:** [0 gefunden (100 % lokal) / Befund]
- **Modell-Isolation:** [100 % strikt offline]
- **Log-Hygiene:** Keine Klartext-Diktate in Logs
- **Lizenz-Konformität:** Alle Pakete permissiv (MIT / BSD / LGPLv3)
```
