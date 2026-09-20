---
name: gitignore-analyzer
description: Analysiert die Verzeichnisstruktur des Projekts, erkennt nicht getrackte Artefakte, Build-Dateien, Modelle und Caches und schlägt optimierte .gitignore-Einträge vor oder aktualisiert die .gitignore.
---

# Gitignore Analyzer & Hygiene Workflow

Führe bei Aktivierung dieses Skills eine systematische Analyse des Projektverzeichnisses durch, um versehentliches Einchecken von Build-Artefakten, lokalen Modellen und sensiblen Dateien zu verhindern.

## 1. Verzeichnis- und Status-Inspektion
1. **Nicht ignorierte & untracked Files ermitteln:**
   ```bash
   git status --porcelain -uall
