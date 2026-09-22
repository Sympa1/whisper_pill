"""Haupteinstiegspunkt fuer die whisper-pill Prototyp-Anwendung."""
from pathlib import Path
import sys

# Projekt-Wurzelverzeichnis ermitteln und zu sys.path hinzufuegen
project_root: Path = Path(__file__).resolve().parent.parent
if str(project_root) not in sys.path:
    sys.path.insert(0, str(project_root))

import signal
from PySide6.QtCore import Qt, QTimer
from PySide6.QtGui import QScreen
from PySide6.QtWidgets import QApplication

from prototype.ui.pill_window import PillWindow
from prototype.ui.tray_icon import PillTrayIcon
from prototype.utils.hotkey import HotkeyManager


def main() -> None:
    """Startet die grafische Pill-Anwendung im Hintergrundbetrieb mit Tray-Icon und Hotkeys."""
    # 1. Pruefen, ob bereits eine Instanz laeuft -> Umschalten statt Doppelstart
    if "--toggle" in sys.argv or HotkeyManager.send_toggle_to_running_instance():
        if "--toggle" not in sys.argv:
            print("[whisper-pill] Eine Instanz laeuft bereits - Sichtbarkeit umgeschaltet.")
        sys.exit(0)

    app: QApplication = QApplication(sys.argv)
    app.setApplicationName("whisper-pill")
    app.setOrganizationName("whisper-pill")
    # Wichtig: App laeuft im Hintergrund weiter, auch wenn das Pill-Fenster ausgeblendet ist
    app.setQuitOnLastWindowClosed(False)

    # Sauberes Beenden ueber Strg+C im Terminal (ohne Python-Traceback)
    signal.signal(signal.SIGINT, lambda *_: app.quit())
    sigint_timer: QTimer = QTimer()
    sigint_timer.setInterval(300)
    sigint_timer.timeout.connect(lambda: None)
    sigint_timer.start()

    window: PillWindow = PillWindow()

    # System-Tray-Icon fuer Hintergrundbetrieb initialisieren
    tray_icon: PillTrayIcon = PillTrayIcon(window)
    tray_icon.show()

    # Globalen Hotkey (Super+Strg+P unter Linux, Win+Strg+P unter Windows) registrieren
    hotkey_mgr: HotkeyManager = HotkeyManager(window.toggle_visibility, parent=window)
    hotkey_mgr.setup()

    # Zentriert am oberen Bildschirmrand platzieren (typisches Pill-Dock-Verhalten)
    primary_screen: QScreen = app.primaryScreen()
    if primary_screen is not None:
        screen_geo = primary_screen.geometry()
        x_pos: int = (screen_geo.width() - window.width()) // 2
        y_pos: int = 40  # 40 Pixel vom oberen Rand
        window.move(x_pos, y_pos)

    window.show()
    window.raise_()
    window.activateWindow()

    print("=" * 65)
    print("  whisper-pill laeuft im Hintergrund!")
    print("  - Globaler Shortcut: Super+Strg+P (Linux) / Win+Strg+P (Windows)")
    print("  - System-Tray: Icon in der Taskleiste (Rechtsklick fuer Menue)")
    print("  - Schliessen / Verbergen: Versteckt die Pill im Hintergrund")
    print("  - Beenden: Im Menue 'Beenden' oder '⏻ Beenden' im Editor waehlen")
    print("=" * 65)

    try:
        sys.exit(app.exec())
    except KeyboardInterrupt:
        print("\nwhisper-pill sauber beendet.")
        sys.exit(0)


if __name__ == "__main__":
    main()
