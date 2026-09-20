"""Hintergrund-Thread fuer die unterbrechungsfreie Whisper-Inferenz in Qt."""

from typing import Any, Dict
import numpy as np
from PySide6.QtCore import QThread, Signal

from prototype.utils.transcriber import WhisperTranscriber


class TranscriptionWorker(QThread):
    """Fuehrt die rechenintensive Whisper-Transkription asynchron im Hintergrund aus."""

    # Signalisiert die erfolgreiche Fertigstellung mit transkribiertem Text und Metadaten
    finished: Signal = Signal(str, dict)
    # Signalisiert Fehler waehrend der Inferenz
    failed: Signal = Signal(str)

    def __init__(
        self,
        transcriber: WhisperTranscriber,
        audio_data: np.ndarray,
        parent: Any = None,
    ) -> None:
        """Initialisiert den Worker-Thread.

        :param transcriber: Instanz des WhisperTranscribers.
        :param audio_data: 1D-NumPy-Audiodaten (16 kHz Float32).
        :param parent: Uebergeordnetes Qt-Objekt.
        """
        super().__init__(parent)
        self._transcriber: WhisperTranscriber = transcriber
        self._audio_data: np.ndarray = audio_data

    def run(self) -> None:
        """Fuehrt die Transkription im separaten Thread aus."""
        try:
            sBuffy_text, meta_info = self._transcriber.transcribe(self._audio_data)
            self.finished.emit(sBuffy_text, meta_info)
        except Exception as exc:
            sBuffy_err: str = f"Fehler bei Transkription: {exc}"
            self.failed.emit(sBuffy_err)
