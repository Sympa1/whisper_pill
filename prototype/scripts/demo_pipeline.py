"""End-to-End Test-Pipeline: Audio-Aufnahme -> Whisper-Inferenz -> Zwischenablage.

Dieses Skript demonstriert den vollstaendigen Kern-Datenfluss des whisper-pill Prototyps.
"""

from pathlib import Path
import sys
import time

# Projekt-Wurzelverzeichnis ermitteln und zu sys.path hinzufuegen
project_root: Path = Path(__file__).resolve().parent.parent.parent
if str(project_root) not in sys.path:
    sys.path.insert(0, str(project_root))

import numpy as np
import pyperclip

from prototype.audio.recorder import AudioRecorder
from prototype.utils.transcriber import WhisperTranscriber


def print_waveform_meter(level: float) -> None:
    """Zeigt eine dynamische Waveform-Visualisierung aehnlich dem Pill-Design.

    :param level: Normalisierter RMS-Lautstaerkepegel.
    """
    iBuffy_bars: int = int(level * 30)
    sBuffy_wave: str = "ılı" * (iBuffy_bars // 3) + "·|·" + "lıl" * (iBuffy_bars // 3)
    sys.stdout.write(f"\r  ● REC   {sBuffy_wave:<25} Pegel: {level * 100:4.1f}%")
    sys.stdout.flush()


def run_pipeline(duration_seconds: float = 4.0, model_size: str = "base") -> None:
    """Startet die Aufnahme, transkribiert die Audiodaten und kopiert das Ergebnis.

    :param duration_seconds: Laenge der Testaufnahme in Sekunden.
    :param model_size: Whisper-Modellgroesse (Standard: 'base').
    """
    print("=" * 65)
    print("  whisper-pill: Vollstaendiger lokaler Pipeline-Test")
    print("=" * 65)

    print(f"\n[1/3] Lade lokales Whisper-Modell ('{model_size}' / CPU int8)...")
    transcriber: WhisperTranscriber = WhisperTranscriber(
        model_size=model_size,
        device="cpu",
        compute_type="int8",
        default_language="de",
    )
    print("      Modell einsatzbereit!\n")

    recorder: AudioRecorder = AudioRecorder(level_callback=print_waveform_meter)

    print(f"[2/3] Starte Aufnahme fuer {duration_seconds:.1f} Sekunden.")
    print("      --> Bitte sprich jetzt deinen Testsatz ins Mikrofon!\n")

    recorder.start_recording()
    time.sleep(duration_seconds)
    audio_data: np.ndarray = recorder.stop_recording()

    print("\n\n[3/3] Audioaufnahme beendet. Starte lokale Transkription...")
    iBuffy_samples: int = len(audio_data)
    print(f"      Puffergroesse: {iBuffy_samples} Samples ({iBuffy_samples / 16000:.2f}s)")

    sBuffy_text, meta = transcriber.transcribe(audio_data)

    print("\n" + "-" * 65)
    print("  ERGEBNIS DER TRANSKRIPTION:")
    print(f"  \"{sBuffy_text}\"")
    print("-" * 65)
    print(f"  Erkannte Sprache: {meta.get('language')} (Sicherheit: {meta.get('language_probability', 0.0):.2f})")

    if sBuffy_text:
        copied: bool = WhisperTranscriber.copy_to_clipboard(sBuffy_text)
        if copied:
            sBuffy_clip: str = pyperclip.paste()
            print(f"  ✔ Text erfolgreich in Zwischenablage kopiert! (Clipboard-Inhalt: \"{sBuffy_clip}\")")
        else:
            print("  [WARNUNG] Zwischenablage konnte nicht beschrieben werden.")
    else:
        print("  (Keine Sprache oder nur Stille erkannt - bitte lauter sprechen)")

    print("=" * 65)


if __name__ == "__main__":
    duration: float = 4.0
    if len(sys.argv) > 1:
        try:
            duration = float(sys.argv[1])
        except ValueError:
            pass
    run_pipeline(duration)
