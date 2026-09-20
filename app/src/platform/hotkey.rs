//! Globaler Hotkey-Empfaenger fuer Linux und Desktop-Umgebungen.
//!
//! Registriert den globalen Shortcut (Super+Strg+P / Meta+Ctrl+P) in KDE KGlobalAccel
//! und lauscht auf systemweite Tastenkombinationen fuer den One-Touch Workflow.

use crossbeam_channel::Sender;

/// Signalisiert, dass die globale Tastenkombination (Super+Strg+P) ausgeloest wurde.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotkeyTriggeredEvent;

/// Verwaltet den Empfang globaler Tastenkombinationen.
pub struct HotkeyManager {
    /// Sender fuer Hotkey-Events an die UI-Hauptschleife.
    sender: Sender<HotkeyTriggeredEvent>,
}

impl HotkeyManager {
    /// Erstellt eine neue Instanz des Hotkey-Managers mit dem Zielkanal.
    pub fn new(sender: Sender<HotkeyTriggeredEvent>) -> Self {
        Self { sender }
    }

    /// Registriert den Shortcut in KDE KGlobalAccel und startet den Hintergrund-Listener.
    #[cfg(target_os = "linux")]
    pub fn start_listener(&self) {
        // 1. Shortcut aktiv bei KDE Plasma anmelden
        Self::register_kde_shortcut();

        let sender_clone = self.sender.clone();

        std::thread::Builder::new()
            .name("whisper-dbus-hotkey".to_string())
            .spawn(move || {
                let rt = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Fehler beim Erstellen der Tokio-Runtime fuer Hotkeys: {}", e);
                        return;
                    }
                };

                rt.block_on(async move {
                    Self::listen_dbus_signals(sender_clone).await;
                });
            })
            .expect("Konnte Hotkey-Thread nicht starten");
    }

    /// Dummy-Listener fuer Nicht-Linux Plattformen.
    #[cfg(not(target_os = "linux"))]
    pub fn start_listener(&self) {
        // Unter Windows wird dies mit RegisterHotKeyW umgesetzt
    }

    /// Meldet den globalen Hotkey (Meta+Ctrl+P) beim KDE KGlobalAccel-Dienst an.
    #[cfg(target_os = "linux")]
    pub fn register_kde_shortcut() {
        // 1. Aktion in KGlobalAccel registrieren
        let _ = std::process::Command::new("gdbus")
            .args([
                "call",
                "--session",
                "--dest",
                "org.kde.kglobalaccel",
                "--object-path",
                "/kglobalaccel",
                "--method",
                "org.kde.KGlobalAccel.doRegister",
                "['whisper_pill', 'toggle_pill', 'whisper-pill', 'Pill ein-/ausblenden']",
            ])
            .output();

        // 2. Tastenkombination Meta+Ctrl+P (Qt Keycode 335544400) zuweisen
        let _ = std::process::Command::new("gdbus")
            .args([
                "call",
                "--session",
                "--dest",
                "org.kde.kglobalaccel",
                "--object-path",
                "/kglobalaccel",
                "--method",
                "org.kde.KGlobalAccel.setShortcut",
                "['whisper_pill', 'toggle_pill', 'whisper-pill', 'Pill ein-/ausblenden']",
                "[335544400]",
                "uint32 2",
            ])
            .output();

        // 3. Optional in kglobalshortcutsrc schreiben fuer Persistenz
        for bin in ["kwriteconfig6", "kwriteconfig5"] {
            let status = std::process::Command::new(bin)
                .args([
                    "--file",
                    "kglobalshortcutsrc",
                    "--group",
                    "whisper_pill",
                    "--key",
                    "toggle_pill",
                    "Meta+Ctrl+P,Meta+Ctrl+P,Pill ein-/ausblenden",
                ])
                .status();

            if status.map(|s| s.success()).unwrap_or(false) {
                break;
            }
        }
    }

    /// Lauscht asynchron auf D-Bus Signale von org.kde.kglobalaccel.
    #[cfg(target_os = "linux")]
    async fn listen_dbus_signals(sender: Sender<HotkeyTriggeredEvent>) {
        use futures_util::StreamExt;
        use zbus::MatchRule;

        let conn = match zbus::connection::Builder::session() {
            Ok(builder) => match builder.build().await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("D-Bus Sitzungsverbindung fehlgeschlagen: {}", e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("D-Bus Builder-Fehler: {}", e);
                return;
            }
        };

        // Match-Rule fuer KDE KGlobalAccel Shortcut-Signale
        let rule_builder = MatchRule::builder().msg_type(zbus::message::Type::Signal);

        let rule = match rule_builder
            .interface("org.kde.kglobalaccel.Component")
            .and_then(|b| b.member("globalShortcutPressed"))
        {
            Ok(b) => b.build(),
            Err(e) => {
                eprintln!("Konnte D-Bus Match-Rule nicht konfigurieren: {}", e);
                return;
            }
        };

        let mut stream = match zbus::MessageStream::for_match_rule(rule, &conn, None).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Konnte D-Bus Match-Rule nicht registrieren: {}", e);
                return;
            }
        };

        while let Some(_msg) = stream.next().await {
            // Signal erhalten: Hotkey ausloesen
            let _ = sender.send(HotkeyTriggeredEvent);
        }
    }

    /// Erlaubt das manuelle Ausloesen des Hotkeys (z. B. fuer Tests oder UI-Buttons).
    pub fn trigger_manually(&self) {
        let _ = self.sender.send(HotkeyTriggeredEvent);
    }
}
