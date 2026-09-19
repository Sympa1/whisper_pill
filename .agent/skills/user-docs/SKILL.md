---
name: user-docs
description: Automatisiert die Erstellung des Endbenutzer-Handbuchs. Erstellt Screenshots der Pill-Zustände und baut die finale, autarke HTML-Dokumentation.
---

# User Documentation Build Workflow

Führe bei Aktivierung dieses Skills folgende Schritte aus:

## 1. Screenshot-Generierung (Headless / Automated)
* Führe das Hilfsskript `prototype/scripts/capture_ui_states.py` aus.
* Validiere, dass folgende Bilddateien erzeugt wurden:
  * `docs/assets/state_1_recording.png`
  * `docs/assets/state_2_ready.png`
  * `docs/assets/state_3_expanded.png`

## 2. HTML-Build & Asset-Inlining
* Lies die Vorlage `docs/templates/user_guide_template.html` ein.
* Setze die generierten Screenshots und aktuellen Hotkeys ein.
* Stelle sicher, dass CSS und Skripte inline eingebunden sind (Zero-CDN, vollständige Offline-Nutzung).

## 3. Validierung & Sanity-Check
* Prüfe die generierte Datei `docs/user_guide.html`:
  * Keine externen `<link>`- oder `<script>`-Tags zu Drittanbietern.
  * Korrekte responsive Darstellung für Desktop-Browser.
  * Lokale relative Pfade zu allen Assets.
