"""Waveform-Visualisierungs-Widget fuer die Pill-UI."""

import math
from typing import List
from PySide6.QtCore import Qt, QTimer
from PySide6.QtGui import QColor, QPainter, QPaintEvent
from PySide6.QtWidgets import QWidget


class WaveformWidget(QWidget):
    """Zeichnet animierte Lautstaerke-Balken (Waveform) basierend auf Audiosignalen."""

    def __init__(self, parent: QWidget | None = None, bar_count: int = 9) -> None:
        """Initialisiert das Waveform-Widget mit Standard-Balkenanzahl.

        :param parent: Uebergeordnetes Qt-Widget.
        :param bar_count: Anzahl der symmetrischen Visualisierungsbalken.
        """
        super().__init__(parent)
        self._bar_count: int = bar_count
        self._current_level: float = 0.0
        self._bar_heights: List[float] = [0.1] * bar_count

        # Timer fuer weiche Rueckstell-Animation bei Pausen (startet nur bei Bedarf)
        self._decay_timer: QTimer = QTimer(self)
        self._decay_timer.setInterval(40)  # 25 fps
        self._decay_timer.timeout.connect(self._decay_step)

        self.setMinimumWidth(80)
        self.setFixedHeight(24)

    def set_level(self, level: float) -> None:
        """Setzt den aktuellen Lautstaerkepegel und aktualisiert die Balken.

        :param level: Normalisierter RMS-Pegelwert im Bereich [0.0, 1.0].
        """
        self._current_level = min(max(level, 0.0), 1.0)
        center_index: float = (self._bar_count - 1) / 2.0

        for iBuffy_idx in range(self._bar_count):
            # Gaussaehnliche Gewichtung zur Mitte hin fuer das charakteristische ılılı·|·lılı Aussehen
            distance_from_center: float = abs(iBuffy_idx - center_index)
            factor: float = math.exp(-0.4 * distance_from_center)
            target_height: float = max(0.12, min(self._current_level * factor * 1.8, 1.0))

            # Sanftes Ansteigen
            if target_height > self._bar_heights[iBuffy_idx]:
                self._bar_heights[iBuffy_idx] = target_height

        if not self._decay_timer.isActive():
            self._decay_timer.start()

        self.update()

    def reset_waveform(self) -> None:
        """Setzt alle Balken auf die minimale Grundhoehe zurueck."""
        self._current_level = 0.0
        self._bar_heights = [0.12] * self._bar_count
        self._decay_timer.stop()
        self.update()

    def _decay_step(self) -> None:
        """Daempft die Balkenhoehen schrittweise fuer fluessige Animation."""
        needs_update: bool = False
        for iBuffy_idx in range(self._bar_count):
            if self._bar_heights[iBuffy_idx] > 0.12:
                self._bar_heights[iBuffy_idx] = max(0.12, self._bar_heights[iBuffy_idx] * 0.88)
                needs_update = True

        if needs_update:
            self.update()
        else:
            # Wenn alle Balken auf Grundniveau sind, Timer bis zum naechsten Sample anhalten
            self._decay_timer.stop()

    def paintEvent(self, event: QPaintEvent) -> None:
        """Zeichnet die Waveform-Balken via QPainter.

        :param event: Qt-PaintEvent-Objekt.
        """
        painter: QPainter = QPainter(self)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)

        widget_width: int = self.width()
        widget_height: int = self.height()
        bar_width: float = 4.0
        spacing: float = 4.0
        total_width: float = self._bar_count * bar_width + (self._bar_count - 1) * spacing
        start_x: float = (widget_width - total_width) / 2.0

        # Aktive Akzentfarbe (Modernes Cyan/Teal fuer Audioimpulse)
        bar_color: QColor = QColor(56, 189, 248)
        painter.setBrush(bar_color)
        painter.setPen(Qt.PenStyle.NoPen)

        for iBuffy_bar in range(self._bar_count):
            h_factor: float = self._bar_heights[iBuffy_bar]
            current_bar_height: float = max(4.0, widget_height * h_factor)
            x_pos: float = start_x + iBuffy_bar * (bar_width + spacing)
            y_pos: float = (widget_height - current_bar_height) / 2.0

            # Abgerundete Rechtecke zeichnen
            painter.drawRoundedRect(
                int(x_pos),
                int(y_pos),
                int(bar_width),
                int(current_bar_height),
                2,
                2,
            )

        painter.end()
