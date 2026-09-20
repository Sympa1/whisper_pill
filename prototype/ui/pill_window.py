"""Schwebendes Pill-Hauptfenster fuer whisper-pill (PySide6).

Implementiert die drei Kernzustaende (Recording, Ready/Copied, Expanded/Edit)
sowie den Leerlaufzustand in einer rahmenlosen, verschiebbaren Pill-Kapsel.
"""

from typing import Optional
import numpy as np
from PySide6.QtCore import QObject, QPoint, Qt, QTimer, Signal
from PySide6.QtGui import QAction, QColor, QFont, QMouseEvent, QPainter
from PySide6.QtWidgets import (
    QApplication,
    QFrame,
    QHBoxLayout,
    QLabel,
    QMenu,
    QPushButton,
    QSizePolicy,
    QStackedWidget,
    QTextEdit,
    QVBoxLayout,
    QWidget,
)

from prototype.audio.recorder import AudioRecorder
from prototype.ui.state import PillState
from prototype.ui.transcription_worker import TranscriptionWorker
from prototype.ui.waveform_widget import WaveformWidget
from prototype.utils.transcriber import WhisperTranscriber


class AudioLevelBridge(QObject):
    """Signalschnittstelle zur thread-sicheren Uebertragung von Lautstaerkepegeln in den UI-Thread."""

    level_updated: Signal = Signal(float)


class CompactStackedWidget(QStackedWidget):
    """QStackedWidget, das seine Groessenanforderung dynamisch an das aktive Panel anpasst."""

    def sizeHint(self):
        current = self.currentWidget()
        return current.sizeHint() if current else super().sizeHint()

    def minimumSizeHint(self):
        current = self.currentWidget()
        return current.minimumSizeHint() if current else super().minimumSizeHint()


class PillWindow(QWidget):
    """Schwebende Desktop-Pill fuer Spracheingabe, Transkription und Zwischenablage."""

    # Feste einheitliche Kapsel-Dimensionen (Breite bleibt in allen Zustaenden konstant)
    PILL_WIDTH: int = 420  # Feste einheitliche Breite in Pixeln
    PILL_HEIGHT_COMPACT: int = 48  # Hoehe der kompakten Pill (Idle, Rec, Processing, Ready)
    PILL_HEIGHT_EXPANDED: int = 240  # Hoehe im ausgeklappten Editor-Zustand

    state_changed: Signal = Signal(PillState)

    def __init__(
        self,
        recorder: Optional[AudioRecorder] = None,
        transcriber: Optional[WhisperTranscriber] = None,
        parent: Optional[QWidget] = None,
    ) -> None:
        """Initialisiert das rahmenlose Pill-Fenster.

        :param recorder: Optionaler AudioRecorder (wird andernfalls neu instanziiert).
        :param transcriber: Optionaler WhisperTranscriber (wird andernfalls lazy geladen).
        :param parent: Uebergeordnetes Qt-Widget.
        """
        super().__init__(parent)

        # Thread-sichere Audio-Signal-Bruecke
        self._level_bridge: AudioLevelBridge = AudioLevelBridge()
        self._level_bridge.level_updated.connect(self._on_audio_level_received)

        # Audio- und KI-Komponenten
        self._recorder: AudioRecorder = recorder or AudioRecorder(
            level_callback=lambda lvl: self._level_bridge.level_updated.emit(lvl)
        )
        self._transcriber: Optional[WhisperTranscriber] = transcriber
        self._worker: Optional[TranscriptionWorker] = None

        # Status & Dragging
        self._current_state: PillState = PillState.IDLE
        self._drag_position: QPoint = QPoint()
        self._recording_seconds: int = 0
        self._last_transcription: str = ""

        # Timer fuer Aufnahmezeit und Auto-Reset
        self._record_timer: QTimer = QTimer(self)
        self._record_timer.setInterval(1000)
        self._record_timer.timeout.connect(self._update_record_timer)

        self._auto_hide_timer: QTimer = QTimer(self)
        self._auto_hide_timer.setSingleShot(True)
        self._auto_hide_timer.setInterval(8000)  # Nach 8s im Ready-Zustand zurueck zu Idle
        self._auto_hide_timer.timeout.connect(self._on_ready_timeout)

        # Fenster-Konfiguration (Rahmenlos, immer oben, transparenter Hintergrund)
        self.setWindowFlags(
            Qt.WindowType.FramelessWindowHint
            | Qt.WindowType.WindowStaysOnTopHint
        )
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground, True)

        self._setup_ui()
        self._apply_styles()
        self.set_state(PillState.IDLE)

    def _setup_ui(self) -> None:
        """Erstellt das Layout und die Widgets fuer alle Zustaende."""
        # Haupt-Container-Rahmen mit abgerundeter Kapselform
        self._main_layout: QVBoxLayout = QVBoxLayout(self)
        self._main_layout.setContentsMargins(0, 0, 0, 0)

        self._card_frame: QFrame = QFrame(self)
        self._card_frame.setObjectName("PillCard")
        self._card_layout: QVBoxLayout = QVBoxLayout(self._card_frame)
        self._card_layout.setContentsMargins(16, 4, 16, 4)
        self._card_layout.setSpacing(6)

        # Kompaktes StackedWidget fuer dynamische Groessenanpassung der Zustaende
        self._stack: CompactStackedWidget = CompactStackedWidget(self._card_frame)
        self._card_layout.addWidget(self._stack)
        self._main_layout.addWidget(self._card_frame)

        # 0. Idle Panel
        self._panel_idle: QWidget = self._create_idle_panel()
        self._stack.addWidget(self._panel_idle)

        # 1. Recording Panel (State 1: Aufnehmen)
        self._panel_recording: QWidget = self._create_recording_panel()
        self._stack.addWidget(self._panel_recording)

        # 2. Processing Panel (Zwischenschritt Inferenz)
        self._panel_processing: QWidget = self._create_processing_panel()
        self._stack.addWidget(self._panel_processing)

        # 3. Ready / Copied Panel (State 2: Kopiert)
        self._panel_ready: QWidget = self._create_ready_panel()
        self._stack.addWidget(self._panel_ready)

        # 4. Expanded Panel (State 3: Ausgeklappt / Edit)
        self._panel_expanded: QWidget = self._create_expanded_panel()
        self._stack.addWidget(self._panel_expanded)

    def _create_idle_panel(self) -> QWidget:
        """Erzeugt das Panel fuer den Ausgangszustand."""
        panel: QWidget = QWidget()
        layout: QHBoxLayout = QHBoxLayout(panel)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(12)

        icon_label: QLabel = QLabel("🎙 whisper-pill")
        icon_label.setObjectName("PillTitle")

        self._btn_start_record: QPushButton = QPushButton("● Aufnehmen")
        self._btn_start_record.setObjectName("RecordButton")
        self._btn_start_record.setCursor(Qt.CursorShape.PointingHandCursor)
        self._btn_start_record.clicked.connect(self.start_recording)

        layout.addWidget(icon_label)
        layout.addStretch()
        layout.addWidget(self._btn_start_record)
        return panel

    def _create_recording_panel(self) -> QWidget:
        """Erzeugt State 1: ● REC   ılılı·|·lılı   00:04."""
        panel: QWidget = QWidget()
        layout: QHBoxLayout = QHBoxLayout(panel)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(12)

        self._rec_badge: QLabel = QLabel("● REC")
        self._rec_badge.setObjectName("RecBadge")

        self._waveform: WaveformWidget = WaveformWidget()

        self._timer_label: QLabel = QLabel("00:00")
        self._timer_label.setObjectName("TimerLabel")

        self._btn_stop_record: QPushButton = QPushButton("■ Stop")
        self._btn_stop_record.setObjectName("StopButton")
        self._btn_stop_record.setCursor(Qt.CursorShape.PointingHandCursor)
        self._btn_stop_record.clicked.connect(self.stop_recording)

        layout.addWidget(self._rec_badge)
        layout.addWidget(self._waveform, 1)
        layout.addWidget(self._timer_label)
        layout.addWidget(self._btn_stop_record)
        return panel

    def _create_processing_panel(self) -> QWidget:
        """Erzeugt das Panel waehrend der Whisper-Inferenz."""
        panel: QWidget = QWidget()
        layout: QHBoxLayout = QHBoxLayout(panel)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(8)

        proc_label: QLabel = QLabel("⟳ Transkribiere Audiosignal...")
        proc_label.setObjectName("ProcessingLabel")

        layout.addWidget(proc_label)
        layout.addStretch()
        return panel

    def _create_ready_panel(self) -> QWidget:
        """Erzeugt State 2: ✔ In Zwischenablage kopiert!   [ ⤢ Details / Edit ]."""
        panel: QWidget = QWidget()
        layout: QHBoxLayout = QHBoxLayout(panel)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(14)

        success_label: QLabel = QLabel("✔ In Zwischenablage kopiert!")
        success_label.setObjectName("SuccessLabel")

        self._btn_details: QPushButton = QPushButton("⤢ Details / Edit")
        self._btn_details.setObjectName("SecondaryButton")
        self._btn_details.setCursor(Qt.CursorShape.PointingHandCursor)
        self._btn_details.clicked.connect(self.expand_details)

        btn_dismiss: QPushButton = QPushButton("✕")
        btn_dismiss.setObjectName("IconButton")
        btn_dismiss.setToolTip("Schliessen")
        btn_dismiss.clicked.connect(lambda: self.set_state(PillState.IDLE))

        layout.addWidget(success_label)
        layout.addStretch()
        layout.addWidget(self._btn_details)
        layout.addWidget(btn_dismiss)
        return panel

    def _create_expanded_panel(self) -> QWidget:
        """Erzeugt State 3: Ausgeklappt mit Texteditor und Schnellaktionen."""
        panel: QWidget = QWidget()
        layout: QVBoxLayout = QVBoxLayout(panel)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(10)

        # Kopfzeile mit Verbergen-, Beenden- und Kopier-Button
        header_layout: QHBoxLayout = QHBoxLayout()
        header_layout.setContentsMargins(0, 0, 0, 0)
        header_layout.setSpacing(8)

        btn_hide: QPushButton = QPushButton("✕ Verbergen")
        btn_hide.setObjectName("SecondaryButton")
        btn_hide.setCursor(Qt.CursorShape.PointingHandCursor)
        btn_hide.setToolTip("Pill verbergen (Super+Strg+P)")
        btn_hide.clicked.connect(self.hide)

        btn_quit: QPushButton = QPushButton("⏻ Beenden")
        btn_quit.setObjectName("QuitButton")
        btn_quit.setCursor(Qt.CursorShape.PointingHandCursor)
        btn_quit.setToolTip("whisper-pill vollstaendig beenden")
        btn_quit.clicked.connect(self.quit_application)

        btn_recopy: QPushButton = QPushButton("⎘ Text kopieren")
        btn_recopy.setObjectName("PrimaryButton")
        btn_recopy.setCursor(Qt.CursorShape.PointingHandCursor)
        btn_recopy.clicked.connect(self._recopy_edited_text)

        header_layout.addWidget(btn_hide)
        header_layout.addWidget(btn_quit)
        header_layout.addStretch()
        header_layout.addWidget(btn_recopy)
        layout.addLayout(header_layout)

        # Editor fuer transkribierten Text
        self._text_editor: QTextEdit = QTextEdit()
        self._text_editor.setObjectName("TranscribeEditor")
        self._text_editor.setMinimumHeight(120)
        self._text_editor.setPlaceholderText("Transkribierter Text erscheint hier...")
        layout.addWidget(self._text_editor)

        return panel

    def _apply_styles(self) -> None:
        """Wendet das moderne dunkle QSS-Design an."""
        self.setStyleSheet("""
            QFrame#PillCard {
                background-color: rgba(24, 24, 27, 0.95);
                border: 1px solid rgba(255, 255, 255, 0.12);
                border-radius: 24px;
            }
            QLabel {
                color: #F4F4F5;
                font-family: 'Segoe UI', 'Ubuntu', 'Helvetica Neue', sans-serif;
                font-size: 13px;
            }
            QLabel#PillTitle {
                font-weight: 600;
                color: #E4E4E7;
            }
            QLabel#RecBadge {
                color: #EF4444;
                font-weight: 700;
                letter-spacing: 0.5px;
            }
            QLabel#TimerLabel {
                color: #A1A1AA;
                font-family: monospace;
                font-size: 13px;
                font-weight: 600;
            }
            QLabel#ProcessingLabel {
                color: #38BDF8;
                font-weight: 500;
            }
            QLabel#SuccessLabel {
                color: #22C55E;
                font-weight: 600;
            }
            QPushButton {
                font-family: 'Segoe UI', 'Ubuntu', 'Helvetica Neue', sans-serif;
                font-size: 12px;
                font-weight: 600;
                padding: 6px 12px;
                border-radius: 12px;
                border: none;
            }
            QPushButton#RecordButton {
                background-color: #EF4444;
                color: white;
            }
            QPushButton#RecordButton:hover {
                background-color: #DC2626;
            }
            QPushButton#StopButton {
                background-color: #3F3F46;
                color: #FAFAFA;
            }
            QPushButton#StopButton:hover {
                background-color: #52525B;
            }
            QPushButton#PrimaryButton {
                background-color: #38BDF8;
                color: #09090B;
            }
            QPushButton#PrimaryButton:hover {
                background-color: #0EA5E9;
            }
            QPushButton#SecondaryButton {
                background-color: #27272A;
                color: #E4E4E7;
                border: 1px solid rgba(255, 255, 255, 0.1);
            }
            QPushButton#SecondaryButton:hover {
                background-color: #3F3F46;
            }
            QPushButton#QuitButton {
                background-color: rgba(239, 68, 68, 0.15);
                color: #F87171;
                border: 1px solid rgba(239, 68, 68, 0.35);
            }
            QPushButton#QuitButton:hover {
                background-color: #EF4444;
                color: #FFFFFF;
            }
            QPushButton#IconButton {
                background-color: transparent;
                color: #71717A;
                font-size: 14px;
                padding: 4px 8px;
            }
            QPushButton#IconButton:hover {
                color: #FAFAFA;
            }
            QTextEdit#TranscribeEditor {
                background-color: #18181B;
                color: #F4F4F5;
                border: 1px solid rgba(255, 255, 255, 0.12);
                border-radius: 12px;
                padding: 8px;
                font-size: 13px;
                line-height: 1.4;
            }
        """)

    @property
    def current_state(self) -> PillState:
        """Gibt den aktuellen Zustand des Pill-Fensters zurueck."""
        return self._current_state

    def set_state(self, state: PillState) -> None:
        """Schaltet das Pill-Fenster in einen neuen Zustand und passt das Layout an.

        Die Kapselbreite bleibt ueber alle Zustaende hinweg exakt konstant (PILL_WIDTH = 420 px).

        :param state: Zielzustand (PillState).
        """
        self._current_state = state

        target_widget: QWidget = self._panel_idle
        iBuffy_target_height: int = self.PILL_HEIGHT_COMPACT

        if state == PillState.IDLE:
            target_widget = self._panel_idle
            self._waveform.reset_waveform()
            self._auto_hide_timer.stop()
            iBuffy_target_height = self.PILL_HEIGHT_COMPACT

        elif state == PillState.RECORDING:
            target_widget = self._panel_recording
            self._recording_seconds = 0
            self._timer_label.setText("00:00")
            self._record_timer.start()
            self._auto_hide_timer.stop()
            iBuffy_target_height = self.PILL_HEIGHT_COMPACT

        elif state == PillState.PROCESSING:
            target_widget = self._panel_processing
            self._record_timer.stop()
            iBuffy_target_height = self.PILL_HEIGHT_COMPACT

        elif state == PillState.READY:
            target_widget = self._panel_ready
            self._record_timer.stop()
            self._auto_hide_timer.start()
            iBuffy_target_height = self.PILL_HEIGHT_COMPACT

        elif state == PillState.EXPANDED:
            target_widget = self._panel_expanded
            self._auto_hide_timer.stop()
            iBuffy_target_height = self.PILL_HEIGHT_EXPANDED

        # Nur das aktive Panel darf Groessenanforderungen stellen
        for iBuffy_idx in range(self._stack.count()):
            child_widget = self._stack.widget(iBuffy_idx)
            if child_widget == target_widget:
                child_widget.setSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
            else:
                child_widget.setSizePolicy(QSizePolicy.Policy.Ignored, QSizePolicy.Policy.Ignored)

        self._stack.setCurrentWidget(target_widget)

        # Feste Breite ueber alle Zustaende hinweg garantieren
        self.setFixedSize(self.PILL_WIDTH, iBuffy_target_height)
        self.adjustSize()
        self.state_changed.emit(state)

    def start_recording(self) -> None:
        """Startet den Audioaufnahmeprozess."""
        try:
            self._recorder.start_recording()
            self.set_state(PillState.RECORDING)
        except Exception as exc:
            print(f"[PillWindow] Fehler beim Starten der Aufnahme: {exc}")

    def stop_recording(self) -> None:
        """Beendet die Aufnahme und stoesst die Hintergrund-Transkription an."""
        audio_data: np.ndarray = self._recorder.stop_recording()
        self.set_state(PillState.PROCESSING)

        if audio_data.size == 0:
            self.set_state(PillState.IDLE)
            return

        # Lazy-Loading des Whisper-Transcribers
        if self._transcriber is None:
            self._transcriber = WhisperTranscriber(model_size="base")

        # Asynchrone Inferenz im Hintergrund ausfuehren
        self._worker = TranscriptionWorker(self._transcriber, audio_data, self)
        self._worker.finished.connect(self._on_transcription_finished)
        self._worker.failed.connect(self._on_transcription_failed)
        self._worker.start()

    def expand_details(self) -> None:
        """Oeffnet State 3 (Ausgeklappt) zur Textansicht und Bearbeitung."""
        self._text_editor.setPlainText(self._last_transcription)
        self.set_state(PillState.EXPANDED)

    def _on_transcription_finished(self, text: str, meta: dict) -> None:
        """Callback bei erfolgreicher Whisper-Transkription."""
        self._last_transcription = text
        if text:
            WhisperTranscriber.copy_to_clipboard(text)

        self._text_editor.setPlainText(text)
        self.set_state(PillState.READY)

    def _on_transcription_failed(self, error_msg: str) -> None:
        """Callback bei Inferenzfehlern."""
        print(f"[PillWindow] {error_msg}")
        self.set_state(PillState.IDLE)

    def _on_audio_level_received(self, level: float) -> None:
        """Aktualisiert die animierte Waveform im UI-Thread."""
        if self._current_state == PillState.RECORDING:
            self._waveform.set_level(level)

    def _update_record_timer(self) -> None:
        """Zaehlt die Aufnahmezeit sekundenweise hoch."""
        self._recording_seconds += 1
        iBuffy_mins: int = self._recording_seconds // 60
        iBuffy_secs: int = self._recording_seconds % 60
        sBuffy_time: str = f"{iBuffy_mins:02d}:{iBuffy_secs:02d}"
        self._timer_label.setText(sBuffy_time)

    def _on_ready_timeout(self) -> None:
        """Automatischer Rueckfall in den Leerlaufzustand nach Timeout."""
        if self._current_state == PillState.READY:
            self.set_state(PillState.IDLE)

    def _recopy_edited_text(self) -> None:
        """Kopiert den vom Nutzer im Editor modifizierten Text erneut in die Zwischenablage."""
        sBuffy_content: str = self._text_editor.toPlainText().strip()
        if sBuffy_content:
            WhisperTranscriber.copy_to_clipboard(sBuffy_content)

    # --- Drag & Drop Fensterverschiebung (Wayland- und X11-kompatibel) ---

    def mousePressEvent(self, event: QMouseEvent) -> None:
        """Erfasst den Startpunkt zum Verschieben des rahmenlosen Fensters."""
        if event.button() == Qt.MouseButton.LeftButton:
            # Unter Wayland delegiert startSystemMove das Verschieben direkt an KWin
            handle = self.windowHandle()
            if handle is not None and hasattr(handle, "startSystemMove"):
                handle.startSystemMove()
                event.accept()
                return

            self._drag_position = event.globalPosition().toPoint() - self.frameGeometry().topLeft()
            event.accept()

    def toggle_visibility(self) -> None:
        """Schaltet die Sichtbarkeit der Pill zwischen sichtbar und verborgen um."""
        if self.isVisible():
            self.hide()
        else:
            self.show()
            self.raise_()
            self.activateWindow()

    def toggle_recording(self) -> None:
        """Startet oder stoppt die Aufnahme je nach aktuellem Zustand."""
        if self._current_state == PillState.RECORDING:
            self.stop_recording()
        elif self._current_state in (PillState.IDLE, PillState.READY):
            self.start_recording()

    def quit_application(self) -> None:
        """Beendet die Audioerfassung und schliesst die Anwendung geordnet."""
        if self._current_state == PillState.RECORDING:
            try:
                self._recorder.stop_recording()
            except Exception:
                pass

        if self._worker is not None and self._worker.isRunning():
            self._worker.terminate()
            self._worker.wait(1000)

        app_instance = QApplication.instance()
        if app_instance is not None:
            app_instance.quit()

    def contextMenuEvent(self, event) -> None:
        """Oeffnet ein Kontextmenue bei Rechtsklick auf die Pill."""
        menu: QMenu = QMenu(self)

        action_toggle_rec: QAction = QAction("● Aufnahme starten / stoppen", menu)
        action_toggle_rec.triggered.connect(self.toggle_recording)
        menu.addAction(action_toggle_rec)

        action_hide: QAction = QAction("👁 Pill verbergen (Super+Strg+P)", menu)
        action_hide.triggered.connect(self.hide)
        menu.addAction(action_hide)

        menu.addSeparator()

        action_quit: QAction = QAction("⏻ whisper-pill beenden", menu)
        action_quit.triggered.connect(self.quit_application)
        menu.addAction(action_quit)

        menu.exec(event.globalPos())

