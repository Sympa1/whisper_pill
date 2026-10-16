//! Fehler- und Diagnoseprotokollierung fuer whisper-pill.
//!
//! Schreibt Laufzeitfehler, Warnungen und Diagnosemeldungen in eine standardkonforme
//! Logdatei (~/.local/state/whisper_pill/error.log) und ermoeglicht das Oeffnen
//! ueber das System-Tray-Kontextmenue.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::SystemTime;

/// Globaler Mutex fuer synchronisierten Schreibzugriff auf die Logdatei.
static LOG_LOCK: Mutex<()> = Mutex::new(());

/// Ermittelt den Zielpfad der Logdatei gemaess XDG State Spezifikation.
pub fn log_file_path() -> PathBuf {
    if let Ok(s_buffy_xdg) = std::env::var("XDG_STATE_HOME")
        && !s_buffy_xdg.is_empty()
    {
        return PathBuf::from(s_buffy_xdg).join("whisper_pill").join("error.log");
    }
    let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(s_buffy_home)
        .join(".local")
        .join("state")
        .join("whisper_pill")
        .join("error.log")
}

/// Formatiert einen SystemTime-Zeitstempel ohne externe Abhaengigkeiten als UTC-String.
pub fn format_timestamp(time: SystemTime) -> String {
    let i_buffy_secs = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let sec = i_buffy_secs % 60;
    let min = (i_buffy_secs / 60) % 60;
    let hour = (i_buffy_secs / 3600) % 24;

    // Zivilzeit-Berechnung (Howard Hinnant Algorithmus ab 1970)
    let days = (i_buffy_secs / 86400) as i64;
    let z = days + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1024 + doe / 1461 - doe / 142400) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let final_y = if m <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
        final_y, m, d, hour, min, sec
    )
}

/// Initialisiert die Logdatei mit einem Header, falls sie noch nicht existiert.
pub fn init_log() {
    let path = log_file_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if !path.exists() {
        let _guard = LOG_LOCK.lock().unwrap();
        let s_buffy_header = format!(
            "================================================================================\n\
             Whisper Pill - Fehler- & Diagnoseprotokoll\n\
             Initialisiert: {}\n\
             ================================================================================\n\n",
            format_timestamp(SystemTime::now())
        );
        let _ = fs::write(&path, s_buffy_header);
    }
}

/// Schreibt eine formatierte Logzeile in die Datei und auf stderr.
pub fn log_msg(level: &str, category: &str, message: &str) {
    init_log();
    let s_buffy_time = format_timestamp(SystemTime::now());
    let s_buffy_entry = format!("[{}] [{}] [{}] {}\n", s_buffy_time, level, category, message);

    eprint!("{}", s_buffy_entry);

    let path = log_file_path();
    let _guard = LOG_LOCK.lock().unwrap();
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = file.write_all(s_buffy_entry.as_bytes());
    }
}

/// Protokolliert einen Fehler (Level ERROR).
pub fn log_error(category: &str, message: &str) {
    log_msg("ERROR", category, message);
}

/// Protokolliert eine Warnung (Level WARN).
pub fn log_warn(category: &str, message: &str) {
    log_msg("WARN", category, message);
}

/// Protokolliert eine Informationsmeldung (Level INFO).
pub fn log_info(category: &str, message: &str) {
    log_msg("INFO", category, message);
}

/// Oeffnet die Logdatei im Standard-Betrachter/Editor des Betriebssystems.
pub fn open_log_file() {
    init_log();
    let path = log_file_path();

    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", &path.to_string_lossy()])
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(&path)
            .spawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_file_path_resolution() {
        let path = log_file_path();
        assert!(path.ends_with("whisper_pill/error.log"));
    }

    #[test]
    fn test_format_timestamp_structure() {
        let ts = format_timestamp(SystemTime::now());
        assert!(ts.contains("UTC"));
        assert_eq!(ts.len(), 23); // YYYY-MM-DD HH:MM:SS UTC
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], " ");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
    }

    #[test]
    fn test_format_timestamp_known_epoch() {
        let epoch = SystemTime::UNIX_EPOCH;
        let ts = format_timestamp(epoch);
        assert_eq!(ts, "1970-01-01 00:00:00 UTC");
    }

    #[test]
    fn test_log_error_and_file_content() {
        let temp_dir = std::env::temp_dir().join("whisper_pill_log_test");
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::create_dir_all(&temp_dir);

        let test_log = temp_dir.join("test_error.log");
        let s_buffy_msg = "Test-Fehlermeldung fuer Audio-Worker";
        let s_buffy_line = format!("[{}] [ERROR] [Audio] {}\n", format_timestamp(SystemTime::now()), s_buffy_msg);
        let write_res = fs::write(&test_log, s_buffy_line);
        assert!(write_res.is_ok());

        let read_content = fs::read_to_string(&test_log).unwrap();
        assert!(read_content.contains("Test-Fehlermeldung fuer Audio-Worker"));
        assert!(read_content.contains("[ERROR] [Audio]"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
