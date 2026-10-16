# Whisper Pill – Benutzerhandbuch

> **Schlankes, minimalistisches Desktop-Diktier-Utility (Pill-UI) für Linux und Windows – 100 % lokal, quelloffen und datenschutzfokussiert.**

---

## 1. Übersicht

Whisper Pill dockt als schlanke, schwebende Kapsel am Bildschirmrand an und wandelt gesprochene Sprache per Tastendruck in präzisen Text um – inklusive automatischer Satzzeichen- und Kommasetzung. Das Transkript wird direkt in deine Zwischenablage kopiert und steht sofort per <kbd>Strg</kbd> + <kbd>V</kbd> zum Einfügen bereit.

* **100 % Offline:** Keine Cloud-Verbindung, keine API-Kosten, keine Datenübertragung.
* **Tray-First:** Läuft unaufdringlich im System-Tray und öffnet die Kapsel nur bei Bedarf.
* **Modernes Design:** Dunkle, abgerundete Kapseloberfläche mit Live-Audiopegel und Statusanzeige.

---

## 2. Installation & Schnellstart

### Ausführen als autarkes Linux-AppImage
Whisper Pill benötigt keine Installation und kann direkt als AppImage gestartet werden:

```bash
# AppImage ausführen
./dist/Whisper_Pill-x86_64.AppImage

# Versionsprüfung
./dist/Whisper_Pill-x86_64.AppImage --version

# Hilfe & CLI-Optionen anzeigen
./dist/Whisper_Pill-x86_64.AppImage --help
```

---

## 3. Tastaturkürzel & Steuerung

| Aktion | Tastenkürzel / Geste | Beschreibung |
|:---|:---|:---|
| **Aufnahme starten / beenden** | <kbd>Super</kbd> + <kbd>Strg</kbd> + <kbd>P</kbd> | Startet oder stoppt die Sprachaufnahme global aus jedem Programm heraus. |
| **Text einfügen** | <kbd>Strg</kbd> + <kbd>V</kbd> | Fügt das fertige Diktat direkt an der Cursor-Position ein. |
| **Kapsel verschieben** | Linke Maustaste gedrückt halten (Drag & Drop) | Die Fensterposition wird automatisch gespeichert und beim nächsten Start wiederhergestellt. |
| **Kontextmenü aufrufen** | Rechtsklick auf Tray-Icon | Öffnet das Menü für Aufnahme, Einstellungen, Fehlerlog und Beenden. |
| **Kapsel verbergen** | Klick auf <kbd>✕</kbd> oder Klick außerhalb | Minimiert die Pill zurück in den System-Tray. |

---

## 4. Die Zustände der Kapsel

```text
1. Aufnahme (Recording)
╭──────────────────────────────────────────────────────────╮
│  ● REC   ılılı·|·lılı   00:04                            │
╰──────────────────────────────────────────────────────────╯

2. Fertig / Kopiert (Ready)
╭──────────────────────────────────────────────────────────╮
│  ✔ In Zwischenablage kopiert!        [ ⤢ Details / Edit ]│
╰──────────────────────────────────────────────────────────╯

3. Ausgeklappt (Review & Korrektur)
╭──────────────────────────────────────────────────────────╮
│  [X] Schließen                       [ ⎘ Text kopieren ] │
│ ┌──────────────────────────────────────────────────────┐ │
│ │ Hier steht der fertig transkribierte Text mit allen  │ │
│ │ Satzzeichen und Kommas, bereit zur Prüfung...        │ │
│ └──────────────────────────────────────────────────────┘ │
╰──────────────────────────────────────────────────────────╯
```

---

## 5. Einstellungen & Anpassung

Über einen Rechtsklick auf das System-Tray-Icon &rarr; **Einstellungen...** erreichst du das Konfigurationsmenü:

* **Modellauswahl:**
  * **Base (`ggml-base.bin`):** Blitzschnell, ca. 140 MB, ideal für den täglichen Standard-Einsatz.
  * **Small (`ggml-small.bin`):** Höhere Erkennungsgenauigkeit, ca. 460 MB, empfohlen für anspruchsvolle Fachsprache.
* **Suchstrategie (Beam Search):**
  * *Beam Search (5 Beams):* Maximale Satzqualität und Kontextanalyse.
  * *Greedy Search:* Minimale Inferenzzeit bei einfachen Sätzen.
* **Automatische Lautstärkeanpassung (Gain):**
  * Gleicht leise Mikrofonsignale dynamisch an.
* **Mit dem System starten (Autostart):**
  * Startet Whisper Pill automatisch beim Hochfahren deines Computers im Hintergrund.

---

## 6. Diagnose & Fehlerbehebung

Sollte ein Audiosignal nicht erfasst werden oder ein Modell fehlen:
1. Klicke mit der rechten Maustaste auf das System-Tray-Icon.
2. Wähle **Fehlerlog anzeigen...**.
3. Das Logfile öffnet sich direkt in deinem Standard-Texteditor mit genauen Fehlerdetails und Lösungshinweisen.
