"""Whisper-Inferenz- und Transkriptionsmodul fuer den whisper-pill Prototyp.

Dieses Modul kapselt faster-whisper fuer die lokale Spracherkennung und
stellt Funktionen zum automatischen Kopieren in die System-Zwischenablage bereit.
"""

from typing import Any, Iterable, Optional, Tuple
import numpy as np
import pyperclip
from faster_whisper import WhisperModel


class WhisperTranscriber:
    """Verwaltet das lokale Whisper-Sprachmodell und fuehrt Transkriptionen aus.

    Unterstuetzt sowohl rohe NumPy-Audiodaten (16 kHz Float32) als auch Dateipfade,
    erkennt Sprache automatisch oder nutzt feste Sprachvorgaben (z. B. 'de') und
    formatiert das Ergebnis als bereinigte Zeichenkette.
    """

    def __init__(
        self,
        model_size: str = "base",
        device: str = "cpu",
        compute_type: str = "int8",
        default_language: Optional[str] = "de",
    ) -> None:
        """Initialisiert den Transcriber und laedt das Whisper-Modell.

        :param model_size: Modellgroesse ('tiny', 'base', 'small', 'medium', etc.).
        :param device: Rechengeraet ('cpu' oder 'cuda').
        :param compute_type: Quantisierungstyp (z. B. 'int8', 'float32', 'float16').
        :param default_language: Standardsprache fuer Inferenz (z. B. 'de' oder None fuer Auto-Detect).
        """
        self._model_size: str = model_size
        self._device: str = device
        self._compute_type: str = compute_type
        self._default_language: Optional[str] = default_language

        # Initialisiert das lokale CTranslate2/faster-whisper Modell
        self._model: WhisperModel = WhisperModel(
            model_size_or_path=self._model_size,
            device=self._device,
            compute_type=self._compute_type,
        )

    @property
    def model_size(self) -> str:
        """Gibt die Modellgroesse zurueck."""
        return self._model_size

    @property
    def default_language(self) -> Optional[str]:
        """Gibt die Standardsprache zurueck."""
        return self._default_language

    def transcribe(
        self,
        audio: np.ndarray | str,
        language: Optional[str] = None,
        beam_size: int = 5,
    ) -> Tuple[str, dict[str, Any]]:
        """Transkribiert uebergebene Audiodaten in Text.

        :param audio: 1D-NumPy-Array (16 kHz Mono Float32) oder Pfad zur Audiodatei.
        :param language: Optionale Sprache (ueberschreibt default_language).
        :param beam_size: Beam-Size fuer den Decoder-Suchbaum (Standard: 5).
        :return: Tupel aus dem transkribierten Gesamttext und Metadaten (Sprache, Wahrscheinlichkeit).
        """
        # Leere Eingabe abfangen
        if isinstance(audio, np.ndarray) and audio.size == 0:
            return "", {"language": None, "language_probability": 0.0}

        target_language: Optional[str] = language if language is not None else self._default_language

        segments: Iterable[Any]
        info: Any
        segments, info = self._model.transcribe(
            audio=audio,
            language=target_language,
            beam_size=beam_size,
            vad_filter=True,  # Filtert reine Stille und Hintergrundrauschen vor
        )

        # Segmente zusammenfuegen unter Nutzung der String-Buffer-Konvention
        segment_texts: list[str] = []
        for segment in segments:
            sBuffy_segment_text: str = segment.text.strip()
            if sBuffy_segment_text:
                segment_texts.append(sBuffy_segment_text)

        sBuffy_full_text: str = " ".join(segment_texts).strip()

        meta_info: dict[str, Any] = {
            "language": getattr(info, "language", target_language),
            "language_probability": getattr(info, "language_probability", 1.0),
            "duration": getattr(info, "duration", 0.0),
        }

        return sBuffy_full_text, meta_info

    @staticmethod
    def copy_to_clipboard(text: str) -> bool:
        """Kopiert den uebergebenen Text in die System-Zwischenablage.

        Nutzt primaer die native Qt-Zwischenablage (funktioniert direkt unter Wayland und X11
        ohne externe Zusatztools) und faellt bei CLI-Nutzung auf pyperclip zurueck.

        :param text: Zu kopierender Zeichenketteninhalt.
        :return: True bei erfolgreicher Zwischenablage, andernfalls False.
        """
        if not text:
            return False

        # 1. Primaer: Native Qt-Zwischenablage pruefen und nutzen (Wayland-kompatibel)
        try:
            from PySide6.QtGui import QGuiApplication
            app = QGuiApplication.instance()
            if app is not None:
                clipboard = QGuiApplication.clipboard()
                if clipboard is not None:
                    clipboard.setText(text)
                    return True
        except Exception:
            pass

        # 2. Sekundaer: pyperclip als Fallback
        try:
            pyperclip.copy(text)
            return True
        except Exception:
            return False

    @staticmethod
    def get_clipboard_text() -> str:
        """Liest den aktuellen Text aus der System-Zwischenablage aus.

        :return: Zeichenkette aus der Zwischenablage oder leer bei Fehlern.
        """
        try:
            from PySide6.QtGui import QGuiApplication
            app = QGuiApplication.instance()
            if app is not None:
                clipboard = QGuiApplication.clipboard()
                if clipboard is not None:
                    return clipboard.text()
        except Exception:
            pass

        try:
            return pyperclip.paste()
        except Exception:
            return ""

