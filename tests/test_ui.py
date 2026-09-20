"""Automatisierte GUI- und State-Machine-Tests fuer die Pill-UI."""

from unittest.mock import MagicMock, patch
from PySide6.QtCore import Qt
from PySide6.QtWidgets import QApplication
import pytest

from prototype.ui.pill_window import PillWindow
from prototype.ui.state import PillState
from prototype.ui.waveform_widget import WaveformWidget


@pytest.fixture(scope="session")
def qapp() -> QApplication:
    """Stellt eine globale QApplication-Instanz fuer headless Tests bereit."""
    app: QApplication | None = QApplication.instance()
    if app is None:
        app = QApplication([])
    return app


def test_waveform_widget_levels(qapp: QApplication) -> None:
    """Prueft das Setzen und Zuruecksetzen von Lautstaerke-Pegeln im WaveformWidget."""
    widget: WaveformWidget = WaveformWidget()

    # Grundzustand
    assert widget._current_level == 0.0

    # Pegel setzen
    widget.set_level(0.75)
    assert widget._current_level == 0.75

    # Zuruecksetzen
    widget.reset_waveform()
    assert widget._current_level == 0.0


def test_pill_window_flags_and_initial_state(qapp: QApplication) -> None:
    """Verifiziert die Fensterattribute (rahmenlos, immer im Vordergrund, transparent)."""
    mock_recorder = MagicMock()
    mock_transcriber = MagicMock()

    window: PillWindow = PillWindow(recorder=mock_recorder, transcriber=mock_transcriber)

    # Pruefen der geforderten Flags
    flags = window.windowFlags()
    assert flags & Qt.WindowType.FramelessWindowHint
    assert flags & Qt.WindowType.WindowStaysOnTopHint

    # Transparenz-Attribut pruefen
    assert window.testAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)

    # Initialer Zustand muss IDLE sein
    assert window.current_state == PillState.IDLE


def test_pill_window_state_transitions(qapp: QApplication) -> None:
    """Testet die Zustandsuebergaenge der Pill-UI (Recording -> Processing -> Ready -> Expanded)."""
    mock_recorder = MagicMock()
    mock_transcriber = MagicMock()

    window: PillWindow = PillWindow(recorder=mock_recorder, transcriber=mock_transcriber)
    state_history: list[PillState] = []
    window.state_changed.connect(lambda s: state_history.append(s))

    # 1. Start Aufnahme -> RECORDING
    window.start_recording()
    assert window.current_state == PillState.RECORDING
    mock_recorder.start_recording.assert_called_once()

    # 2. Stop Aufnahme -> PROCESSING
    mock_recorder.stop_recording.return_value = MagicMock(size=16000)
    with patch("prototype.ui.pill_window.TranscriptionWorker") as mock_worker_cls:
        mock_worker_instance = MagicMock()
        mock_worker_cls.return_value = mock_worker_instance

        window.stop_recording()
        assert window.current_state == PillState.PROCESSING
        mock_worker_instance.start.assert_called_once()

    # 3. Transkription beendet -> READY
    window._on_transcription_finished("Dies ist ein Testtext.", {"language": "de"})
    assert window.current_state == PillState.READY
    assert window._last_transcription == "Dies ist ein Testtext."

    # 4. Details oeffnen -> EXPANDED
    window.expand_details()
    assert window.current_state == PillState.EXPANDED
    assert window._text_editor.toPlainText() == "Dies ist ein Testtext."

    # 5. Schliessen -> Zurueck zu IDLE
    window.set_state(PillState.IDLE)
    assert window.current_state == PillState.IDLE

    # Validierung der vollstaendigen Zustandsabfolge
    assert state_history == [
        PillState.RECORDING,
        PillState.PROCESSING,
        PillState.READY,
        PillState.EXPANDED,
        PillState.IDLE,
    ]


def test_pill_recopy_edited_text(qapp: QApplication) -> None:
    """Stellt sicher, dass manuell editierter Text im Expanded-Modus neu kopiert wird."""
    mock_recorder = MagicMock()
    mock_transcriber = MagicMock()

    window: PillWindow = PillWindow(recorder=mock_recorder, transcriber=mock_transcriber)
    window.set_state(PillState.EXPANDED)
    window._text_editor.setPlainText("Korrigierter Text nach Review")

    with patch("prototype.ui.pill_window.WhisperTranscriber.copy_to_clipboard") as mock_copy:
        window._recopy_edited_text()
        mock_copy.assert_called_once_with("Korrigierter Text nach Review")


def test_pill_toggle_visibility(qapp: QApplication) -> None:
    """Testet das Ein- und Ausblenden der Pill."""
    window: PillWindow = PillWindow()
    window.show()
    assert window.isVisible()

    window.toggle_visibility()
    assert not window.isVisible()

    window.toggle_visibility()
    assert window.isVisible()


def test_pill_quit_application(qapp: QApplication) -> None:
    """Ueberprueft den geordneten Beendigungsvorgang."""
    mock_recorder = MagicMock()
    window: PillWindow = PillWindow(recorder=mock_recorder)
    window.start_recording()

    with patch.object(QApplication, "quit") as mock_quit:
        window.quit_application()
        mock_recorder.stop_recording.assert_called_once()
        mock_quit.assert_called_once()


def test_pill_tray_icon_actions(qapp: QApplication) -> None:
    """Testet die Kontextmenue-Aktionen des Tray-Icons."""
    from prototype.ui.tray_icon import PillTrayIcon

    window: PillWindow = PillWindow()
    tray: PillTrayIcon = PillTrayIcon(window)

    # Menue-Aktionen pruefen
    menu = tray.contextMenu()
    assert menu is not None
    actions = menu.actions()
    assert len(actions) >= 3

    # Toggle-Aktion via Tray
    window.show()
    tray._toggle_pill()
    assert not window.isVisible()


def test_pill_uniform_width_across_all_states(qapp: QApplication) -> None:
    """Verifiziert, dass die Pill-Kapsel in jedem Zustand exakt dieselbe feste Breite beibehaelt."""
    window: PillWindow = PillWindow()

    # Liste aller Zustaende pruefen
    for state in [
        PillState.IDLE,
        PillState.RECORDING,
        PillState.PROCESSING,
        PillState.READY,
        PillState.EXPANDED,
    ]:
        window.set_state(state)
        # Die Breite muss in jedem Zustand absolut identisch 420 px betragen
        assert window.width() == PillWindow.PILL_WIDTH
        assert window.width() == 420

        # Die Hoehe muss fuer kompakte Zustaende 48 px und fuer Expanded 240 px betragen
        if state == PillState.EXPANDED:
            assert window.height() == PillWindow.PILL_HEIGHT_EXPANDED
            assert window.height() == 240
        else:
            assert window.height() == PillWindow.PILL_HEIGHT_COMPACT
            assert window.height() == 48


def test_hotkey_manager_setup_and_signal(qapp: QApplication) -> None:
    """Testet die Registrierung und den Slot-Aufruf des HotkeyManagers."""
    from prototype.utils.hotkey import HotkeyManager

    toggle_called: bool = False

    def on_toggle() -> None:
        nonlocal toggle_called
        toggle_called = True

    mgr: HotkeyManager = HotkeyManager(on_toggle)

    # Slot-Signal direkt aufrufen
    mgr._on_kde_shortcut_pressed("whisper_pill", "toggle_pill", 123456)
    assert toggle_called is True

    # Falsche Komponente oder Aktion darf nicht triggern
    toggle_called = False
    mgr._on_kde_shortcut_pressed("other_comp", "toggle_pill", 123456)
    assert toggle_called is False




