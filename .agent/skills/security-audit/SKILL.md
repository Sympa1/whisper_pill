---
name: security-audit
description: Führt einen vollständigen Sicherheits-, Schwachstellen- und Secret-Scan durch. Prüft Projekt-Code und Abhängigkeiten auf bekannte CVEs, statische Schwachstellen (SAST), hardcodierte Zugangsdaten (API-Keys/Tokens) sowie Telemetrie und Datenabfluss.
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
* Überprüfe die installierten Paketversionen in `.venv/` auf gelistete Sicherheitslücken:
  * Führe einen Abgleich via `pip-audit` oder lokaler Advisory-Datenbank durch.
  * Identifiziere bekannte Exploits (z. B. Pufferüberläufe in C-Extensions von NumPy/SciPy oder Schwachstellen in älteren PySide6-Versionen).

## 3. Statische Code-Analyse (SAST & Typische Schwachstellen)
Scanne `prototype/` und `tests/` auf gängige Implementierungsrisiken:
* **Injections & Execution:** Keine Nutzung von `eval()`, `exec()` oder `subprocess.Popen(..., shell=True)`.
* **Unsichere Deserialisierung:** Ausschließlich `json` verwenden; `pickle` oder unsicheres YAML-Laden ist strikt verboten.
* **Dateisystem & Race Conditions:** Temporäre Audio-Dateien dürfen nicht mit vorhersehbaren Namen in `/tmp` geschrieben werden (`tempfile`-Modul mit sicheren Flags erzwingen).
* **Keyboard-/Paste-Injection:** Überprüfe, ob Eingaben über `pyautogui` vor der Ausführung gegen Steuerbefehle validiert werden.

## 4. Privacy- & Quellcode-Tiefenprüfung (Drittanbieter-Libs)
Inspiziere den installierten Quellcode in `.venv/.../site-packages/`:
* **Netzwerk & Telemetrie:** Durchsuche `faster-whisper`, `sounddevice`, `pyperclip` nach Sockets, HTTP-Clients, Analytics oder verschleierten Payloads.
* **Offline-Garantie:** Verifiziere, dass HuggingFace- und Whisper-Modelle strikt offline geladen werden (`local_files_only=True`).
* **RAM & Audio-Flushing:** Sicherstellen, dass Audio-Arrays nach Transkriptionsende überschrieben und freigegeben werden.

## 5. Audit-Bericht (`docs/security_audit_report.md`)
Erstelle oder überschreibe nach jedem Scan zwingend die Datei `docs/security_audit_report.md` nach folgendem Standard:

```markdown
# Sicherheits-Auditbericht: whisper-pill

- **Datum & Uhrzeit:** [YYYY-MM-DD HH:MM:SS]
- **Gesamtbewertung:** [SECURE / ACTION REQUIRED]

## 1. Secrets & Zugangsdaten (Hardcoded Credentials)
| Datei / Zeile | Gefundener Typ | Gefundener Bezeichner | Schweregrad | Status |
| :--- | :--- | :--- | :--- | :--- |
| - | Keine hardcodierten Keys/Tokens gefunden | - | Info | PASSED |

## 2. Gefundene Schwachstellen & CVEs
| Paket / Datei | CVE-ID / Schwachstelle | Schweregrad | Status | Empfehlung |
| :--- | :--- | :--- | :--- | :--- |
| - | Keine bekannten CVEs gefunden | - | OK | - |

## 3. Statische Code-Sicherheit (SAST)
- **Injection-Prüfung:** [PASSED / FAILED]
- **Sichere Dateihandhabung:** [PASSED / FAILED]
- **Clipboard- & Input-Validierung:** [PASSED / FAILED]

## 4. Privacy & Offline-Verifikation
- **Netzwerk-Aufrufe:** 0 gefunden (100 % lokal)
- **Log-Hygiene:** Keine Klartext-Diktate in Logs
- **Lizenz-Konformität:** Alle Pakete permissiv (MIT / BSD / LGPLv3)
