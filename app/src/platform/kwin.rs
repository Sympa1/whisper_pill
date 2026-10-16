//! KWin-Fensterregeln fuer die Whisper Pill unter KDE Plasma (Wayland / X11).
//!
//! Stellt sicher, dass das schwebende Overlay weder in der Anwendungsleiste (Taskbar)
//! noch im Fensterwechsler (Alt+Tab) angezeigt wird.

/// Stellt sicher, dass die KWin-Fensterregel fuer Whisper Pill existiert und aktiv ist.
#[cfg(target_os = "linux")]
pub fn ensure_kwin_window_rules() {
    for bin in ["kreadconfig6", "kreadconfig5"] {
        if let Ok(output) = std::process::Command::new(bin)
            .args(["--file", "kwinrulesrc", "--group", "General", "--key", "rules"])
            .output()
            && output.status.success()
        {
            let mut s_buffy_rules = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !s_buffy_rules.contains("whisper_pill") {
                    if !s_buffy_rules.is_empty() {
                        s_buffy_rules.push(',');
                    }
                    s_buffy_rules.push_str("whisper_pill");
                }

                let write_bin = if bin == "kreadconfig6" { "kwriteconfig6" } else { "kwriteconfig5" };
                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "General", "--key", "rules", &s_buffy_rules])
                    .status();

                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "whisper_pill", "--key", "Description", "Whisper Pill Overlay"])
                    .status();
                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "whisper_pill", "--key", "title", "Whisper Pill"])
                    .status();
                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "whisper_pill", "--key", "titlematch", "1"])
                    .status();
                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "whisper_pill", "--key", "skiptaskbar", "true"])
                    .status();
                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "whisper_pill", "--key", "skiptaskbarrule", "2"])
                    .status();
                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "whisper_pill", "--key", "skipswitcher", "true"])
                    .status();
                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "whisper_pill", "--key", "skipswitcherrule", "2"])
                    .status();
                // Hinweis: 'above' wird in KWin NICHT mit aboverule=2 erzwungen.
                // Ein erzwungenes Keep-Above fuehrt bei Dropdown-Terminals wie Yakuake (F12)
                // oder beim Druecken der Super-Taste (KDE Anwendungsmenue) dazu, dass
                // die Pill darueber schwebt und visuell durchschimmert.
                // Slints natives always-on-top regelt die normale Vordergrund-Platzierung.
                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "whisper_pill", "--key", "above", "false"])
                    .status();
                let _ = std::process::Command::new(write_bin)
                    .args(["--file", "kwinrulesrc", "--group", "whisper_pill", "--key", "aboverule", "0"])
                    .status();

                // KWin-Konfiguration via D-Bus neu laden
                for qdbus_bin in ["qdbus6", "qdbus"] {
                    let _ = std::process::Command::new(qdbus_bin)
                        .args(["org.kde.KWin", "/KWin", "reconfigure"])
                        .status();
                }

                break;
            }
        }
    }

/// Setzt unter X11 / XFCE die EWMH-Fensterattribute (_NET_WM_STATE_SKIP_TASKBAR, _NET_WM_STATE_SKIP_PAGER),
/// damit das Fenster weder in der XFCE-Leiste noch im Alt+Tab Umschalter aufgefuehrt wird.
#[cfg(target_os = "linux")]
pub fn ensure_x11_window_properties() {
    if std::env::var("DISPLAY").is_ok() {
        let _ = std::process::Command::new("xprop")
            .args([
                "-name",
                "Whisper Pill",
                "-f",
                "_NET_WM_STATE",
                "32a",
                "-set",
                "_NET_WM_STATE",
                "_NET_WM_STATE_SKIP_TASKBAR,_NET_WM_STATE_SKIP_PAGER",
            ])
            .output();
    }
}

/// Fallback fuer Nicht-Linux-Plattformen.
#[cfg(not(target_os = "linux"))]
pub fn ensure_kwin_window_rules() {}

#[cfg(not(target_os = "linux"))]
pub fn ensure_x11_window_properties() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_kwin_window_rules_runs_without_panic() {
        ensure_kwin_window_rules();
    }

    #[test]
    fn test_ensure_x11_window_properties_runs_without_panic() {
        ensure_x11_window_properties();
    }
}
