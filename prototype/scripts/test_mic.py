"""Live-Mikrofontest fuer den AudioRecorder.

Dieses Skript nimmt 3 Sekunden Audio ueber das Standard-Mikrofon auf,
zeigt einen Live-Pegelausschlag im Terminal an und speichert das Ergebnis
als WAV-Datei zur Qualitaetspruefung.
"""

from pathlib import Path
import sys
import time

# Projekt-Wurzelverzeichnis ermitteln und zu sys.path hinzufuegen
project_root: Path = Path(__file__).resolve().parent.parent.parent
if str(project_root) not in sys.path:
    sys.path.insert(0, str(project_root))

import numpy as np
from scipy.io import wavfile

from prototype.audio.recorder import AudioRecorder


def print_level_bar(level: float) -> None:
    """Gibt einen einfachen ASCII-Pegelbalken auf stdout aus.

    :param level: Normalisierter RMS-Pegel (0.0 bis 1.0).
    """
    iBuffy_bars: int = int(level * 40)
    sBuffy_bar: str = "█" * iBuffy_bars + "░" * (40 - iBuffy_bars)
    sys.stdout.write(f"\rPegel: [{sBuffy_bar}] {level * 100:5.1f} %")
    sys.stdout.flush()


def run_microphone_test(record_seconds: float = 3.0) -> None:
    """Fuehrt einen zeitgesteuerten Audioaufnahmetest durch.

    :param record_seconds: Aufnahmedauer in Sekunden.
    """
    sBuffy_info: str = f"Initialisiere AudioRecorder (16 kHz, Mono) fuer {record_seconds}s Test..."
    print(sBuffy_info)

    recorder: AudioRecorder = AudioRecorder(level_callback=print_level_bar)

    print("\nAufnahme startet JETZT! Bitte kurz sprechen oder Geraeusche machen...\n")
    recorder.start_recording()

    time.sleep(record_seconds)

    audio_samples: np.ndarray = recorder.stop_recording()
    print("\n\nAufnahme beendet.")

    # Kennzahlen auswerten
    iBuffy_sample_count: int = len(audio_samples)
    duration: float = iBuffy_sample_count / recorder.sample_rate if recorder.sample_rate > 0 else 0.0
    peak_value: float = float(np.max(np.abs(audio_samples))) if iBuffy_sample_count > 0 else 0.0
    rms_value: float = recorder.calculate_rms(audio_samples)

    print("\n--- Aufnahme-Statistik ---")
    print(f"Samples erfasst: {iBuffy_sample_count}")
    print(f"Dauer:          {duration:.2f} Sekunden")
    print(f"Spitzenwert:    {peak_value:.4f}")
    print(f"Gesamt-RMS:     {rms_value:.4f}")

    if iBuffy_sample_count == 0:
        print("\n[FEHLER] Es wurden keine Audiodaten aufgezeichnet.")
        return

    # Speichern als WAV-Datei fuer die akustische Kontrolle
    output_dir: Path = Path("prototype/output")
    output_dir.mkdir(parents=True, exist_ok=True)
    wav_path: Path = output_dir / "mic_test.wav"

    # Konvertierung fuer Standard-PCM-16bit WAV
    scaled_samples: np.ndarray = (audio_samples * 32767).astype(np.int16)
    wavfile.write(str(wav_path), recorder.sample_rate, scaled_samples)
    print(f"Testaufnahme gespeichert unter: {wav_path}")


if __name__ == "__main__":
    run_microphone_test(3.0)
