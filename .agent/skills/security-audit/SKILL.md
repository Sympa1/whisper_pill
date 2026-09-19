---
name: security-audit
description: Führt ein vollständiges lokales Sicherheits-Audit durch. Prüft Projekt-Code und inspiziert den Quellcode installierter externer Python-Libraries auf Datenabfluss, Telemetrie und Schwachstellen.
---

# Security & Privacy Audit Workflow

Führe bei Aktivierung dieses Skills folgende Schritte strikt und offline durch:

## 1. Audit des Projektcodes (`prototype/` und `app/`)
* **Telemetrie- & Netzwerk-Check:** Prüfe, ob irgendwo Socket-Verbindungen, HTTP-Requests (`urllib`, `requests`, `aiohttp`) oder unzulässige IPC-Aufrufe stattfinden.
* **Clipboard & Logging:** Stelle sicher, dass transkribierte Texte niemals unverschlüsselt in temporäre Log-Dateien oder Konsolenausgaben geschrieben werden.
* **Audio-Speicher:** Überprüfe, ob Audio-Puffer nach der Inferenz sofort im RAM überschrieben/freigegeben und temporäre WAV-Dateien restlos gelöscht werden.

## 2. Tiefenprüfung externer Abhängigkeiten (Library-Quellcode)
* **Lokalisierung:** Finde den physischen Installationspfad der Packages in der virtuellen Umgebung (`.venv/lib/python*/site-packages/`):
  * `faster-whisper`, `sounddevice`, `pyperclip`, `pyautogui`
* **Netzwerk-Calls in Drittcode suchen:**
  * Durchsuche den Quellcode der installierten Libs gezielt nach `socket`, `connect`, `http`, `urllib`, `telemetry`, `analytics` und Base64-verschleierten Payloads.
* **Modell-Download-Verhalten (`faster-whisper`):**
  * Prüfe, ob HuggingFace-Downloads nur einmalig initial erfolgen und danach ein strikter Offline-Modus erzwungen wird (`local_files_only=True`).
* **Lizenz-Verifikation:**
  * Überprüfe die `LICENSE`-Dateien in den installierten Site-Packages auf Übereinstimmung mit MIT / BSD-3-Clause / LGPLv3 (keine versteckte GPLv3).

## 3. Ergebnisbericht
Erstelle einen Audit-Report in `docs/security_audit_report.md` mit:
* Geprüften Paketversionen und Dateipfaden
* Gefundenen Risiken (Severity: Low/Medium/High/Critical)
* Bestätigung der 100 % lokalen Datenverarbeitung (Privacy Seal)
