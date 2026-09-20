"""System-Tray-Icon fuer den whisper-pill Hintergrundbetrieb."""

from typing import Optional
from PySide6.QtCore import Qt
from PySide6.QtGui import QAction, QColor, QIcon, QPainter, QPixmap
from PySide6.QtWidgets import QMenu, QSystemTrayIcon, QWidget


class PillTrayIcon(QSystemTrayIcon):
    """Verwaltet das Statussymbol im System-Tray fuer Hintergrundbetrieb und Schnellzugriff."""

    def __init__(self, pill_window: QWidget, parent: Optional[QWidget] = None) -> None:
        """Initialisiert das Tray-Icon mit Kontextmenue und Klick-Aktionen.

        :param pill_window: Referenz zum schwebenden PillWindow.
        :param parent: Uebergeordnetes Qt-Widget.
        """
        super().__init__(parent)
        self._pill_window = pill_window

        # Dynamisches Icon mit abgerundeter Kapsel und Mikrofon zeichnen
        self.setIcon(self._create_tray_icon())
        self.setToolTip("whisper-pill (Super+Strg+P)")

        self._setup_menu()

        # Linksklick auf das Tray-Icon schaltet die Sichtbarkeit um
        self.activated.connect(self._on_tray_activated)

    def _create_tray_icon(self) -> QIcon:
        """Erzeugt ein Vektor-/Pixmap-Icon fuer den System-Tray ohne externe Grafikdatei.

        :return: QIcon mit der whisper-pill Kapsel.
        """
        pixmap: QPixmap = QPixmap(64, 64)
        pixmap.fill(Qt.GlobalColor.transparent)

        painter: QPainter = QPainter(pixmap)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)

        # Dunkle Kapsel als Hintergrund
        painter.setBrush(QColor(24, 24, 27))
        painter.setPen(QColor(56, 189, 248))  # Cyan/Teal Rand
        painter.drawRoundedRect(4, 16, 56, 32, 16, 16)

        # Roter Aufnahme-Punkt in der Mitte
        painter.setBrush(QColor(239, 68, 68))
        painter.setPen(Qt.PenStyle.NoPen)
        painter.drawEllipse(24, 24, 16, 16)

        painter.end()
        return QIcon(pixmap)

    def _setup_menu(self) -> None:
        """Erstellt das Kontextmenue fuer das Tray-Icon."""
        menu: QMenu = QMenu()

        # Aktion: Ein- und Ausblenden
        self._action_toggle: QAction = QAction("🎙 Pill ein-/ausblenden (Super+Strg+P)", menu)
        self._action_toggle.triggered.connect(self._toggle_pill)
        menu.addAction(self._action_toggle)

        # Aktion: Aufnahme starten / stoppen
        self._action_record: QAction = QAction("● Aufnahme starten / stoppen", menu)
        self._action_record.triggered.connect(self._toggle_recording)
        menu.addAction(self._action_record)

        menu.addSeparator()

        # Aktion: Vollstaendig beenden
        action_quit: QAction = QAction("✕ whisper-pill beenden", menu)
        action_quit.triggered.connect(self._quit_application)
        menu.addAction(action_quit)

        self.setContextMenu(menu)

    def _on_tray_activated(self, reason: QSystemTrayIcon.ActivationReason) -> None:
        """Behandelt Klickereignisse auf das Tray-Icon."""
        if reason == QSystemTrayIcon.ActivationReason.Trigger:
            # Einfacher Linksklick: Sichtbarkeit umschalten
            self._toggle_pill()

    def _toggle_pill(self) -> None:
        """Schaltet die Sichtbarkeit des Pill-Fensters um."""
        if hasattr(self._pill_window, "toggle_visibility"):
            self._pill_window.toggle_visibility()
        else:
            if self._pill_window.isVisible():
                self._pill_window.hide()
            else:
                self._pill_window.show()

    def _toggle_recording(self) -> None:
        """Startet oder stoppt die Aufnahme je nach aktuellem Zustand."""
        if hasattr(self._pill_window, "toggle_recording"):
            self._pill_window.toggle_recording()

    def _quit_application(self) -> None:
        """Beendet die gesamte Anwendung geordnet."""
        if hasattr(self._pill_window, "quit_application"):
            self._pill_window.quit_application()
        else:
            from PySide6.QtWidgets import QApplication
            QApplication.quit()
