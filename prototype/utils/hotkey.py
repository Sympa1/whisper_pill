"""Globales Hotkey- und Single-Instance-Management fuer Linux und Windows.

Ermoeglicht das globale Ein- und Ausblenden der schwebenden Pill ueber
Super+Strg+P (Linux) bzw. Windows-Taste+Strg+P (Windows) im Hintergrundbetrieb.
"""

import os
from pathlib import Path
import subprocess
import sys
from typing import Callable, Optional
from PySide6.QtCore import QObject, Slot


class HotkeyManager(QObject):
    """Verwaltet globale System-Hotkeys plattformuebergreifend fuer Linux und Windows."""

    def __init__(self, toggle_callback: Callable[[], None], parent: Optional[QObject] = None) -> None:
        """Initialisiert den Hotkey-Manager mit der Umschaltfunktion.

        :param toggle_callback: Funktion zum Ein-/Ausblenden der Pill.
        :param parent: Uebergeordnetes Qt-Objekt.
        """
        super().__init__(parent)
        self._toggle_callback: Callable[[], None] = toggle_callback
        self._is_registered: bool = False

    def setup(self) -> bool:
        """Registriert den globalen Hotkey abhaengig vom Betriebssystem.

        :return: True bei erfolgreicher Registrierung.
        """
        if sys.platform == "win32":
            return self._setup_windows()
        elif sys.platform.startswith("linux"):
            return self._setup_linux()
        return False

    def _setup_windows(self) -> bool:
        """Registriert Windows+Strg+P auf Windows-Systemen via keyboard."""
        try:
            import keyboard
            keyboard.add_hotkey("windows+ctrl+p", self._toggle_callback)
            self._is_registered = True
            return True
        except Exception as exc:
            print(f"[HotkeyManager] Windows-Hotkey konnte nicht registriert werden: {exc}")
            return False

    def _setup_linux(self) -> bool:
        """Richtet D-Bus-Service und KDE/Desktop-Shortcut fuer Super+Strg+P (Meta+Ctrl+P) ein."""
        dbus_ready: bool = self._register_dbus_service()

        # Nativen KDE KGlobalAccel Hotkey-Listener fuer Wayland anbinden
        self._register_kglobalaccel()

        # KDE Plasma / XDG Shortcut-Konfiguration fuer Super+Strg+P anlegen
        self._setup_kde_shortcut()
        return dbus_ready

    def _register_kglobalaccel(self) -> bool:
        """Registriert whisper-pill direkt im KDE KGlobalAccel-Dienst und lauscht auf Tastendruecke."""
        try:
            from PySide6.QtDBus import QDBusConnection, QDBusInterface

            session_bus = QDBusConnection.sessionBus()
            kglobal_iface = QDBusInterface(
                "org.kde.kglobalaccel",
                "/kglobalaccel",
                "org.kde.KGlobalAccel",
                session_bus,
            )

            if not kglobal_iface.isValid():
                return False

            action_id: list[str] = [
                "whisper_pill",
                "toggle_pill",
                "whisper-pill",
                "Pill ein-/ausblenden",
            ]
            kglobal_iface.call("doRegister", action_id)

            # Qt-Keycode fuer Meta+Ctrl+P: 0x10000000 | 0x04000000 | 0x50 = 335544400
            iBuffy_keycode: int = 335544400

            # Shortcut in KGlobalAccel via gdbus aktiv schalten
            subprocess.run(
                [
                    "gdbus",
                    "call",
                    "--session",
                    "--dest",
                    "org.kde.kglobalaccel",
                    "--object-path",
                    "/kglobalaccel",
                    "--method",
                    "org.kde.KGlobalAccel.setShortcut",
                    str(action_id),
                    f"[{iBuffy_keycode}]",
                    "uint32 2",
                ],
                capture_output=True,
                check=False,
            )

            # In kglobalshortcutsrc fuer Sitzungsneustarts persistieren
            for binary in ["kwriteconfig6", "kwriteconfig5"]:
                try:
                    res = subprocess.run(["which", binary], capture_output=True, text=True)
                    if res.returncode == 0:
                        kwrite_path = res.stdout.strip()
                        subprocess.run(
                            [
                                kwrite_path,
                                "--file",
                                "kglobalshortcutsrc",
                                "--group",
                                "whisper_pill",
                                "--key",
                                "_k_friendly_name",
                                "whisper-pill",
                            ],
                            check=False,
                        )
                        subprocess.run(
                            [
                                kwrite_path,
                                "--file",
                                "kglobalshortcutsrc",
                                "--group",
                                "whisper_pill",
                                "--key",
                                "toggle_pill",
                                "Meta+Ctrl+P,Meta+Ctrl+P,Pill ein-/ausblenden",
                            ],
                            check=False,
                        )
                        break
                except Exception:
                    pass

            # Direkt an das D-Bus Signal globalShortcutPressed von KGlobalAccel koppeln
            session_bus.connect(
                "org.kde.kglobalaccel",
                "/component/whisper_pill",
                "org.kde.kglobalaccel.Component",
                "globalShortcutPressed",
                self,
                "1_on_kde_shortcut_pressed(QString,QString,qlonglong)",
            )
            return True
        except Exception as exc:
            print(f"[HotkeyManager] KGlobalAccel-Registrierung fehlgeschlagen: {exc}")
            return False

    @Slot(str, str, "qlonglong")
    def _on_kde_shortcut_pressed(self, comp: str, action: str, timestamp: int) -> None:
        """Empfaengt das globale Shortcut-Signal von KDE Plasma und schaltet die Pill um."""
        if comp == "whisper_pill" and action == "toggle_pill":
            self._toggle_callback()

    def _register_dbus_service(self) -> bool:
        """Registriert das Objekt /Pill und den Service org.whisper_pill.Pill auf dem Session-Bus."""
        try:
            from PySide6.QtDBus import QDBusAbstractAdaptor, QDBusConnection

            class PillDBusAdaptor(QDBusAbstractAdaptor):
                """D-Bus-Adapter zur Entgegennahme externer Steuerbefehle."""

                def __init__(self, parent_obj: QObject, callback: Callable[[], None]) -> None:
                    super().__init__(parent_obj)
                    self._callback = callback

                @Slot()
                def toggle(self) -> None:
                    """Schaltet die Sichtbarkeit der Pill um."""
                    self._callback()

            # Adaptor an den HotkeyManager binden
            self._adaptor = PillDBusAdaptor(self, self._toggle_callback)
            session_bus = QDBusConnection.sessionBus()

            obj_registered: bool = session_bus.registerObject("/Pill", self)
            srv_registered: bool = session_bus.registerService("org.whisper_pill.Pill")

            return obj_registered and srv_registered
        except Exception as exc:
            print(f"[HotkeyManager] D-Bus-Registrierung fehlgeschlagen: {exc}")
            return False

    @staticmethod
    def _setup_kde_shortcut() -> None:
        """Erzeugt Desktop-Eintrag und hinterlegt Meta+Ctrl+P (Super+Strg+P) in KDE Plasma Shortcuts."""
        try:
            app_dir: Path = Path.home() / ".local" / "share" / "applications"
            app_dir.mkdir(parents=True, exist_ok=True)
            desktop_file: Path = app_dir / "whisper-pill-toggle.desktop"

            # Absoluter Pfad zum Python-Interpreter und zur main.py
            repo_root: Path = Path(__file__).resolve().parent.parent.parent
            python_bin: Path = repo_root / "prototype" / ".venv" / "bin" / "python"
            main_script: Path = repo_root / "prototype" / "main.py"

            desktop_content: str = f"""[Desktop Entry]
Name=whisper-pill Toggle
Comment=whisper-pill ein- und ausblenden
Exec={python_bin} {main_script} --toggle
Icon=audio-input-microphone
Type=Application
Terminal=false
Categories=Utility;
"""
            desktop_file.write_text(desktop_content, encoding="utf-8")
            desktop_file.chmod(0o755)

            # XDG Desktop-Datenbank aktualisieren
            subprocess.run(["update-desktop-database", str(app_dir)], capture_output=True, check=False)

            # In KDE Plasma Shortcut-Registry schreiben (Meta+Ctrl+P = Super+Strg+P)
            kwrite_path: Optional[str] = None
            for binary in ["kwriteconfig6", "kwriteconfig5"]:
                try:
                    res = subprocess.run(["which", binary], capture_output=True, text=True)
                    if res.returncode == 0:
                        kwrite_path = res.stdout.strip()
                        break
                except Exception:
                    pass

            if kwrite_path:
                subprocess.run(
                    [
                        kwrite_path,
                        "--file",
                        "kglobalshortcutsrc",
                        "--group",
                        "services",
                        "--group",
                        "whisper-pill-toggle.desktop",
                        "--key",
                        "_launch",
                        "Meta+Ctrl+P,none,whisper-pill Toggle",
                    ],
                    check=False,
                )
        except Exception as exc:
            print(f"[HotkeyManager] KDE-Shortcut konnte nicht eingerichtet werden: {exc}")

    @staticmethod
    def send_toggle_to_running_instance() -> bool:
        """Prueft, ob bereits eine Instanz laeuft, und sendet den Umschaltbefehl via D-Bus.

        :return: True, wenn eine laufende Instanz erfolgreich benachrichtigt wurde.
        """
        if sys.platform.startswith("linux"):
            try:
                from PySide6.QtDBus import QDBusConnection, QDBusInterface
                session_bus = QDBusConnection.sessionBus()
                iface = QDBusInterface("org.whisper_pill.Pill", "/Pill", "", session_bus)
                if iface.isValid():
                    iface.call("toggle")
                    return True
            except Exception:
                pass
        return False
