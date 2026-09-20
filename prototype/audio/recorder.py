"""Audio-Recorder-Modul fuer den whisper-pill Prototyp.

Dieses Modul verwaltet die lokale Erfassung von Audiosignalen ueber sounddevice
und bereitet die Audiodaten als 16-kHz-Mono-Array fuer die Whisper-Inferenz vor.
"""

import queue
from typing import Any, Callable, List, Optional
import numpy as np
import sounddevice as sd


class AudioRecorder:
    """Verwaltet die Audioaufnahme ueber die Systemsoundkarte und puffert Samples.

    Die Klasse erfasst Audiodaten im Hintergrund ueber sounddevice.InputStream,
    haelt Samples in einer threadsicheren Queue und stellt bei Aufnahmestopp
    ein zusammenhaengendes NumPy-Array zur weiteren Verarbeitung bereit.
    """

    def __init__(
        self,
        sample_rate: int = 16000,
        channels: int = 1,
        dtype: str = "float32",
        level_callback: Optional[Callable[[float], None]] = None,
    ) -> None:
        """Initialisiert den AudioRecorder mit Standard-Whisper-Parametern.

        :param sample_rate: Abtastrate in Hertz (Standard: 16000 Hz fuer Whisper).
        :param channels: Kanalanzahl (Standard: 1 fuer Mono).
        :param dtype: Datentyp fuer NumPy-Samples (Standard: float32).
        :param level_callback: Optionaler Rueckruf fuer Lautstaerke-Pegel (0.0 bis 1.0).
        """
        self._sample_rate: int = sample_rate
        self._channels: int = channels
        self._dtype: str = dtype
        self._level_callback: Optional[Callable[[float], None]] = level_callback

        # Threadsichere Queue fuer eintreffende Audio-Chunks aus dem Audio-Thread
        self._audio_queue: queue.Queue[np.ndarray] = queue.Queue()
        self._stream: Optional[sd.InputStream] = None
        self._is_recording: bool = False
        self._current_level: float = 0.0

    @property
    def is_recording(self) -> bool:
        """Gibt an, ob aktuell eine Audioaufnahme laeuft."""
        return self._is_recording

    @property
    def sample_rate(self) -> int:
        """Gibt die konfigurierte Abtastrate zurueck."""
        return self._sample_rate

    @property
    def channels(self) -> int:
        """Gibt die Anzahl der konfigurierten Audiokanaele zurueck."""
        return self._channels

    @property
    def current_level(self) -> float:
        """Gibt den zuletzt ermittelten Effektivwert (RMS) der Aufnahme zurueck."""
        return self._current_level

    def start_recording(self, device: Optional[int | str] = None) -> None:
        """Startet den Audio-Eingabestrom.

        :param device: Optionale Geraete-ID oder Geraetename fuer die Audioquelle.
        :raises RuntimeError: Falls bereits eine Aufnahme aktiv ist.
        """
        if self._is_recording:
            # Sicherheitspruefung gegen mehrfaches Starten
            raise RuntimeError("Audioaufnahme laeuft bereits.")

        # Vorherige Datenreste aus der Queue verwerfen
        while not self._audio_queue.empty():
            try:
                self._audio_queue.get_nowait()
            except queue.Empty:
                break

        self._current_level = 0.0
        self._is_recording = True

        # InputStream erzeugen und im Audio-Thread starten
        self._stream = sd.InputStream(
            samplerate=self._sample_rate,
            channels=self._channels,
            dtype=self._dtype,
            device=device,
            callback=self._audio_callback,
        )
        self._stream.start()

    def stop_recording(self) -> np.ndarray:
        """Beendet den Audio-Eingabestrom und liefert das aggregierte Audiosignal.

        :return: 1D-NumPy-Array mit Float32-Samples im Bereich [-1.0, 1.0].
        """
        if not self._is_recording:
            # Leeres Array zurueckgeben, wenn keine Aufnahme aktiv war
            return np.array([], dtype=self._dtype)

        self._is_recording = False

        if self._stream is not None:
            # Stream ordnungsgemaess anhalten und freigeben
            try:
                self._stream.stop()
                self._stream.close()
            finally:
                self._stream = None

        return self.collect_recorded_audio()

    def collect_recorded_audio(self) -> np.ndarray:
        """Liest alle verfuegbaren Chunks aus der Queue und fuehrt sie zusammen.

        :return: 1D-NumPy-Array der gesamten aufgenommenen Samples.
        """
        chunks_list: List[np.ndarray] = []

        # Alle gepufferten Chunks aus der Queue entnehmen
        while not self._audio_queue.empty():
            try:
                chunk: np.ndarray = self._audio_queue.get_nowait()
                chunks_list.append(chunk)
            except queue.Empty:
                break

        if not chunks_list:
            return np.array([], dtype=self._dtype)

        # Zusammenfuehren der Audio-Segmente
        concatenated: np.ndarray = np.concatenate(chunks_list, axis=0)

        # Falls mehrkanalig, auf 1D verflachen
        if concatenated.ndim > 1:
            concatenated = concatenated.flatten()

        return concatenated.astype(self._dtype)

    def _audio_callback(
        self,
        indata: np.ndarray,
        frames: int,
        time_info: Any,
        status: sd.CallbackFlags,
    ) -> None:
        """Callback-Methode, die kontinuierlich vom Audio-Thread aufgerufen wird.

        :param indata: Eingehender Audio-Pufferblock als NumPy-Array.
        :param frames: Anzahl der Samples im aktuellen Frame.
        :param time_info: Zeitstempel-Informationen des Treibers.
        :param status: Status-Flags fuer Pufferunter- oder -ueberlaeufe.
        """
        if not self._is_recording:
            return

        # Kopie anlegen, da indata vom Soundtreiber wiederverwendet wird
        self._audio_queue.put(indata.copy())

        # RMS-Lautstaerkepegel fuer UI-Visualisierung berechnen
        rms_level: float = self.calculate_rms(indata)
        self._current_level = rms_level

        if self._level_callback is not None:
            self._level_callback(rms_level)

    @staticmethod
    def calculate_rms(samples: np.ndarray) -> float:
        """Berechnet den Effektivwert (Root Mean Square) eines Audiosignals.

        :param samples: Audio-Samples als NumPy-Array.
        :return: RMS-Wert normalisiert im Bereich [0.0, 1.0].
        """
        if samples.size == 0:
            return 0.0

        # Quadratischer Mittelwert ueber alle Samples des Chunks
        mean_square: float = float(np.mean(np.square(samples)))
        if mean_square <= 0.0:
            return 0.0

        iBuffy_rms: float = float(np.sqrt(mean_square))
        # Auf Maximalbereich [0.0, 1.0] begrenzen
        return min(max(iBuffy_rms, 0.0), 1.0)
