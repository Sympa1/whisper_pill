"""Zustandsdefinitionen fuer die Pill-Benutzeroberflaeche."""

from enum import Enum


class PillState(Enum):
    """Repraesentiert die diskreten Zustaende des schwebenden Pill-Fensters."""

    IDLE = "idle"              # Startbereit / Ruhezustand
    RECORDING = "recording"    # State 1: Aufnehmen (REC, Waveform, Timer)
    PROCESSING = "processing"  # Inferenz laeuft (Transkription im Hintergrund)
    READY = "ready"            # State 2: Kopiert (Bestaetigung, Details-Button)
    EXPANDED = "expanded"      # State 3: Ausgeklappt (Volltextpruefung & Editieren)
