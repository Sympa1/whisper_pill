"""Unit-Tests fuer die Whisper-Inferenz und Zwischenablagen-Funktionalitaet."""

from unittest.mock import MagicMock, patch
import numpy as np
import pytest

from prototype.utils.transcriber import WhisperTranscriber


def test_copy_to_clipboard() -> None:
    """Ueberprueft das Kopieren in die Zwischenablage."""
    with patch("prototype.utils.transcriber.pyperclip.copy") as mock_copy:
        # Leerer Text sollte nicht kopiert werden
        assert not WhisperTranscriber.copy_to_clipboard("")
        mock_copy.assert_not_called()

        # Gueltiger Text wird weitergeleitet
        sBuffy_payload: str = "Test-Transkription fuer whisper-pill"
        success: bool = WhisperTranscriber.copy_to_clipboard(sBuffy_payload)

        assert success
        mock_copy.assert_called_once_with(sBuffy_payload)


def test_copy_to_clipboard_with_qt_active() -> None:
    """Ueberprueft, dass bei aktiver Qt-App das native Qt-Clipboard bevorzugt wird."""
    mock_app = MagicMock()
    mock_clipboard = MagicMock()

    with patch("PySide6.QtGui.QGuiApplication.instance", return_value=mock_app):
        with patch("PySide6.QtGui.QGuiApplication.clipboard", return_value=mock_clipboard):
            sBuffy_qt_text: str = "Text ueber Qt Clipboard"
            success: bool = WhisperTranscriber.copy_to_clipboard(sBuffy_qt_text)

            assert success
            mock_clipboard.setText.assert_called_once_with(sBuffy_qt_text)



def test_transcribe_empty_audio() -> None:
    """Stellt sicher, dass leere Audiodaten direkt als leerer Text quittiert werden."""
    with patch("prototype.utils.transcriber.WhisperModel"):
        transcriber: WhisperTranscriber = WhisperTranscriber(model_size="tiny")
        text, meta = transcriber.transcribe(np.array([], dtype=np.float32))

        assert text == ""
        assert meta["language"] is None


def test_transcribe_with_mocked_model() -> None:
    """Verifiziert die Aggregation von Segment-Texten aus dem Whisper-Modell."""
    with patch("prototype.utils.transcriber.WhisperModel") as mock_model_cls:
        mock_model_instance = MagicMock()
        mock_model_cls.return_value = mock_model_instance

        # Mock-Segmente definieren
        segment_1 = MagicMock()
        segment_1.text = " Hallo Welt,"
        segment_2 = MagicMock()
        segment_2.text = " dies ist ein lokaler Test. "

        mock_info = MagicMock()
        mock_info.language = "de"
        mock_info.language_probability = 0.98
        mock_info.duration = 2.5

        mock_model_instance.transcribe.return_value = ([segment_1, segment_2], mock_info)

        transcriber: WhisperTranscriber = WhisperTranscriber(model_size="tiny")
        dummy_audio: np.ndarray = np.zeros(16000, dtype=np.float32)

        sBuffy_result, meta = transcriber.transcribe(dummy_audio)

        assert sBuffy_result == "Hallo Welt, dies ist ein lokaler Test."
        assert meta["language"] == "de"
        assert meta["language_probability"] == 0.98


def test_transcription_worker_ram_audio_flushing() -> None:
    """Verifiziert, dass Audiodaten im RAM nach Inferenzabschluss aus Datenschutzgruenden genullt werden."""
    from prototype.ui.transcription_worker import TranscriptionWorker

    mock_transcriber = MagicMock()
    mock_transcriber.transcribe.return_value = ("Ergebnis", {"language": "de"})

    # Array mit Werten initialisieren
    audio_data: np.ndarray = np.ones(1600, dtype=np.float32)
    assert np.any(audio_data != 0.0)

    worker: TranscriptionWorker = TranscriptionWorker(mock_transcriber, audio_data)
    worker.run()

    # Nach run() muss das Array vollstaendig genullt sein
    assert np.all(audio_data == 0.0)

