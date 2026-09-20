"""Automatisiertes Erfassen aller Pill-Zustaende als hochaufloesende Screenshots fuer die Dokumentation."""

import os
from pathlib import Path
import sys

# Repository-Wurzelverzeichnis zum Suchpfad hinzufuegen
repo_root: Path = Path(__file__).resolve().parent.parent.parent
if str(repo_root) not in sys.path:
    sys.path.insert(0, str(repo_root))

from PySide6.QtWidgets import QApplication
from prototype.ui.pill_window import PillWindow
from prototype.ui.state import PillState


def capture_all_states() -> None:
    """Rendert und speichert alle Zustaende der Pill-UI fuer technische und Benutzer-Dokumentation."""
    # Zielverzeichnis fuer Dokumentations-Assets anlegen
    repo_root: Path = Path(__file__).resolve().parent.parent.parent
    assets_dir: Path = repo_root / "docs" / "assets"
    assets_dir.mkdir(parents=True, exist_ok=True)

    # Headless Qt-Applikation initialisieren
    app: QApplication = QApplication.instance() or QApplication([])

    window: PillWindow = PillWindow()
    window.show()

    # 0. Idle-Zustand (Bereit zur Aufnahme)
    window.set_state(PillState.IDLE)
    app.processEvents()
    window.grab().save(str(assets_dir / "state_0_idle.png"))

    # 1. Recording-Zustand (Aufnahme laeuft mit Waveform)
    window.set_state(PillState.RECORDING)
    window._waveform.set_level(0.72)
    window._timer_label.setText("00:04")
    app.processEvents()
    window.grab().save(str(assets_dir / "state_1_recording.png"))

    # 2. Ready-Zustand (In Zwischenablage kopiert)
    window.set_state(PillState.READY)
    app.processEvents()
    window.grab().save(str(assets_dir / "state_2_ready.png"))

    # 3. Expanded-Zustand (Text pruefen & editieren)
    sBuffy_sample_text: str = (
        "Dies ist ein erfolgreich transkribierter Diktat-Text.\n"
        "Der Text liegt automatisch in der Zwischenablage bereit "
        "und kann hier vor dem Einfuegen geprueft werden."
    )
    window._last_transcription = sBuffy_sample_text
    window.expand_details()
    app.processEvents()
    window.grab().save(str(assets_dir / "state_3_expanded.png"))

    print(f"[Screenshot-Generator] 4 Zustaende erfolgreich nach {assets_dir} exportiert.")


if __name__ == "__main__":
    capture_all_states()
