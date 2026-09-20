//! Zustandsmodell fuer die schwebende whisper-pill Kapsel.

use serde::{Deserialize, Serialize};

/// Repraesentiert die fuenf Zustaende der schwebenden Desktop-Pill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PillState {
    /// Ruhezustand am oberen Bildschirmrand (420x48 px, 0 % CPU-Last).
    Idle,

    /// Aktive Audio-Aufnahme mit animierter Waveform (420x48 px).
    Recording,

    /// Whisper-Inferenz laeuft im Hintergrund (420x48 px).
    Processing,

    /// Transkription abgeschlossen & Text im Clipboard bereit (420x48 px).
    Ready,

    /// Ausgeklappter Editor zur Ansicht und manuellen Bearbeitung (420x240 px).
    Expanded,
}

impl Default for PillState {
    fn default() -> Self {
        PillState::Idle
    }
}
