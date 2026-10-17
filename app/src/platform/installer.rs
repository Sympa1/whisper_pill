//! Plattformuebergreifende Installations-Logik fuer den Whisper Pill Setup-Wizard.
//!
//! Kapselt Zielpfad-Aufloesung, Datei-Bereitstellung, Startmenue- und Desktop-Verknuepfungen,
//! Autostart-Registrierung sowie die Erstellung eines Deinstallations-Skripts fuer
//! Windows (10/11) und Linux (KDE Plasma, XFCE, GNOME).

use std::fs;
use std::path::{Path, PathBuf};

use crate::config::AppConfig;

/// Konfigurationsparameter fuer einen Installationslauf des Setup-Assistenten.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallOptions {
    /// Zielverzeichnis fuer die Anwendungsdateien.
    pub target_dir: PathBuf,
    /// Soll eine Verknuepfung auf dem Desktop angelegt werden?
    pub create_desktop_shortcut: bool,
    /// Soll ein Eintrag im Startmenue / Anwendungsmenue angelegt werden?
    pub create_startmenu_shortcut: bool,
    /// Soll Whisper Pill automatisch beim Systemstart starten?
    pub enable_autostart: bool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            target_dir: default_install_dir(),
            create_desktop_shortcut: true,
            create_startmenu_shortcut: true,
            enable_autostart: true,
        }
    }
}

/// Ermittelt das empfohlene Standard-Installationsverzeichnis fuer das aktuelle Betriebssystem.
pub fn default_install_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(s_buffy_local) = std::env::var("LOCALAPPDATA")
            && !s_buffy_local.is_empty()
        {
            return PathBuf::from(s_buffy_local)
                .join("Programs")
                .join("WhisperPill");
        }
        let s_buffy_profile = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string());
        PathBuf::from(s_buffy_profile)
            .join("AppData")
            .join("Local")
            .join("Programs")
            .join("WhisperPill")
    }
    #[cfg(not(target_os = "windows"))]
    {
        let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(s_buffy_home)
            .join(".local")
            .join("share")
            .join("whisper_pill")
            .join("bin")
    }
}

/// Liefert den empfohlenen Windows-Standardpfad als Anzeige-String (auch unter Linux fuer Vorschau/Tests nutzbar).
pub fn default_windows_install_path_string() -> String {
    if let Ok(s_buffy_local) = std::env::var("LOCALAPPDATA")
        && !s_buffy_local.is_empty()
    {
        return format!("{}\\Programs\\WhisperPill", s_buffy_local);
    }
    "C:\\Users\\Standard\\AppData\\Local\\Programs\\WhisperPill".to_string()
}

/// Erzeugt ein VBScript, mit dem unter Windows ohne externe Abhaengigkeiten eine native
/// `.lnk`-Verknuepfung (fuer Desktop oder Startmenue) erstellt wird.
pub fn generate_windows_lnk_vbs(
    shortcut_lnk_path: &str,
    target_exe_path: &str,
    working_dir: &str,
    description: &str,
) -> String {
    format!(
        "Set oWS = WScript.CreateObject(\"WScript.Shell\")\r\n\
         sLinkFile = \"{}\"\r\n\
         Set oLink = oWS.CreateShortcut(sLinkFile)\r\n\
         oLink.TargetPath = \"{}\"\r\n\
         oLink.WorkingDirectory = \"{}\"\r\n\
         oLink.Description = \"{}\"\r\n\
         oLink.IconLocation = \"{},0\"\r\n\
         oLink.Save\r\n",
        shortcut_lnk_path, target_exe_path, working_dir, description, target_exe_path
    )
}

/// Erzeugt das Deinstallations-Skript (`uninstall.cmd` fuer Windows bzw. `uninstall.sh` fuer Linux).
pub fn generate_uninstall_script(install_dir: &Path, is_windows: bool) -> String {
    let s_buffy_dir = install_dir.to_string_lossy();
    if is_windows {
        format!(
            "@echo off\r\n\
             echo Deinstalliere Whisper Pill...\r\n\
             taskkill /F /IM whisper_pill.exe >nul 2>&1\r\n\
             reg delete \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run\" /v \"WhisperPill\" /f >nul 2>&1\r\n\
             del /F /Q \"%APPDATA%\\Microsoft\\Windows\\Start Menu\\Programs\\Whisper Pill.lnk\" >nul 2>&1\r\n\
             del /F /Q \"%USERPROFILE%\\Desktop\\Whisper Pill.lnk\" >nul 2>&1\r\n\
             echo Whisper Pill Verknuepfungen wurden entfernt.\r\n\
             echo Programmordner: \"{}\"\r\n",
            s_buffy_dir
        )
    } else {
        format!(
            "#!/usr/bin/env bash\n\
             set -e\n\
             echo \"Deinstalliere Whisper Pill...\"\n\
             rm -f \"$HOME/.config/autostart/whisper-pill.desktop\"\n\
             rm -f \"$HOME/.local/share/applications/whisper-pill.desktop\"\n\
             rm -f \"$HOME/Desktop/whisper-pill.desktop\"\n\
             rm -rf \"{}\"\n\
             echo \"Whisper Pill wurde erfolgreich entfernt.\"\n",
            s_buffy_dir
        )
    }
}

/// Fuehrt die Installation anhand der uebergebenen Optionen durch und gibt ein mehrzeiliges
/// Statusprotokoll fuer das Wizard-Logfenster zurueck.
pub fn perform_installation(options: &InstallOptions) -> Result<String, String> {
    let mut s_buffy_log = String::new();
    s_buffy_log.push_str(&format!(
        "[1/4] Erstelle Zielverzeichnis: {}\n",
        options.target_dir.display()
    ));

    fs::create_dir_all(&options.target_dir).map_err(|e| {
        format!(
            "Zielverzeichnis '{}' konnte nicht angelegt werden: {}",
            options.target_dir.display(),
            e
        )
    })?;

    // Aktuelle Binärdatei bzw. AppImage in das Zielverzeichnis kopieren
    let s_buffy_src_exe = crate::platform::autostart::get_persistent_executable_path()
        .map_err(|e| format!("Quell-Programmdatei konnte nicht ermittelt werden: {}", e))?;
    let src_path = PathBuf::from(&s_buffy_src_exe);

    let s_buffy_bin_name = if cfg!(target_os = "windows") {
        "whisper_pill.exe"
    } else if s_buffy_src_exe.ends_with(".AppImage") {
        "Whisper_Pill-x86_64.AppImage"
    } else {
        "whisper_pill"
    };

    let dest_bin_path = options.target_dir.join(s_buffy_bin_name);
    if src_path.exists() && src_path != dest_bin_path {
        let _ = fs::copy(&src_path, &dest_bin_path);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = fs::metadata(&dest_bin_path) {
                let mut perms = meta.permissions();
                perms.set_mode(0o755);
                let _ = fs::set_permissions(&dest_bin_path, perms);
            }
        }
    }
    s_buffy_log.push_str(&format!(
        "[2/4] Programmdatei bereitgestellt: {}\n",
        dest_bin_path.display()
    ));

    // Standard-Konfiguration speichern
    let mut cfg = AppConfig::load();
    cfg.autostart = options.enable_autostart;
    let _ = cfg.save();
    s_buffy_log.push_str("[3/4] Benutzerkonfiguration (config.json) aktualisiert.\n");

    // Verknuepfungen & Autostart einrichten
    let s_buffy_dest_str = dest_bin_path.to_string_lossy().to_string();
    #[cfg(target_os = "windows")]
    {
        setup_windows_integration(options, &s_buffy_dest_str, &mut s_buffy_log);
    }
    #[cfg(not(target_os = "windows"))]
    {
        setup_linux_integration(options, &s_buffy_dest_str, &mut s_buffy_log);
    }

    // Uninstall-Skript erzeugen
    let uninstall_name = if cfg!(target_os = "windows") {
        "uninstall.cmd"
    } else {
        "uninstall.sh"
    };
    let uninstall_path = options.target_dir.join(uninstall_name);
    let s_buffy_uninstall = generate_uninstall_script(&options.target_dir, cfg!(target_os = "windows"));
    let _ = fs::write(&uninstall_path, s_buffy_uninstall);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(&uninstall_path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            let _ = fs::set_permissions(&uninstall_path, perms);
        }
    }

    s_buffy_log.push_str(&format!(
        "[4/4] Deinstallations-Skript erstellt: {}\n✔ Installation erfolgreich abgeschlossen.",
        uninstall_path.display()
    ));

    Ok(s_buffy_log)
}

#[cfg(target_os = "windows")]
fn setup_windows_integration(options: &InstallOptions, target_exe: &str, s_buffy_log: &mut String) {
    let s_buffy_workdir = options.target_dir.to_string_lossy().to_string();

    if options.create_desktop_shortcut
        && let Ok(profile) = std::env::var("USERPROFILE")
    {
        let lnk = PathBuf::from(profile).join("Desktop").join("Whisper Pill.lnk");
        create_windows_shortcut(&lnk, target_exe, &s_buffy_workdir);
        s_buffy_log.push_str("      • Desktop-Verknüpfung erstellt.\n");
    }

    if options.create_startmenu_shortcut
        && let Ok(appdata) = std::env::var("APPDATA")
    {
        let lnk = PathBuf::from(appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Whisper Pill.lnk");
        create_windows_shortcut(&lnk, target_exe, &s_buffy_workdir);
        s_buffy_log.push_str("      • Startmenü-Eintrag erstellt.\n");
    }

    if options.enable_autostart {
        let _ = std::process::Command::new("reg")
            .args([
                "add",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v",
                "WhisperPill",
                "/t",
                "REG_SZ",
                "/d",
                &format!("\"{}\"", target_exe),
                "/f",
            ])
            .status();
        s_buffy_log.push_str("      • Windows-Autostart (Registry Run) registriert.\n");
    }
}

#[cfg(target_os = "windows")]
fn create_windows_shortcut(lnk_path: &Path, target_exe: &str, working_dir: &str) {
    let vbs_content = generate_windows_lnk_vbs(
        &lnk_path.to_string_lossy(),
        target_exe,
        working_dir,
        "Whisper Pill – Lokales KI-Desktop-Diktier-Utility",
    );
    let tmp_vbs = std::env::temp_dir().join("whisper_pill_shortcut.vbs");
    if fs::write(&tmp_vbs, vbs_content).is_ok() {
        let _ = std::process::Command::new("wscript")
            .args(["//NoLogo", &tmp_vbs.to_string_lossy()])
            .status();
        let _ = fs::remove_file(&tmp_vbs);
    }
}

#[cfg(not(target_os = "windows"))]
fn setup_linux_integration(options: &InstallOptions, target_exe: &str, s_buffy_log: &mut String) {
    let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let home_path = PathBuf::from(&s_buffy_home);

    if options.create_startmenu_shortcut {
        let app_dir = home_path.join(".local").join("share").join("applications");
        let desktop_file = app_dir.join("whisper-pill.desktop");
        let _ = crate::platform::autostart::write_desktop_file_at(&desktop_file, target_exe);
        s_buffy_log.push_str("      • Anwendungsmenü-Eintrag erstellt.\n");
    }

    if options.create_desktop_shortcut {
        let desktop_dir = home_path.join("Desktop");
        if desktop_dir.exists() {
            let desktop_file = desktop_dir.join("whisper-pill.desktop");
            let _ = crate::platform::autostart::write_desktop_file_at(&desktop_file, target_exe);
            s_buffy_log.push_str("      • Desktop-Verknüpfung erstellt.\n");
        }
    }

    if options.enable_autostart {
        let autostart_path = crate::platform::autostart::autostart_desktop_path();
        let _ = crate::platform::autostart::write_desktop_file_at(&autostart_path, target_exe);
        s_buffy_log.push_str("      • System-Autostart aktiviert.\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_install_options() {
        let opts = InstallOptions::default();
        assert!(opts.create_desktop_shortcut);
        assert!(opts.create_startmenu_shortcut);
        assert!(opts.enable_autostart);
        assert!(!opts.target_dir.as_os_str().is_empty());
    }

    #[test]
    fn test_generate_windows_lnk_vbs_content() {
        let s_buffy_vbs = generate_windows_lnk_vbs(
            "C:\\Users\\Test\\Desktop\\Whisper Pill.lnk",
            "C:\\Program Files\\WhisperPill\\whisper_pill.exe",
            "C:\\Program Files\\WhisperPill",
            "Whisper Pill",
        );
        assert!(s_buffy_vbs.contains("WScript.CreateObject(\"WScript.Shell\")"));
        assert!(s_buffy_vbs.contains("CreateShortcut"));
        assert!(s_buffy_vbs.contains("whisper_pill.exe"));
    }

    #[test]
    fn test_generate_uninstall_scripts_windows_and_linux() {
        let win_script = generate_uninstall_script(Path::new("C:\\WhisperPill"), true);
        assert!(win_script.contains("taskkill /F /IM whisper_pill.exe"));
        assert!(win_script.contains("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run"));

        let lin_script = generate_uninstall_script(Path::new("/tmp/whisper_pill_test"), false);
        assert!(lin_script.contains("#!/usr/bin/env bash"));
        assert!(lin_script.contains("whisper-pill.desktop"));
    }

    #[test]
    fn test_perform_installation_in_temp_dir() {
        let temp_target = std::env::temp_dir().join("whisper_pill_installer_test_dir");
        let _ = fs::remove_dir_all(&temp_target);

        let opts = InstallOptions {
            target_dir: temp_target.clone(),
            create_desktop_shortcut: false,
            create_startmenu_shortcut: false,
            enable_autostart: false,
        };

        let res = perform_installation(&opts);
        assert!(res.is_ok());
        let s_buffy_log = res.unwrap();
        assert!(s_buffy_log.contains("Installation erfolgreich abgeschlossen"));
        assert!(temp_target.exists());

        let _ = fs::remove_dir_all(&temp_target);
    }
}
