//! System-Tray-Icon Integration via ksni (KDE StatusNotifierItem fuer Linux/Wayland).
//!
//! Stellt ein natives Icon in der System-Taskleiste bereit mit Schnellaktionen
//! zur Aufnahmesteuerung und zum Beenden der Anwendung.

use crossbeam_channel::Sender;
use ksni::menu::{MenuItem, StandardItem};
use ksni::{Tray, TrayMethods};

/// Ereignisse, die aus dem System-Tray-Menue ausgeloest werden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    /// Aufnahme starten bzw. stoppen oder Fenster umschalten.
    Toggle,
    /// Anwendung vollstaendig beenden.
    Quit,
}

/// Repraesentiert das StatusNotifierItem in der Desktop-Leiste.
pub struct PillTray {
    sender: Sender<TrayAction>,
}

impl PillTray {
    /// Erstellt eine neue Instanz des Tray-Icons.
    pub fn new(sender: Sender<TrayAction>) -> Self {
        Self { sender }
    }

    /// Startet das Tray-Icon in einem dedizierten Hintergrund-Thread.
    pub fn spawn_in_thread(self) {
        std::thread::Builder::new()
            .name("whisper-tray".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build();

                if let Ok(runtime) = rt {
                    runtime.block_on(async move {
                        let _ = self.spawn().await;
                        std::future::pending::<()>().await;
                    });
                }
            })
            .expect("Konnte Tray-Thread nicht starten");
    }
}

impl Tray for PillTray {
    fn id(&self) -> String {
        "whisper_pill".into()
    }

    fn title(&self) -> String {
        "whisper-pill".into()
    }

    fn icon_name(&self) -> String {
        "audio-input-microphone".into()
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let sender_toggle = self.sender.clone();
        let sender_quit = self.sender.clone();

        vec![
            StandardItem {
                label: "Diktat starten / beenden (Super+Strg+P)".into(),
                activate: Box::new(move |_| {
                    let _ = sender_toggle.send(TrayAction::Toggle);
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "whisper-pill beenden".into(),
                activate: Box::new(move |_| {
                    let _ = sender_quit.send(TrayAction::Quit);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}
