---
name: code-refactoring
description: Strukturierter Leitfaden für verhaltensbewahrendes Code-Refactoring nach Clean-Code- und SOLID-Prinzipien im isolierten Branch.
---

# SKILL: Sicheres & Verhaltensbewahrendes Code-Refactoring (`code-refactoring`)

> **Rolle & Einsatzzweck:**
> Dieser Skill unterstützt den **`researcher`-** und **`coder`-Agenten** dabei, technischen Schulden (Code Smells, zu lange Methoden, Verletzungen des Single Responsibility Principle) abzubauen, **ohne das beobachtbare fachliche Außenverhalten zu verändern**.

---

## 1. Grundregeln des Refactorings im 4-Agenten-Workflow

1. **Strikte Trennung von Refactoring und Feature-Änderung:**
   Verhaltensänderungen (neue Features) und strukturelle Bereinigungen (Refactoring) dürfen nicht unkontrolliert vermischt werden.
2. **Sicherheitsnetz durch bestehende Tests:**
   Vor Beginn jedes Refactorings müssen alle bestehenden Unit-Tests (`./prototype/.venv/bin/pytest tests/` und `cargo test --manifest-path app/Cargo.toml`) grün sein. Fehlen Tests für den zu überarbeitenden Bereich, müssen **zuerst Charakterisierungs-Tests** ergänzt werden.
3. **Abstimmung mit dem `researcher`-Agenten:**
   Weicht der `coder`-Agent während des Refactorings von der vom `researcher` entworfenen Zielstruktur ab, muss er den `researcher` via `send_message` nach einer Alternative fragen, statt eigenmächtig Schnittstellen umzubauen.

---

## 2. Typische Refactoring-Muster (Clean Code & OOP)

- **Extract Method / Function:** Lange Methoden (> 30 Zeilen oder mehrere Abstraktionsebenen) in kleine, sprechend benannte englische Hilfsmethoden mit deutschen Docstrings aufteilen.
- **Extract Class (SRP):** Klassen, die mehrere Verantwortlichkeiten mischen (z. B. Geschäftslogik + direkter Datei-/Datenbankzugriff), in getrennte Service- und Repository-Klassen aufteilen.
- **Replace Magic Numbers with Symbolic Constants:** Hartcodierte Zahlenwerte oder Strings in sprechende Modul-/Klassenkonstanten (`UPPER_SNAKE_CASE`) überführen.
- **Introduce Parameter Object / Dataclass:** Funktionen mit mehr als 4–5 Einzelparametern durch ein typisiertes `dataclass`- oder `Pydantic`-Objekt vereinfachen.
- **Guard Clauses statt tiefer Verschachtelung:** Tiefe `if/else`-Pyramiden durch frühe Rückgaben (`return` / `raise`) am Methodenbeginn auflösen.

---

## 3. Checkliste vor Übergabe an `tester` & `security-specialist`

- `[ ]` Öffentliche Signaturen (Public API) blieben kompatibel oder alle Aufrufer im gesamten Projekt wurden vollständig angepasst.
- `[ ]` Englische Bezeichner, deutsche Kommentare/Docstrings (Fokus auf das *Warum*) und vollständige Type-Hints sind durchgängig vorhanden.
- `[ ]` Paralleler Start von `tester` (mit Fail-Fast-Abbruchregel) und `security-specialist` wurde nach Abschluss des Refactorings ausgelöst.
