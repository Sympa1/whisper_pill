---
name: gitignore-analyzer
description: Analysiert die Verzeichnisstruktur des Projekts, erkennt nicht getrackte Artefakte, Build-Dateien, lokale KI-Modelle, Caches und Secrets und hält die .gitignore sauber.
---

# SKILL: Gitignore Analyzer & Repository-Hygiene (`gitignore-analyzer`)

> **Rolle & Einsatzzweck:**
> Dieser Skill verhindert das versehentliche Einchecken von temporären Build-Artefakten, virtuellen Umgebungen, großen lokalen Sprach-/KI-Modellen, Caches und sensiblen `.env`-Dateien in das Versionskontrollsystem.

---

## 1. Verzeichnis- und Status-Inspektion

1. **Nicht ignorierte & untracked Dateien ermitteln:**
   ```bash
   git status --porcelain -uall
   ```
2. **Große Dateien & Modell-Gewichte aufspüren (> 5 MB):**
   Prüfe, ob Binärdateien (`.bin`, `.pt`, `.onnx`, `.gguf`, `.safetensors`, `.wav`, `.db`, `.sqlite3`) im Arbeitsverzeichnis liegen, die nicht in Git eingecheckt werden dürfen.

---

## 2. Pflicht-Kategorien für eine saubere `.gitignore`

Stelle sicher, dass die `.gitignore` des Projekts mindestens folgende Schutzblöcke abdeckt:

```gitignore
# 1. Secrets, lokale Konfigurationen & Credentials (KRITISCH!)
.env
.env.*
!.env.example
*.pem
*.key
*secret*
*credentials*.json

# 2. Python Bytecode, Virtuelle Umgebungen & Test-/Linter-Caches
__pycache__/
*.py[cod]
.venv/
venv/
.pytest_cache/
.mypy_cache/
.ruff_cache/
.coverage
htmlcov/

# 3. Rust / C++ / Node Build-Artefakte (falls mehrstufiges Projekt)
target/
dist/
build/
node_modules/

# 4. Lokale Datenbanken, KI-Modelle & Audio-/Temp-Artefakte
*.db
*.sqlite3
*.db.backup
models/
*.gguf
*.pt
*.onnx
*.wav
tmp/
```

---

## 3. Checkliste für den Agenten
- `[ ]` Keine `.env`-Dateien oder privaten Schlüssel im Git-Index (`git status`).
- `[ ]` Keine `__pycache__`-Ordner oder `.venv`-Verzeichnisse versioniert.
- `[ ]` Öffentliche Berichte in `docs/*_report.md` sind frei von persönlichen Systempfaden (`/home/<username>/...`).
