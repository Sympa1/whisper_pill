//! Zwischenablage-Steuerung fuer Desktop-Betriebssysteme (Linux & Windows).
//!
//! Nutzt die arboard-Bibliothek zum Setzen und Lesen des Textes in der System-Zwischenablage.

use arboard::Clipboard;
use std::sync::Mutex;

/// Verwaltet den sicheren Zugriff auf die System-Zwischenablage.
pub struct ClipboardManager {
    /// Mutex-geschuetzte Zwischenablage-Instanz.
    clipboard: Mutex<Option<Clipboard>>,
}

impl ClipboardManager {
    /// Erstellt eine neue Instanz des Zwischenablage-Managers.
    pub fn new() -> Self {
        let cb = Clipboard::new().ok();
        Self {
            clipboard: Mutex::new(cb),
        }
    }

    /// Schreibt den uebergebenen Text in die System-Zwischenablage.
    pub fn set_text(&self, text: &str) -> Result<(), String> {
        let mut guard = self.clipboard.lock().map_err(|e| e.to_string())?;

        // Falls noch keine Instanz vorhanden war, erneut versuchen zu initialisieren
        if guard.is_none() {
            *guard = Clipboard::new().ok();
        }

        let s_buffy_text = text.to_string();
        if let Some(ref mut cb) = *guard {
            cb.set_text(s_buffy_text)
                .map_err(|e| format!("Fehler beim Kopieren in die Zwischenablage: {}", e))?;
            Ok(())
        } else {
            Err("System-Zwischenablage konnte nicht geoeffnet werden.".to_string())
        }
    }

    /// Liest den aktuellen Text aus der System-Zwischenablage.
    pub fn get_text(&self) -> Result<String, String> {
        let mut guard = self.clipboard.lock().map_err(|e| e.to_string())?;

        if guard.is_none() {
            *guard = Clipboard::new().ok();
        }

        if let Some(ref mut cb) = *guard {
            let s_buffy_content = cb
                .get_text()
                .map_err(|e| format!("Fehler beim Lesen der Zwischenablage: {}", e))?;
            Ok(s_buffy_content)
        } else {
            Err("System-Zwischenablage konnte nicht geoeffnet werden.".to_string())
        }
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_initialization() {
        let manager = ClipboardManager::new();
        // Unter Headless-CI kann Clipboard fehlschlagen, der Manager darf aber nicht panicken
        let _ = manager.get_text();
    }
}
