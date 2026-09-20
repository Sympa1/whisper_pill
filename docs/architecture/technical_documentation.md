# Technische Architektur- und Entwicklerdokumentation: whisper-pill (Prototyp)

Diese Dokumentation beschreibt die interne Software-Architektur, Modulstrukturen, Datenflüsse und Threading-Modelle des **whisper-pill** Python-Prototyps.

---

## 1. Systemübersicht & Design-Prinzipien

`whisper-pill` ist ein schlankes, minimalistisches, schwebendes Desktop-Diktier-Utility (Pill-UI) für Linux und Windows als freie, quelloffene und 100 % lokale Alternative zu proprietären Tools.

### Kernprinzipien:
* **100 % Offline / Zero-Cloud:** Vollständige lokale Inferenz auf der CPU via `faster-whisper` (CTranslate2). Keine externen Server, keine Telemetrie, kein Cloud-Backend.
* **Non-Blocking UI:** Entkoppelte Thread-Architektur; die GUI friert während der Audioaufnahme oder der KI-Transkription zu keinem Zeitpunkt ein.
* **Wayland & X11 Native:** Volle Kompatibilität mit modernen Linux-Desktops (KDE Plasma 6 Wayland) über native Qt- und D-Bus-Schnittstellen (`startSystemMove`, `QDBus`, `QClipboard`).
* **Minimaler Footprint:** Streng limitierte Abhängigkeiten (Whitelist-Policy) und CPU-schonende Standby-Timer.

---

## 2. Zustandsautomat (Pill State Machine)

Die Pill besitzt fünf wohldefinierte Zustände mit einer **durchgehend einheitlichen Kapselbreite von 420 px**.

```mermaid
stateDiagram-v2
    [*] --> IDLE
    IDLE --> RECORDING: Super+Strg+P / Klick auf [● Aufnehmen]
    RECORDING --> PROCESSING: Klick auf [■ Stop] / Shortcut
    PROCESSING --> READY: Inferenz abgeschlossen & Text im Clipboard
    PROCESSING --> IDLE: Leeres Audiosignal / Abbruch
    READY --> EXPANDED: Klick auf [⤢ Details / Edit]
    READY --> IDLE: Auto-Hide Timer (8s) / Klick auf [✕]
    EXPANDED --> READY: Klick auf [⎘ Text kopieren]
    EXPANDED --> IDLE: Fenster minimieren / schließen
```

### Die Zustände im Detail:
1. **`IDLE` (420×48 px):** Ruhezustand der Kapsel. Zeigt `🎙 whisper-pill` und den Start-Button. Timer und Waveform-Animationen sind gestoppt (0 % CPU-Last).
2. **`RECORDING` (420×48 px):** Audio-Stream aktiv. Waveform-Widget visualisiert den Live-RMS-Pegel (`ılılı·|·lılı`), Aufnahmezeit wird sekundenweise hochgezählt.
3. **`PROCESSING` (420×48 px):** Hintergrund-Inferenz via `faster-whisper`. Zeigt `⟳ Transkribiere Audiosignal...`.
4. **`READY` (420×48 px):** Erfolgreich transkribierter Text liegt im System-Clipboard. Zeigt `✔ In Zwischenablage kopiert!` und Schnellzugriff auf Details. Fällt nach 8 Sekunden automatisch auf `IDLE` zurück.
5. **`EXPANDED` (420×240 px):** Texteditor zur Prüfung und Korrektur des Transkripts mit Buttons `[✕ Verbergen]`, `[⏻ Beenden]` und `[⎘ Text kopieren]`.

---

## 3. Klassen- und Modulstruktur

```mermaid
classDiagram
    class PillWindow {
        +PILL_WIDTH: int = 420
        +PILL_HEIGHT_COMPACT: int = 48
        +PILL_HEIGHT_EXPANDED: int = 240
        -AudioRecorder _recorder
        -WhisperTranscriber _transcriber
        -CompactStackedWidget _stack
        -WaveformWidget _waveform
        -QTimer _record_timer
        -QTimer _auto_hide_timer
        +set_state(state: PillState)
        +start_recording()
        +stop_recording()
        +toggle_visibility()
        +toggle_recording()
        +quit_application()
    }

    class AudioRecorder {
        -int sample_rate = 16000
        -Queue _audio_queue
        -InputStream _stream
        -bool _is_recording
        +start_recording()
        +stop_recording() np.ndarray
        -_audio_callback(indata, frames, time, status)
    }

    class WhisperTranscriber {
        -WhisperModel _model
        -str model_size
        -str device
        -str compute_type
        +transcribe(audio_data: np.ndarray) tuple[str, dict]
        +copy_to_clipboard(text: str)$ bool
    }

    class TranscriptionWorker {
        -WhisperTranscriber _transcriber
        -np.ndarray _audio_data
        +run()
        +finished: Signal
        +failed: Signal
    }

    class HotkeyManager {
        -Callable _toggle_callback
        +setup() bool
        -_setup_windows() bool
        -_setup_linux() bool
        -_register_kglobalaccel() bool
        -_on_kde_shortcut_pressed(comp, action, ts)
    }

    class PillTrayIcon {
        -QWidget _pill_window
        -_create_tray_icon() QIcon
        -_setup_menu()
        -_toggle_pill()
    }

    PillWindow --> AudioRecorder: verwendet
    PillWindow --> WhisperTranscriber: delegiert an
    PillWindow --> TranscriptionWorker: startet asynchron
    PillWindow --> PillTrayIcon: besitzt
    PillWindow --> HotkeyManager: bindet Callbacks
```

---

## 4. Thread-Architektur & Datenfluss

Die Anwendung trennt strikt zwischen Audio-Erfassung, Benutzeroberfläche und KI-Inferenz:

```mermaid
sequenceDiagram
    autonumber
    actor User as Benutzer
    participant HW as Mikrofon (PortAudio)
    participant AR as AudioRecorder (Audio-Thread)
    participant UI as PillWindow (Main GUI Thread)
    participant TW as TranscriptionWorker (QThread)
    participant WT as WhisperTranscriber (CTranslate2)
    participant CB as System Clipboard (Wayland / OS)

    User->>UI: Shortcut Super+Strg+P / Klick [Aufnehmen]
    UI->>AR: start_recording()
    AR->>HW: sounddevice.InputStream(16kHz, mono, float32)
    loop Audio Frames (50ms)
        HW-->>AR: audio callback (chunk)
        AR->>AR: RMS Pegel berechnen (iBuffy_rms)
        AR-->>UI: level_updated(float) via Signal
        UI->>UI: Waveform animieren
    end

    User->>UI: Shortcut / Klick [■ Stop]
    UI->>AR: stop_recording()
    AR-->>UI: return np.ndarray (gesamte Aufnahme)
    UI->>UI: set_state(PROCESSING)
    UI->>TW: start(worker)
    activate TW
    TW->>WT: transcribe(audio_data)
    WT->>WT: CTranslate2 Inferenz (CPU int8)
    WT-->>TW: (transcribed_text, metadata)
    TW-->>UI: finished(text, meta) via Qt Signal
    deactivate TW
    UI->>CB: WhisperTranscriber.copy_to_clipboard(text)
    UI->>UI: set_state(READY)
    Note over UI,CB: Text steht sofort in jeder Anwendung via Strg+V bereit!
```

---

## 5. Plattform-Integration & Globale Hotkeys

### Linux (Wayland / KDE Plasma 6):
* **Problemstellung:** Unter Wayland verbietet der Compositor globalen X11-Key-Grabbing aus Sicherheitsgründen.
* **Lösung:** Native Anbindung an den **KDE KGlobalAccel D-Bus-Dienst**:
  1. Beim Start registriert `HotkeyManager` die Komponente `whisper_pill` mit der Aktion `toggle_pill` via D-Bus bei `org.kde.KGlobalAccel`.
  2. Der Qt-Keycode `335544400` (`Qt.Key_P | Qt.MetaModifier | Qt.ControlModifier` = `Super+Strg+P`) wird aktiv zugewiesen.
  3. Die Applikation verbindet sich mit dem Signal `org.kde.kglobalaccel.Component.globalShortcutPressed` und schaltet die Sichtbarkeit ohne Verzögerung oder Subprozess-Overhead um.
  4. Ergänzend wird `~/.local/share/applications/whisper-pill-toggle.desktop` erzeugt und in `kglobalshortcutsrc` persistiert.

### Windows:
* Low-Level Keyboard-Hooking über das Whitelist-Paket `keyboard`:
  `keyboard.add_hotkey("windows+ctrl+p", self._toggle_callback)`

---

## 6. Dual-Clipboard Pipeline

Um unter verschiedenen Desktop-Umgebungen (Wayland, X11, Windows) maximale Zuverlässigkeit zu garantieren, implementiert `WhisperTranscriber.copy_to_clipboard()` eine duale Übergabe:
1. **Primär:** Natives `QGuiApplication.clipboard().setText(text, QClipboard.Mode.Clipboard)` (funktioniert auf Wayland direkt über das Wayland Data Device ohne externe Hilfsprogramme wie `wl-copy`).
2. **Sekundär / Fallback:** `pyperclip.copy(text)`.

---

## 7. Performance & Ressourcen

| Metrik | Prototyp (Python) | Zielgröße (Rust Phase 2) |
|---|---|---|
| **RAM (Ruhezustand)** | ~65 MB | < 15 MB |
| **RAM (Modell base int8)** | ~280 MB | ~190 MB |
| **CPU (Idle)** | 0.0 % | 0.0 % |
| **Inferenzzeit (5s Audio, i7/Ryzen)** | ~0.35 s | ~0.20 s |
| **Startup-Zeit** | ~0.6 s | < 0.05 s |
