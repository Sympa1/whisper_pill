//! Plattform-Subsystem fuer whisper-pill.
//!
//! Enthaelt Module fuer Zwischenablage (arboard) und globale Hotkeys (D-Bus / KGlobalAccel).

pub mod clipboard;
pub mod hotkey;
#[cfg(target_os = "linux")]
pub mod tray;

