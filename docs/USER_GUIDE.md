# Whisper Pill – Benutzerhandbuch (Rust-App & AppImage)

> **Schlankes, minimalistisches, schwebendes Desktop-Diktier-Utility für Linux (KDE Plasma, XFCE, GNOME) und Windows – 100 % lokal, quelloffen und datenschutzfokussiert.**

---

## 1. Übersicht & Funktionsprinzip

**Whisper Pill** ist eine native Rust-Desktop-Anwendung mit Slint-Benutzeroberfläche, die gesprochene Sprache per Tastendruck lokal in präzisen Text inklusive Satzzeichen und Kommas umwandelt. Das fertige Diktat wird sofort in die System-Zwischenablage gelegt und steht überall mit <kbd>Strg</kbd> + <kbd>V</kbd> zum Einfügen bereit.

* **100 % Offline & Privat:** Die gesamte Spracherkennung erfolgt lokal auf deiner CPU über `whisper.cpp` (`whisper-rs`). Kein Cloud-Server, keine Abokosten, keine Datenübertragung.
* **Tray-First-Bedienkonzept:** Whisper Pill startet unaufdringlich im System-Tray (Benachrichtigungsbereich der Taskleiste) und blendet die schwebende Kapsel („Pill“) nur dann ein, wenn du diktierst oder deinen Text bearbeiten möchtest.
* **Portables Linux-AppImage:** Die gesamte Anwendung ist als einzelnes, ca. **13 MB** schlankes AppImage gepackt und läuft ohne Installation.

---

## 2. Installation & Schnellstart (AppImage)

### 2.1 AppImage direkt starten
Für das AppImage (`Whisper_Pill-x86_64.AppImage`) sind keine Root-Rechte und keine Paketinstallationen erforderlich. Es empfiehlt sich, die Datei an einem festen Ort im eigenen Benutzerverzeichnis abzulegen (z. B. unter `~/Applications/` oder `~/Programme/`), damit der optionale System-Autostart die Datei dauerhaft findet.

```bash
# 1. Ausführrechte sicherstellen (einmalig)
chmod +x dist/Whisper_Pill-x86_64.AppImage

# 2. Whisper Pill starten (startet im System-Tray)
./dist/Whisper_Pill-x86_64.AppImage
```

### 2.2 Kommandozeilen-Optionen (CLI)
```bash
# Installierte Version prüfen
./dist/Whisper_Pill-x86_64.AppImage --version

# Hilfe und verfügbare Parameter anzeigen
./dist/Whisper_Pill-x86_64.AppImage --help

# Screenshots aller UI-Zustände automatisch neu generieren
./dist/Whisper_Pill-x86_64.AppImage --capture-screenshots docs/assets
```

> **Hinweis zu FUSE unter aktuellen Linux-Distributionen:** Falls ein AppImage auf einem frisch installierten System nicht direkt startet, stelle sicher, dass `libfuse2` (Ubuntu/Debian: `sudo apt install libfuse2t64` bzw. `libfuse2`, Arch/Manjaro: `fuse2`, Fedora: `fuse-libs`) installiert ist.

---

## 3. Desktop-Umgebungen: KDE Plasma & XFCE im Detail

### 3.1 Nutzung unter KDE Plasma (Wayland & X11)
Whisper Pill bietet eine tiefgreifende, vollautomatische Integration für **KDE Plasma 5 & 6**:
* **Globaler Shortcut (`Super` + `Strg` + `P`):** Beim Start registriert Whisper Pill das globale Tastenkürzel automatisch beim KDE-Dienst `KGlobalAccel`. Du kannst das Kürzel jederzeit auch in den *KDE-Systemeinstellungen → Kurzbefehle → Whisper Pill* einsehen oder anpassen.
* **Strikte Signal-Isolierung:** Der D-Bus-Listener reagiert ausschließlich auf die eigene Aktion (`whisper_pill` / `toggle_pill`). Andere globale KDE-Shortcuts wie <kbd>F12</kbd> (Yakuake Dropdown-Terminal) oder die <kbd>Super</kbd>-Taste (Anwendungsstarter) werden niemals gestört oder fälschlich ausgelöst.
* **Automatische KWin-Fensterregeln:** Whisper Pill legt selbstständig passende KWin-Regeln an, sodass die schwebende Kapsel weder in der Kontrollleiste (Taskbar) noch im Fensterwechsler (<kbd>Alt</kbd> + <kbd>Tab</kbd>) auftaucht. Zudem ist die Z-Order so abgestimmt, dass Dropdown-Terminals wie **Yakuake** sauber im Vordergrund öffnen können.
* **System-Tray (StatusNotifierItem):** Das Kapsel-Icon erscheint direkt im KDE-Systemabschnitt der Kontrollleiste und wechselt während einer laufenden Transkription dynamisch auf eine bernsteinfarbene Wellenform (`ılı`).

### 3.2 Nutzung unter XFCE (X11)
Auch unter **XFCE** (z. B. Xubuntu, Manjaro XFCE, Debian XFCE) läuft das AppImage nativ und ressourcenschonend:
* **System-Tray in der XFCE-Leiste:** Whisper Pill nutzt das moderne Freedesktop `StatusNotifierItem`-Protokoll (`ksni`). Stelle sicher, dass in deiner XFCE-Leiste das Standard-Plugin **„Benachrichtigungsfläche“ (Status Tray Plugin / `xfce4-statusnotifier-plugin`)** aktiviert ist. Dort erscheint das Whisper-Pill-Icon mit vollem Rechtsklick-Menü.
* **Sauberes Fensterverhalten unter Xfwm4:** Über EWMH-X11-Fensterattribute (`_NET_WM_STATE_SKIP_TASKBAR` und `_NET_WM_STATE_SKIP_PAGER`) sorgt Whisper Pill dafür, dass die Kapsel nicht als gewöhnliches Programmfenster in der XFCE-Fensterknopfleiste erscheint, sondern als rahmenlose Kapsel über deinen Arbeitsfenstern schwebt.
* **Globales Tastenkürzel unter XFCE einrichten:** Da XFCE standardmäßig keinen KDE-`KGlobalAccel`-Daemon nutzt, kannst du unter XFCE die Aufnahme entweder bequem per **Linksklick / Rechtsklick auf das Tray-Icon** starten oder in den **XFCE-Tastatureinstellungen** (`Einstellungen → Tastatur → Tastenkürzel für Anwendungen`) einen eigenen Kurzbefehl für den Start bzw. das Einblenden hinterlegen.
* **Audio-Anbindung (PipeWire / PulseAudio / ALSA):** Unter XFCE greift Whisper Pill über `cpal` automatisch auf das Standard-Eingabegerät (PipeWire oder PulseAudio) zu; die Mikrofonauswahl lässt sich wie gewohnt über `pavucontrol` (PulseAudio-Lautstärkeregler) steuern.

---

## 4. Die Oberflächen-Zustände der Rust-App

### 4.1 Ruhezustand (`Idle` – `520 × 48 px`)
Wenn die Kapsel eingeblendet ist und auf eine Aufnahme wartet, zeigt sie links den Titel und rechts den Start-Button `[● Aufnehmen]`:

![Ruhezustand](assets/state_0_idle.png)

### 4.2 Aktive Sprachaufnahme (`Recording` – `520 × 48 px`)
Sobald du sprichst, visualisieren **11 zentrierte Wellenform-Balken** in Echtzeit deinen Mikrofonpegel. Rechts laufen der sekundenweise Timer und der `[■ Stop]`-Button. Erkennst du eine längere Sprechpause, beendet die integrierte **Voice Activity Detection (VAD)** die Aufnahme auf Wunsch automatisch:

![Aufnahme läuft](assets/state_1_recording.png)

### 4.3 Fertig & Kopiert (`Ready` – `520 × 48 px`)
Nach der CPU-Transkription liegt der Text sofort in deiner Zwischenablage (`✔ In Zwischenablage kopiert!`). Du hast direkt folgende Möglichkeiten:
* Sofort im Zielprogramm <kbd>Strg</kbd> + <kbd>V</kbd> drücken.
* **`[+ Weiter]`:** Startet eine Folgeaufnahme und hängt den neuen Satz nahtlos mit Leerzeichen an das bestehende Diktat an.
* **`[● Neu]`:** Verwirft das bisherige Diktat und startet eine frische Aufnahme.
* **`[↗ Details]`:** Klappt den Texteditor zur Prüfung und Korrektur auf.
* **Auto-Dismiss (8 Sekunden):** Wenn du nichts anklickst, blendet sich die Kapsel nach 8 Sekunden automatisch wieder in den System-Tray aus.

![In Zwischenablage kopiert](assets/state_2_ready.png)

### 4.4 Ausgeklappter Text-Editor (`Expanded` – `520 × 240 px`)
Im erweiterten Modus kannst du längere Texte prüfen, frei bearbeiten und mit dem **Mausrad** oder über die **rechte Scrollbar** durch das Transkript navigieren:
* **`[▲ Zuklappen]`:** Kehrt zur kompakten Kapselansicht zurück.
* **`[– Verbergen]`:** Minimiert das Fenster in den System-Tray.
* **`[✕ Beenden]`:** Schließt Whisper Pill vollständig.
* **`[⎘ Text kopieren]`:** Übernimmt deine Änderungen im Textfeld wieder in die System-Zwischenablage.

![Ausgeklappter Editor](assets/state_3_expanded.png)

### 4.5 Integrierter Einstellungs-Dialog (`Settings` – `580 × 480 px`)
Über einen Rechtsklick auf das Tray-Icon → **Einstellungen...** öffnet sich das Konfigurationsfenster:

![Einstellungs-Dialog](assets/state_4_settings.png)

Hier stehen dir vier zentrale Optionen zur Verfügung:
1. **Mit dem System starten (Autostart):** Erstellt automatisch einen Freedesktop-Autostart-Eintrag (`~/.config/autostart/whisper-pill.desktop`). Beim Betrieb als AppImage erkennt Whisper Pill selbstständig den persistenten Pfad deiner `.AppImage`-Datei (statt des temporären `/tmp/.mount_*`-Ordners) und repariert den Pfad automatisch, falls du das AppImage aktualisierst.
2. **KI-Sprachmodell (`base` vs. `small`):**
   * **`base` (~140 MB):** Sehr schnelle Transkription bei geringem RAM-Bedarf – ideal für klare Aussprache und kürzere Notizen.
   * **`small` (~460 MB):** Höhere Erkennungsgüte bei schnellem Sprechen, Nuscheln, Dialektfärbung oder Fachbegriffen.
   * Ist ein gewähltes Modell lokal unter `~/.cache/whisper_pill/` noch nicht vorhanden, lädt Whisper Pill es beim Klick auf **`[Speichern & Übernehmen]`** automatisch im Hintergrund von der offiziellen HuggingFace-Quelle herunter.
3. **Automatische Lautstärkeanpassung (Gain):** Hebt leise Mikrofonaufnahmen vor der KI-Analyse dynamisch auf einen optimalen Zielpegel an (bis maximal 5-fache Verstärkung ohne Übersteuern).
4. **Erkennungs-Modus (Suchstrategie):**
   * **Schnell (Greedy):** Wählt unmittelbar das wahrscheinlichste Wort für minimale Wartezeit.
   * **Gründlich (Beam Search):** Prüft 5 parallele Satzhypothesen für bestmögliche Grammatik und Kommasetzung.

---

## 5. Tastenkombinationen & Maussteuerung

| Aktion | Tastenkürzel / Geste | Beschreibung |
|:---|:---|:---|
| **Aufnahme starten / stoppen** | <kbd>Super</kbd> + <kbd>Strg</kbd> + <kbd>P</kbd> | Globaler Hotkey (automatisch unter KDE Plasma registriert) oder per Klick auf das Tray-Icon. |
| **Transkript einfügen** | <kbd>Strg</kbd> + <kbd>V</kbd> | Fügt den erkannten Text an der aktuellen Cursor-Position ein. |
| **Kapsel frei verschieben** | Linke Maustaste gedrückt halten (Drag & Drop) | Ziehe die kompakte Kapsel oder die Kopfleiste an deine Wunschposition; die Koordinaten werden dauerhaft in `~/.config/whisper_pill/config.json` gespeichert. |
| **System-Tray-Menü öffnen** | Rechtsklick auf das Tray-Icon | Zugriff auf *Aufnahme starten/stoppen*, *Autostart*, *Einstellungen...*, *Fehlerlog öffnen* und *Beenden*. |
| **Kapsel in Tray verbergen** | `[– Verbergen]`, `[✕]` oder 8 s warten | Blendet das Overlay aus, während Whisper Pill im Hintergrund weiterläuft. |
| **Anwendung beenden** | `[✕ Beenden]` im Editor oder Tray → *Beenden* | Beendet den Prozess vollständig. |

---

## 6. Diagnose, Konfigurationsdateien & Fehlerbehebung

### Speicherorte im Benutzerverzeichnis
* **Konfiguration:** `~/.config/whisper_pill/config.json` (speichert gewähltes Modell, Gain, Beam Search, Autostart und Fensterposition `window_x` / `window_y`).
* **KI-Modelle:** `~/.cache/whisper_pill/ggml-base.bin` bzw. `ggml-small.bin`.
* **Fehler- & Diagnose-Log:** `~/.local/share/whisper_pill/whisper_pill_error.log`.
* **Autostart-Eintrag:** `~/.config/autostart/whisper-pill.desktop`.

### Häufige Fragen & Lösungen
1. **„Whisper-Modell nicht gefunden“ beim ersten Diktat:**
   Öffne über Rechtsklick auf das Tray-Icon die **Einstellungen...**, wähle `base` oder `small` und klicke auf **Speichern & Übernehmen** (oder führe `./scripts/download_model.sh base` im Terminal aus).
2. **Kein Mikrofonsignal / Ausschlag in der Wellenform:**
   Prüfe in deinen Audio-Einstellungen (KDE Audio-Applet oder `pavucontrol` unter XFCE), ob das richtige Eingabegerät als Standard-Mikrofon ausgewählt und nicht stummgeschaltet ist. Aktiviere zudem in den Whisper-Pill-Einstellungen die **Automatische Lautstärkeanpassung (Gain)**.
3. **Fehlerprotokoll einsehen:**
   Klicke mit der rechten Maustaste auf das Tray-Icon und wähle **Fehlerlog öffnen**. Die Datei öffnet sich direkt in deinem Standard-Texteditor mit Zeitstempeln und genauen Diagnosehinweisen.
