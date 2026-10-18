---
name: security-audit
description: Umfassendes Security-, Supply-Chain-, Secret- & Zero-Download-Audit für den security-specialist Agenten. Prüft externe Libraries (CVEs & bösartiger Code), dokumentiert lückenlos JEDEN Download und JEDE Installation (Was, Woher, Wohin, Auslöser), scannt auf Hardcoded Secrets (Regex & Entropie), SQL-/Prompt-/Command-Injection sowie alle API-Angriffsvektoren und generiert docs/security_audit_report.md.
---

# SKILL: Comprehensive Security, Supply-Chain & Zero-Download Audit (`security-audit`)

> **Rolle & Einsatzzweck im 4-Agenten-Workflow:**
> Dieser Skill ist das verbindliche Prüfprotokoll für den **`security-specialist`-Agenten** (Read-Only, `enable_write_tools=False`).
> Er wird parallel zum `tester`-Agenten gestartet, sobald der `coder`-Agent seine Implementierung im Branch abgeschlossen hat.
> **Fail-Fast-Regel:** Meldet der parallel laufende `tester`-Agent einen Testfehler, wird dieser Security-Audit-Lauf **sofort abgebrochen (`kill`)** und erst nach erfolgreicher Fehlerbehebung durch den `coder` neu gestartet.

---

## 0. Automatisierter Vorab-Scan (`check_downloads.py`)

Führe zu Beginn jedes Audits das mitgelieferte statische Prüfskript aus (bzw. analysiere dessen Prüfkriterien im Read-Only-Modus):

```bash
python3 .agents/skills/security-audit/scripts/check_downloads.py
```

Zusätzlich (falls in der Projektumgebung installiert):
```bash
pip-audit -r requirements.txt
bandit -r src/ -ll
```

---

## 1. Externe Libraries, Downloads & Installationen (Supply-Chain & Zero-Download Check)

### 1.1 Prüfung externer Libraries auf Whitelist, CVEs & bösartigen Code
Überprüfe alle Abhängigkeits- und Lock-Dateien (`requirements.txt`, `pyproject.toml`, `setup.py`, `Cargo.lock`, `package.json`, `.venv/site-packages`):
1. **Whitelist-Abgleich (`AGENTS.md` Bibliotheks-Whitelist):**
   - Sind ausschließlich die in `AGENTS.md` freigegebenen Pakete eingebunden? Jedes nicht genehmigte Drittanbieter-Paket gilt als kritischer Verstoß.
2. **Bekannte Sicherheitslücken (CVEs):**
   - Abgleich der Paketversionen gegen bekannte Advisories (z. B. Pufferüberläufe in C-Extensions, Deserialisierungs-CVEs, veraltete Web-/UI-Frameworks).
3. **Unerwünschter oder bösartiger Code (Malicious Packages & Supply-Chain):**
   - **Typosquatting:** Prüfung auf täuschend ähnliche Paketnamen.
   - **Install-Hooks & Build-Skripte:** Prüfung von `setup.py` (`cmdclass`), `package.json` (`preinstall`/`postinstall`) und Rust `build.rs` auf versteckte Shell- oder Netzwerkaufrufe während der Installation/Kompilierung.
   - **Tiefenprüfung von `site-packages` & `Cargo.lock`:** Prüfe, ob eingebundene Libraries heimlich Netzwerk-Crates (`reqwest`, `hyper`, `ureq`, `curl-sys`) oder verdeckte Telemetrie/Analytics-Sockets öffnen.

### 1.2 Lückenlose Prüfung auf JEDEN Download und JEDE Installation
Durchsuche das **gesamte Projekt** (Quellcode, Skripte, Tests, `Dockerfile`, CI/CD-Workflows) systematisch nach **jedem einzelnen Vorgang**, bei dem Dateien, Pakete, KI-Modelle oder Binaries heruntergeladen oder installiert werden:
- **CLI-Downloader & Paketmanager:** `curl`, `wget`, `aria2`, `ftp`, `pip install`, `npm install`, `cargo install`, `apt-get install`, `git clone`.
- **Netzwerk-Clients & Sockets:** Python (`urllib`, `requests`, `aiohttp`, `httpx`, `socket`), Rust (`reqwest`, `hyper`, `ureq`, `std::net::TcpStream`, `tokio::net::TcpStream`).
- **KI-Modell-Downloads & Offline-Isolation:** Prüfe explizit, ob KI-Frameworks (`faster-whisper`, `transformers`, `huggingface_hub`, `torch.hub`) beim Initialisieren ungefragt Verbindung zu externen Servern (z. B. HuggingFace) aufbauen. Bei lokalen Anwendungen muss **`local_files_only=True`** erzwungen sein bzw. dürfen Downloads nur über ein explizit autorisiertes Downloader-Modul mit Hash-Prüfung erfolgen.

### 1.3 Pflicht-Dokumentation im Report (Was, Woher, Wohin, Auslöser)
Für **jeden einzelnen gefundenen Download- oder Installationsvorgang** MUSS im Audit-Bericht zwingend dokumentiert werden:
1. **WAS** wird heruntergeladen oder installiert? (Genauer Paket-, Datei-, Binary- oder Modellname inkl. Version).
2. **WOHER** wird geladen? (Exakte Quell-URL, Domain oder Paket-Registry + Angabe, ob **HTTPS**, **SHA256-Hash-Pinning** oder **Signaturprüfung** erfolgen).
3. **WOHIN** wird im System installiert/gespeichert? (Exakter Zielverzeichnis-Typ, z. B. lokales `.venv/site-packages`, App-Modellordner, Benutzer-Cache oder `/tmp`).
4. **AUSLÖSER** im Projekt: Welche Komponente / Methode löst den Vorgang aus (z. B. manuelle Initialisierung vs. automatischer Start beim Import)?

---

## 2. API-Keys, Login-Daten & Hardcoded Secrets (Regex & Entropie)

Scanne das gesamte Projektverzeichnis auf versehentlich fest hinterlegte Zugangsdaten:
- **Musterbasierte Erkennung (Regex):**
  - **API-Keys & Tokens:** OpenAI (`sk-...`), Anthropic, HuggingFace (`hf_...`), GitHub (`ghp_...`, `gho_...`), Google (`AIza...`), AWS (`AKIA...`), Generic Bearer Tokens, JWT-Secrets.
  - **Login-Daten & Passwörter:** Variablenzuweisungen wie `password = "..."`, `passwd = "..."`, `secret = "..."`, `api_key = "..."`, `access_token = "..."` sowie Datenbank-URLs mit eingebetteten Credentials.
  - **Zertifikate & Private Keys:** Header wie `-----BEGIN PRIVATE KEY-----` oder `-----BEGIN RSA PRIVATE KEY-----`.
- **Entropie-Check:** Suche nach langen Hexadezimal- oder Base64-Strings mit hoher Shannon-Entropie in Variablenzuweisungen, die auf nicht maskierte Schlüssel hinweisen.
- **Ausschlusskriterium:** Zugangsdaten dürfen ausschließlich aus Umgebungsvariablen (`os.getenv`) oder sicheren OS-Keystores bezogen werden, niemals aus dem Quellcode.

---

## 3. SQL-Injection, Prompt-Injection & Command-/Code-Injection

### 3.1 SQL-Injection (SQLi)
- Prüfe alle Datenbank-Aufrufe (`cursor.execute`, `session.execute`, `raw()`, `text()`).
- **Verboten:** Jegliche String-Konkatenation (`+`), `f"SELECT ... {var}"`, `.format()` oder `%`-Formatierung in SQL-Statements.
- **Gefordert:** Ausschließliche Verwendung parametrisierter Queries / Prepared Statements (`?` bzw. `:param`).

### 3.2 Prompt-Injection (Direct & Indirect Prompt Injection)
Prüfe sämtliche LLM-/KI-Prompts, Transkriptions-Pipelines und Agenten-Aufrufe:
- **Direct Prompt Injection:** Können Benutzereingaben System-Instruktionen überschreiben, weil System-Prompt und Nutzereingabe per String-Verkettung verbunden statt über strikte Rollen (`system` vs. `user`) und klare Delimiter (`<user_input>...</user_input>`) getrennt werden?
- **Indirect Prompt Injection:** Werden externe Datenquellen (Webseiten, Dokumente, Datenbankfelder, **Audio-Transkripte**) ungefiltert in Prompts oder automatisierte Folge-Aktionen (z. B. Auto-Paste via `pyautogui` / Clipboard-Befehle / Tool-Calls) eingespeist?
- **Output- & Tool-Validierung:** Werden vom Modell generierte Ausgaben oder Tool-Aufrufe vor der Ausführung gegen eine strikte Whitelist/Schema-Validierung geprüft (z. B. Schutz vor Keyboard-/Paste-Injection bei Steuerzeichen)?

### 3.3 Command-Injection, Unsichere Deserialisierung & RAM-Hygiene
- **Command-/Code-Injection:** Verbot von `eval()`, `exec()`, `os.system()` und `subprocess.Popen/run(..., shell=True)`.
- **Deserialisierung:** Ausschließliche Nutzung von `json` (Standardbibliothek) oder strikten Pydantic-Schemas; `pickle.loads()` und unsicheres `yaml.load()` sind strikt verboten.
- **Dateisystem, Race Conditions & RAM-Flushing:**
  - Temporäre Dateien dürfen niemals mit vorhersehbaren Namen in `/tmp` angelegt werden (`tempfile`-Modul mit sicheren Flags `0600` erzwingen).
  - Sensible Puffer (z. B. Audio-Arrays, Passwort-/Token-Puffer `sBuffy`) müssen nach Gebrauch im Speicher geleert/überschrieben werden, und es dürfen keine sensiblen Klartext-Inhalte (z. B. Diktate, Passwörter) in Log-Dateien landen.

---

## 4. Alle möglichen Schwachstellen & API-Angriffsvektoren

Untersuche alle APIs, IPC-Schnittstellen, UI-Eingaben und Datei-Handler auf:
1. **Authentication Bypass & Session-Schwachstellen:** Fehlende Auth-Prüfung auf Endpunkten, unsicheres Passwort-Hashing (statt `argon2`/`bcrypt`), fehlende Token-Ablaufprüfung (`exp`).
2. **Broken Object Level Authorization (BOLA / IDOR):** Zugriff auf fremde Ressourcen durch Manipulation von Objekt-IDs ohne Besitzprüfung (`resource.owner_id == current_user.id`).
3. **Fehlendes Rate-Limiting & Denial of Service (DoS):** Unbegrenzte Request-Raten, fehlende Größenlimits bei Datei-Uploads/Audio-Streams oder blockierende Endlos-Aufrufe ohne Timeout.
4. **Server-Side Request Forgery (SSRF) & Offene Weiterleitungen:** Serverseitiges Abrufen vom Nutzer übergebener URLs ohne Blockade lokaler/interner Netze (`127.0.0.1`, `169.254.169.254`, `10.0.0.0/8`).
5. **Path Traversal:** Ungeprüfte Dateipfade aus Benutzereingaben (`../../etc/passwd`) ohne `Path.resolve()` und `is_relative_to()`.
6. **CORS & Security-Header:** Überpermissive CORS-Regeln (`allow_origins=["*"]` mit Credentials).

### Pflicht zur Beschreibung konkreter Angriffsvektoren
Für **jede identifizierte Schwachstelle** muss im Bericht detailliert beschrieben werden:
- **Angriffsszenario & Vorbedingungen:** Wer kann wie angreifen?
- **Konkreter Beispiel-Payload:** Exakter Request, CLI-Aufruf, SQL-String oder Prompt-Injection-Payload.
- **Datenfluss (Source-to-Sink):** Weg der Eingabe von der Einstiegskomponente bis zur verwundbaren Funktion.
- **Konkreter Fix:** Direkt umsetzbarer Code-Vorschlag für den `coder`-Agenten.

---

## 5. Verbindliches Audit-Bericht-Template (`docs/security_audit_report.md`)

Erstelle oder aktualisiere nach jedem Audit die Datei `docs/security_audit_report.md` nach folgendem Standard.

> **Wichtig (Datenschutz & Public-Repo-Konformität):** In den Berichten (`docs/security_audit_report.md`) dürfen **keine persönlichen Entwickler-Systempfade** (wie `/home/<username>/...` oder `C:\Users\<username>\...`) aufgeführt werden, da Berichte öffentlich einsehbar sind. Verwende ausschließlich sprechende Modul- und Komponentennamen (z. B. `AudioRecorder`, `TranscriptionEngine`, `UserService`, `AuthRouter`)!

```markdown
# Sicherheits- & Supply-Chain-Auditbericht: whisper-pill

- **Datum & Uhrzeit:** [YYYY-MM-DD HH:MM:SS]
- **Prüfer:** `security-specialist` (Skill: `security-audit` + `check_downloads.py`)
- **Gesamtbewertung:** `[SECURE (PASSED) | ACTION REQUIRED (FAILED)]`
- **Geprüfter Scope:** [Komponentenübersicht ohne persönliche Dateipfade]
- **Befund-Übersicht:** `X Kritisch | Y Hoch | Z Mittel | W Niedrig`

---

## 1. Supply-Chain, Externe Libraries & Lizenz-Konformität
- **Whitelist-Konformität (`AGENTS.md`):** `[PASSED / VERSTOSS]`
- **CVE- & Vulnerability-Check:** `[Keine bekannten CVEs gefunden / Details]`
- **Malicious Code, Install-Hooks & Telemetrie (`site-packages` / `Cargo.lock`):** `[100 % sauber / Befunde]`
- **Lizenz-Konformität:** `[Alle Pakete permissiv (z. B. MIT / BSD / Apache-2.0)]`

### 1.1 Lückenlose Nachweistabelle ALLER Downloads & Installationen
*(Pflichtdokumentation für JEDEN Download und JEDE Installation im gesamten Projekt)*

| Nr. | WAS wird geladen/installiert? | WOHER? (Quell-URL / Registry + HTTPS/SHA256-Schutz) | WOHIN? (Zielort / Verzeichnis-Typ im System) | AUSLÖSER (Komponente & Methode) | Status (`OK` / `RISIKO`) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| 1 | `[z. B. Python-Abhängigkeiten oder lokales KI-Modell]` | `[z. B. PyPI / HuggingFace über HTTPS + SHA256]` | `[z. B. Lokales .venv / Lokaler Modell-Cache]` | `[z. B. Setup-Routine / ModelDownloader]` | `[OK / RISIKO]` |
| 2 | `[Bei strikten Offline-Komponenten: "0 Laufzeit-Downloads (local_files_only=True erzwungen)"]` | `-` | `-` | `TranscriptionEngine` | `OK` |

---

## 2. Secrets, API-Keys & Login-Daten (Hardcoded Credentials)
| Komponente | Gefundener Typ | Gefundener Bezeichner | Schweregrad | Status |
| :--- | :--- | :--- | :--- | :--- |
| `[Alle Module & Konfigurationen]` | `[Keine hardcodierten Keys/Passwörter gefunden]` | `-` | `Info` | `PASSED` |

---

## 3. Injection- & SAST-Prüfung (SQL, Prompt, Command, Dateisystem)
- **SQL-Injection (Prepared Statements):** `[PASSED / FAILED / N/A]`
- **Prompt-Injection (Direct & Indirect, Delimiter, Output-Sanitizing):** `[PASSED / FAILED / N/A]`
- **Command- & Code-Injection (`eval`, `exec`, `shell=True`, `pickle`):** `[PASSED / FAILED]`
- **Sichere Temp-Files, RAM-Flushing & Log-Hygiene:** `[PASSED / FAILED]`

---

## 4. API-Sicherheit & Konkrete Angriffsvektoren

*(Für jeden identifizierten Befund einen detaillierten Block anlegen; falls keine Schwachstellen existieren, explizit „Keine ausnutzbaren Angriffsvektoren identifiziert" vermerken.)*

### [FINDING-01] `[Kurztitel der Schwachstelle]`
- **Schweregrad & Kategorie:** `[KRITISCH / HOCH / MITTEL] – [z. B. Prompt-Injection / IDOR / SQLi / SSRF]`
- **Betroffene Komponente:** `[Komponenten- und Methodenname ohne Systempfad]`
- **Konkreter Angriffsvektor:**
  - **Angriffsszenario:** `[Wie geht der Angreifer vor?]`
  - **Beispiel-Payload:** ``[Konkreter bösartiger Input / Request / Prompt]``
  - **Datenfluss (Source-to-Sink):** `[Eingabe-Komponente -> Verarbeitung -> Kritische Senke]`
  - **Auswirkung (Impact):** `[Konsequenz bei erfolgreicher Ausnutzung]`
- **Verbindlicher Fix für den `coder`-Agenten:**
  ```python
  # Konkreter, sicherer Code-Fix
  ```

---

## 5. Abschließendes Prüfurteil
- `[ ]` **FREIGEGEBEN (`PASSED`):** Alle Sicherheits-, Supply-Chain- und Offline-/Download-Vorgaben sind vollständig erfüllt.
- `[ ]` **ZURÜCKGEWIESEN (`FAILED`):** Rückgabe an den `coder`-Agenten zur verpflichtenden Behebung der dokumentierten Findings.
```
