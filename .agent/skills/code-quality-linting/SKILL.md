---
name: code-quality-linting
description: Führt statische Code-Analyse, Formatierungsprüfungen mit ruff und strikte Typüberprüfung mit mypy nach den Konventionen der AGENTS.md durch.
---

# SKILL: Statische Code-Qualität, Linting & Typüberprüfung (`code-quality-linting`)

> **Rolle & Einsatzzweck:**
> Dieser Skill unterstützt den **`coder`-Agenten** und den **`tester`-Agenten** dabei, sicherzustellen, dass sämtlicher Quellcode den PEP-8-Formatierungsvorgaben, der strengen Typisierung (`typing`) und den Sprachkonventionen der `AGENTS.md` entspricht.

---

## 1. Benötigte Werkzeuge

- **`ruff`**: Schneller Python-Linter und Code-Formatter (ersetzt `flake8`, `black`, `isort`).
- **`mypy`**: Statischer Type-Checker zur Verifikation aller Parameter- und Rückgabe-Type-Hints.

---

## 2. Standard-Befehle zur Prüfung

```bash
# Whisper-Pill Linting- & Statische Prüfung (Rust Clippy & Python Ruff):
cargo clippy --all-targets -- -D warnings
ruff check prototype/ tests/ || true

# A) Linting & Import-Sortierung prüfen:
ruff check src/ tests/

# B) Formatierung prüfen (ohne Dateien ungefragt umzuschreiben):
ruff format --check src/ tests/

# C) Statische Typprüfung mit Mypy:
mypy src/ --ignore-missing-imports
```

---

## 3. Verbindliche Prüf-Checkliste (Clean Code & `AGENTS.md`)

1. **Sprachkonventionen:**
   - `[ ]` Alle Bezeichner (Klassen, Funktionen, Methoden, Variablen, Module) sind auf **Englisch** verfasst.
   - `[ ]` Alle Docstrings und Inline-Kommentare sind auf **Deutsch** verfasst und erklären das fachliche *Warum*.
2. **Strikte Typisierung:**
   - `[ ]` Jede Funktion und Methode besitzt vollständige Type-Hints für sämtliche Argumente und den Rückgabewert (inkl. `-> None`).
   - `[ ]` Keine unkommentierten `# type: ignore`-Anweisungen ohne zwingende technische Begründung.
3. **Code-Hygiene:**
   - `[ ]` Keine ungenutzten Importe, Variablen oder auskommentierten Code-Leichen.
   - `[ ]` Keine Magic Numbers oder Magic Strings im Code (Einsatz benannter Konstanten in `UPPER_SNAKE_CASE`).
   - `[ ]` Zeilenlänge max. 100 Zeichen, 4 Leerzeichen Einrückung.
