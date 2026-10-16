//! Autostart-Verwaltung nach XDG Desktop Application Autostart Specification.
//!
//! Stellt sicher, dass whisper-pill unter KDE Plasma (ab 6.0), GNOME (ab 45),
//! XFCE sowie weiteren Freedesktop-konformen Desktop-Umgebungen beim Systemstart
//! automatisch ausgefuehrt werden kann.

use std::fs;
use std::path::PathBuf;

/// Liefert den Pfad zur .desktop-Autostart-Datei im Benutzerverzeichnis.
pub fn autostart_desktop_path() -> PathBuf {
    if let Ok(s_buffy_xdg) = std::env::var("XDG_CONFIG_HOME")
        && !s_buffy_xdg.is_empty()
    {
        return PathBuf::from(s_buffy_xdg).join("autostart").join("whisper-pill.desktop");
    }
    let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(s_buffy_home).join(".config").join("autostart").join("whisper-pill.desktop")
}

/// Prueft, ob der Autostart fuer whisper-pill aktuell eingerichtet und aktiv ist.
pub fn is_autostart_enabled() -> bool {
    let path = autostart_desktop_path();
    if !path.exists() {
        return false;
    }
    // Pruefen, ob die Datei nicht explizit deaktiviert wurde
    if let Ok(s_buffy_content) = fs::read_to_string(&path) {
        if s_buffy_content.contains("Hidden=true") || s_buffy_content.contains("X-GNOME-Autostart-enabled=false") {
            return false;
        }
        // Validierung des Exec-Pfades: Falls der Pfad auf ein nicht existierendes Mount-Verzeichnis zeigt
        for line in s_buffy_content.lines() {
            if let Some(s_exec) = line.strip_prefix("Exec=") {
                let s_clean = s_exec.trim().trim_matches('"');
                if !s_clean.is_empty() && !std::path::Path::new(s_clean).exists() {
                    return false;
                }
            }
        }
        return true;
    }
    false
}

/// Ermittelt den tatsaechlich persistenten Ausfuehrungspfad der Anwendung.
/// Beruecksichtigt primaer AppImage-Umgebungen ($APPIMAGE), damit der Autostart
/// nach einem Systemneustart nicht auf temporaere /tmp/.mount_*-Verzeichnisse verweist.
pub fn get_persistent_executable_path() -> Result<String, std::io::Error> {
    if let Ok(s_buffy_appimage) = std::env::var("APPIMAGE") {
        let s_trimmed = s_buffy_appimage.trim();
        if !s_trimmed.is_empty() {
            let path = std::path::Path::new(s_trimmed);
            if path.exists() {
                return Ok(s_trimmed.to_string());
            }
        }
    }
    let current = std::env::current_exe()?;
    Ok(current.to_string_lossy().to_string())
}

/// Erzeugt den standardkonformen .desktop-Eintrag fuer KDE Plasma (6+), GNOME (45+) und XFCE.
pub fn generate_desktop_entry(s_buffy_exe_path: &str) -> String {
    format!(
        "[Desktop Entry]\n\
        Type=Application\n\
        Version=1.0\n\
        Name=Whisper Pill\n\
        GenericName=Desktop-Diktier-Utility\n\
        Comment=Minimalistisches, schwebendes Desktop-Diktier-Utility\n\
        Exec=\"{}\"\n\
        Icon=whisper-pill\n\
        Terminal=false\n\
        Categories=Utility;AudioVideo;\n\
        StartupNotify=false\n\
        X-GNOME-Autostart-enabled=true\n\
        X-KDE-autostart-after=panel\n",
        s_buffy_exe_path
    )
}

/// Schreibt die .desktop-Datei an einen konkreten Zielpfad und setzt Ausfuehrungsrechte.
pub fn write_desktop_file_at(path: &std::path::Path, s_buffy_exe_path: &str) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let s_buffy_entry = generate_desktop_entry(s_buffy_exe_path);
    fs::write(path, s_buffy_entry)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // Ausfuehrungsberechtigung fuer die .desktop-Datei selbst setzen (wichtig fuer KDE Plasma 6)
        if let Ok(meta) = fs::metadata(path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            let _ = fs::set_permissions(path, perms);
        }

        // Sicherstellen, dass auch das Zielprogramm (AppImage / Binary) ausfuehrbar ist
        let target = std::path::Path::new(s_buffy_exe_path);
        if target.exists()
            && let Ok(meta) = fs::metadata(target)
        {
            let mut perms = meta.permissions();
            if perms.mode() & 0o111 == 0 {
                perms.set_mode(perms.mode() | 0o755);
                let _ = fs::set_permissions(target, perms);
            }
        }
    }

    Ok(())
}

/// Loescht die .desktop-Datei am Zielpfad und verifiziert die vollstaendige Entfernung.
pub fn remove_desktop_file_at(path: &std::path::Path) -> Result<(), std::io::Error> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    // Sicherheitspruefung: Sicherstellen, dass die Datei wirklich vom Dateisystem entfernt wurde
    if path.exists() {
        return Err(std::io::Error::other(
            "Autostart-Datei konnte nicht vollstaendig geloescht werden.",
        ));
    }
    Ok(())
}

/// Aktiviert den Autostart durch Anlegen der .desktop-Datei mit persistentem Pfad.
pub fn enable_autostart() -> Result<(), std::io::Error> {
    let path = autostart_desktop_path();
    let s_buffy_exe = get_persistent_executable_path()?;
    write_desktop_file_at(&path, &s_buffy_exe)
}

/// Deaktiviert den Autostart durch Entfernen der .desktop-Datei.
pub fn disable_autostart() -> Result<(), std::io::Error> {
    let path = autostart_desktop_path();
    remove_desktop_file_at(&path)
}

/// Setzt den Autostart-Status gezielt auf aktiv oder inaktiv.
pub fn set_autostart(enabled: bool) -> Result<(), std::io::Error> {
    if enabled {
        enable_autostart()
    } else {
        disable_autostart()
    }
}

/// Schaltet den Autostart-Status um (Toggle).
pub fn toggle_autostart() -> Result<bool, std::io::Error> {
    if is_autostart_enabled() {
        disable_autostart()?;
        Ok(false)
    } else {
        enable_autostart()?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_desktop_entry_structure() {
        let s_buffy_dummy_path = "/usr/local/bin/whisper_pill";
        let s_buffy_entry = generate_desktop_entry(s_buffy_dummy_path);

        assert!(s_buffy_entry.contains("[Desktop Entry]"));
        assert!(s_buffy_entry.contains("Type=Application"));
        assert!(s_buffy_entry.contains("Name=Whisper Pill"));
        assert!(s_buffy_entry.contains("Exec=\"/usr/local/bin/whisper_pill\""));
        // Spezifische Flags fuer KDE Plasma und GNOME
        assert!(s_buffy_entry.contains("X-KDE-autostart-after=panel"));
        assert!(s_buffy_entry.contains("X-GNOME-Autostart-enabled=true"));
    }

    #[test]
    fn test_autostart_desktop_path_resolution() {
        let path = autostart_desktop_path();
        assert!(path.ends_with("autostart/whisper-pill.desktop"));
    }

    #[test]
    fn test_write_and_verify_complete_removal_on_disable() {
        let temp_dir = std::env::temp_dir().join("whisper_pill_test_autostart_clean");
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::create_dir_all(&temp_dir);

        let desktop_file = temp_dir.join("test-pill.desktop");

        // 1. Schreiben
        let res_write = write_desktop_file_at(&desktop_file, "/opt/whisper_pill/bin");
        assert!(res_write.is_ok());
        assert!(desktop_file.exists());

        // 2. Loeschen
        let res_remove = remove_desktop_file_at(&desktop_file);
        assert!(res_remove.is_ok());

        // 3. Verifikation: Datei ist vollstaendig vom Dateisystem entfernt!
        assert!(!desktop_file.exists(), "Die .desktop Datei muss nach Deaktivierung geloescht sein");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_persistent_executable_path_fallback() {
        let res = get_persistent_executable_path();
        assert!(res.is_ok());
        let s_path = res.unwrap();
        assert!(!s_path.is_empty());
    }

    #[test]
    fn test_invalid_exec_path_detected_as_disabled() {
        let temp_dir = std::env::temp_dir().join("whisper_pill_test_invalid_exec");
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::create_dir_all(&temp_dir);

        let desktop_file = temp_dir.join("whisper-pill.desktop");
        let non_existing = "/tmp/.mount_WhispeeCaFBk/usr/bin/whisper_pill";
        let content = generate_desktop_entry(non_existing);
        let _ = fs::write(&desktop_file, content);

        // Pruefen direkt am Dateiinhalt
        let s_buffy_content = fs::read_to_string(&desktop_file).unwrap();
        let mut is_valid = true;
        for line in s_buffy_content.lines() {
            if let Some(s_exec) = line.strip_prefix("Exec=") {
                let s_clean = s_exec.trim().trim_matches('"');
                if !s_clean.is_empty() && !std::path::Path::new(s_clean).exists() {
                    is_valid = false;
                }
            }
        }
        assert!(!is_valid, "Ein veralteter Mount-Pfad muss als ungueltig erkannt werden");

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
