---
name: tech-docs
description: Erstellt und pflegt die technische Entwickler- und Architektur-Dokumentation in Markdown inklusive nativer UML-Diagramme via Mermaid.js.
---

# Technical Documentation Workflow

Erstelle präzise technische Dokumentationen für Entwickler nach folgenden Standards:

## 1. Ausgabeformat & Struktur
* **Format:** Reines GitHub-flavored Markdown.
* **Ablageort:** `docs/architecture/` oder `docs/api/`.
* **Inhalte:** Klassen- und Modulbeschreibungen, Datenflüsse, Zustandsübergänge und Thread-Architektur (Audio-Thread vs. UI-Thread).

## 2. UML-Diagramme via Mermaid.js
Binde Architekturpläne direkt als Mermaid-Codeblöcke ein:
* **Zustandsdiagramm (Pill-States):**
  ```mermaid
  stateDiagram-v2
      [*] --> Recording: Hotkey / Klick
      Recording --> Processing: Aufnahme Stop
      Processing --> Ready: Transkription fertig (Text im Clipboard)
      Ready --> Expanded: Klick auf Edit/Details
      Expanded --> Ready: Minimieren
      Ready --> [*]: Timeout / Hide
