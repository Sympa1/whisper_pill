"""Unit-Tests fuer die Audiopufferung und den AudioRecorder."""

import math
from typing import List
from unittest.mock import MagicMock, patch
import numpy as np
import pytest
import sounddevice as sd

from prototype.audio.recorder import AudioRecorder


def test_audio_recorder_initialization() -> None:
    """Ueberprueft die Standard-Initialisierungsparameter fuer Whisper."""
    recorder: AudioRecorder = AudioRecorder()

    assert recorder.sample_rate == 16000
    assert recorder.channels == 1
    assert not recorder.is_recording
    assert recorder.current_level == 0.0


def test_calculate_rms_silence_and_signals() -> None:
    """Prueft die RMS-Berechnung bei Stille, leerem Signal und definiertem Sinus."""
    # Testfall 1: Leeres Signal
    empty_samples: np.ndarray = np.array([], dtype=np.float32)
    assert AudioRecorder.calculate_rms(empty_samples) == 0.0

    # Testfall 2: Vollstaendige Stille (Nullen)
    silence: np.ndarray = np.zeros(1024, dtype=np.float32)
    assert AudioRecorder.calculate_rms(silence) == 0.0

    # Testfall 3: Konstantwert 0.5
    constant_signal: np.ndarray = np.full(1024, 0.5, dtype=np.float32)
    assert math.isclose(AudioRecorder.calculate_rms(constant_signal), 0.5, rel_tol=1e-4)

    # Testfall 4: Synthetisierte Sinuswelle mit Amplitude 1.0 (theoretischer RMS: 1 / sqrt(2) approx 0.7071)
    iBuffy_sample_count: int = 16000
    time_points: np.ndarray = np.linspace(0, 1, iBuffy_sample_count, endpoint=False)
    sine_wave: np.ndarray = (np.sin(2 * np.pi * 440 * time_points)).astype(np.float32)
    expected_rms: float = 1.0 / np.sqrt(2.0)
    assert math.isclose(AudioRecorder.calculate_rms(sine_wave), expected_rms, rel_tol=1e-2)


def test_audio_callback_buffering_and_collection() -> None:
    """Verifiziert das thread-sichere Puffern von Audio-Chunks ueber den Callback."""
    collected_levels: List[float] = []

    def level_listener(level: float) -> None:
        """Sammelt gemessene Pegel fuer die Pruefung."""
        collected_levels.append(level)

    recorder: AudioRecorder = AudioRecorder(level_callback=level_listener)
    recorder._is_recording = True  # Fuer isolierten Callback-Test aktivieren

    # Simulierte Frames einspeisen
    iBuffy_chunk_size: int = 512
    dummy_flags: sd.CallbackFlags = sd.CallbackFlags()

    for iBuffy_step in range(3):
        # Erzeuge synthetischen Chunk mit steigender Amplitude
        amplitude: float = (iBuffy_step + 1) * 0.2
        chunk: np.ndarray = np.full((iBuffy_chunk_size, 1), amplitude, dtype=np.float32)
        recorder._audio_callback(
            indata=chunk,
            frames=iBuffy_chunk_size,
            time_info={},
            status=dummy_flags,
        )

    # Pruefen, ob der Level-Callback 3-mal aufgerufen wurde
    assert len(collected_levels) == 3
    assert collected_levels[-1] > collected_levels[0]

    # Daten abholen und Formate validieren
    audio_data: np.ndarray = recorder.collect_recorded_audio()

    # Gesamtlaenge: 3 * 512 = 1536 Samples
    assert audio_data.shape == (1536,)
    assert audio_data.dtype == np.float32

    # Pruefen, ob die Queue danach geleert ist
    empty_result: np.ndarray = recorder.collect_recorded_audio()
    assert empty_result.size == 0


def test_start_and_stop_recording_with_mocked_stream() -> None:
    """Testet den Lebenszyklus von start_recording und stop_recording mit Mock."""
    with patch("prototype.audio.recorder.sd.InputStream") as mock_stream_cls:
        mock_stream_instance = MagicMock()
        mock_stream_cls.return_value = mock_stream_instance

        recorder: AudioRecorder = AudioRecorder()
        recorder.start_recording()

        assert recorder.is_recording
        mock_stream_cls.assert_called_once()
        mock_stream_instance.start.assert_called_once()

        # Erneutes Starten muss einen RuntimeError ausloesen
        with pytest.raises(RuntimeError):
            recorder.start_recording()

        # Simulieren eines Chunks waehrend der Aufnahme
        dummy_chunk: np.ndarray = np.full((256, 1), 0.1, dtype=np.float32)
        recorder._audio_callback(dummy_chunk, 256, {}, sd.CallbackFlags())

        # Aufnahme beenden
        result: np.ndarray = recorder.stop_recording()

        assert not recorder.is_recording
        mock_stream_instance.stop.assert_called_once()
        mock_stream_instance.close.assert_called_once()
        assert result.shape == (256,)


def test_stop_recording_when_idle_returns_empty_array() -> None:
    """Stellt sicher, dass das Beenden einer inaktiven Aufnahme ein leeres Array liefert."""
    recorder: AudioRecorder = AudioRecorder()
    result: np.ndarray = recorder.stop_recording()

    assert isinstance(result, np.ndarray)
    assert result.size == 0
    assert result.dtype == np.float32
