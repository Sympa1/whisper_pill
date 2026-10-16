//! System-Tray-Icon Integration via ksni (KDE StatusNotifierItem fuer Linux/Wayland).
//!
//! Stellt ein natives Icon in der System-Taskleiste bereit mit Schnellaktionen
//! zur Aufnahmesteuerung, Online-Hilfe und zum Beenden der Anwendung.

use crossbeam_channel::Sender;
use ksni::menu::{CheckmarkItem, MenuItem, StandardItem};
use ksni::{Icon, ToolTip, Tray, TrayMethods};

use crate::platform::autostart::{is_autostart_enabled, toggle_autostart};

/// Ereignisse, die aus dem System-Tray-Menue ausgeloest werden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    /// Aufnahme starten bzw. stoppen oder Fenster umschalten.
    Toggle,
    /// Einstellungsdialog oeffnen.
    Settings,
    /// Fehler- und Diagnoseprotokoll oeffnen.
    OpenLog,
    /// Anwendung vollstaendig beenden.
    Quit,
}

/// Kommunikations-Handle zur dynamischen Status-Aktualisierung des Systemtray-Icons.
#[derive(Clone)]
pub struct TrayHandle {
    status_tx: tokio::sync::mpsc::UnboundedSender<bool>,
}

impl TrayHandle {
    /// Aktualisiert den Transkriptionsstatus des Systemtray-Icons.
    pub fn set_transcribing(&self, is_transcribing: bool) {
        let _ = self.status_tx.send(is_transcribing);
    }
}

/// Oeffnet eine Webadresse im Standard-Browser des Betriebssystems.
pub fn open_url(url: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(url)
            .spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", url])
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(url)
            .spawn();
    }
}

/// Erzeugt ein Kapsel-Icon im ARGB32-Format fuer das StatusNotifierItem.
///
/// Im Normalzustand zeichnet es die charakteristische whisper-pill Kapsel mit dunklem
/// Hintergrund, cyanfarbenem Rand und rotem Aufnahme-Punkt (REC-Indikator).
/// Im Transkriptions-Zustand wird anstelle des roten Punkts eine dynamische
/// Wellenform (ılı) in leuchtendem Bernstein (#F59E0B) dargestellt.
pub fn create_tray_icon(i_buffy_size: i32, is_transcribing: bool) -> Icon {
    let mut s_buffy_data = Vec::with_capacity((i_buffy_size * i_buffy_size * 4) as usize);

    let scale = i_buffy_size as f32 / 64.0;
    let cx = i_buffy_size as f32 / 2.0;
    let cy = i_buffy_size as f32 / 2.0;
    let half_w = 28.0 * scale;
    let radius = 16.0 * scale;
    let dot_radius = 8.0 * scale;
    let border_w = 2.0 * scale;

    // Parameter fuer die 3 Wellenform-Balken bei Transkription (ılı)
    let bar_w = 2.5 * scale;
    let bar_r = 1.25 * scale;
    let bar_h_mid = 10.0 * scale;
    let bar_h_side = 6.0 * scale;
    let bar_offset = 6.0 * scale;

    for i_buffy_py in 0..i_buffy_size {
        let py = i_buffy_py as f32 + 0.5 - cy;
        for i_buffy_px in 0..i_buffy_size {
            let px = i_buffy_px as f32 + 0.5 - cx;

            // Exakte mathematische Distanz zur horizontalen Kapsel
            let cap_dx = (px.abs() - (half_w - radius)).max(0.0);
            let cap_dy = py.abs();
            let dist_capsule = (cap_dx * cap_dx + cap_dy * cap_dy).sqrt() - radius;

            // Antialiasing an den Kanten
            let capsule_alpha = (0.5 - dist_capsule).clamp(0.0, 1.0);

            if capsule_alpha <= 0.0 {
                // Vollstaendig transparenter Pixel ausserhalb der Kapsel
                s_buffy_data.extend_from_slice(&[0, 0, 0, 0]);
            } else {
                // Flaechendeckung des Kapsel-Innenbereichs vs. Rand
                let fill_alpha = (-dist_capsule - border_w + 0.5).clamp(0.0, 1.0);

                // Cyanfarbener Rand (#38BDF8) oder dunkler Hintergrund (#18181B)
                let r_base = 56.0 * (1.0 - fill_alpha) + 24.0 * fill_alpha;
                let g_base = 189.0 * (1.0 - fill_alpha) + 24.0 * fill_alpha;
                let b_base = 248.0 * (1.0 - fill_alpha) + 27.0 * fill_alpha;

                let (r_final, g_final, b_final) = if is_transcribing {
                    // Transkriptions-Modus: 3 leuchtende Wellenform-Balken in Bernstein (#F59E0B)
                    let dx_mid = (px.abs() - (bar_w - bar_r)).max(0.0);
                    let dy_mid = (py.abs() - (bar_h_mid - bar_r)).max(0.0);
                    let dist_mid = (dx_mid * dx_mid + dy_mid * dy_mid).sqrt() - bar_r;

                    let dx_side = ((px.abs() - bar_offset).abs() - (bar_w - bar_r)).max(0.0);
                    let dy_side = (py.abs() - (bar_h_side - bar_r)).max(0.0);
                    let dist_side = (dx_side * dx_side + dy_side * dy_side).sqrt() - bar_r;

                    let dist_bars = dist_mid.min(dist_side);
                    let bar_alpha = (0.5 - dist_bars).clamp(0.0, 1.0);

                    // Bernsteinfarbe (#F59E0B: R=245, G=158, B=11)
                    let r = r_base * (1.0 - bar_alpha) + 245.0 * bar_alpha;
                    let g = g_base * (1.0 - bar_alpha) + 158.0 * bar_alpha;
                    let b = b_base * (1.0 - bar_alpha) + 11.0 * bar_alpha;
                    (r, g, b)
                } else {
                    // Normaler Zustand: Roter Aufnahme-Punkt (#EF4444)
                    let dist_dot = (px * px + py * py).sqrt() - dot_radius;
                    let dot_alpha = (0.5 - dist_dot).clamp(0.0, 1.0);
                    let r = r_base * (1.0 - dot_alpha) + 239.0 * dot_alpha;
                    let g = g_base * (1.0 - dot_alpha) + 68.0 * dot_alpha;
                    let b = b_base * (1.0 - dot_alpha) + 68.0 * dot_alpha;
                    (r, g, b)
                };

                // Big-Endian ARGB32 (Network Byte Order): [Alpha, Red, Green, Blue]
                let a_byte = (capsule_alpha * 255.0).round() as u8;
                let r_byte = (r_final.clamp(0.0, 255.0)).round() as u8;
                let g_byte = (g_final.clamp(0.0, 255.0)).round() as u8;
                let b_byte = (b_final.clamp(0.0, 255.0)).round() as u8;

                s_buffy_data.extend_from_slice(&[a_byte, r_byte, g_byte, b_byte]);
            }
        }
    }

    Icon {
        width: i_buffy_size,
        height: i_buffy_size,
        data: s_buffy_data,
    }
}

/// Repraesentiert das StatusNotifierItem in der Desktop-Leiste.
pub struct PillTray {
    sender: Sender<TrayAction>,
    is_transcribing: bool,
}

impl PillTray {
    /// Erstellt eine neue Instanz des Tray-Icons.
    pub fn new(sender: Sender<TrayAction>) -> Self {
        Self {
            sender,
            is_transcribing: false,
        }
    }

    /// Aktualisiert den internen Zustand (Transkription aktiv/inaktiv).
    pub fn set_transcribing(&mut self, is_transcribing: bool) {
        self.is_transcribing = is_transcribing;
    }

    /// Startet das Tray-Icon in einem dedizierten Hintergrund-Thread und liefert ein Control-Handle.
    pub fn spawn_in_thread(self) -> TrayHandle {
        let (status_tx, mut status_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();

        std::thread::Builder::new()
            .name("whisper-tray".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build();

                if let Ok(runtime) = rt {
                    runtime.block_on(async move {
                        if let Ok(handle) = self.spawn().await {
                            while let Some(is_transcribing) = status_rx.recv().await {
                                let _ = handle
                                    .update(|tray| {
                                        tray.set_transcribing(is_transcribing);
                                    })
                                    .await;
                            }
                        }
                    });
                }
            })
            .expect("Konnte Tray-Thread nicht starten");

        TrayHandle { status_tx }
    }
}

impl Tray for PillTray {
    fn id(&self) -> String {
        "whisper_pill".into()
    }

    fn title(&self) -> String {
        if self.is_transcribing {
            "Whisper Pill (Transkribiert...)".into()
        } else {
            "Whisper Pill".into()
        }
    }

    fn icon_name(&self) -> String {
        if self.is_transcribing {
            "whisper-pill-transcribing".into()
        } else {
            "whisper-pill".into()
        }
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        vec![
            create_tray_icon(32, self.is_transcribing),
            create_tray_icon(64, self.is_transcribing),
        ]
    }

    /// Liefert die Tooltip-Informationen fuer Mouseover-Hover im System-Tray.
    fn tool_tip(&self) -> ToolTip {
        if self.is_transcribing {
            ToolTip {
                icon_name: "whisper-pill-transcribing".into(),
                icon_pixmap: vec![
                    create_tray_icon(32, true),
                    create_tray_icon(64, true),
                ],
                title: "Whisper Pill: Transkribiert...".into(),
                description: "Whisper-KI verarbeitet Sprachaufnahme...".into(),
            }
        } else {
            ToolTip {
                icon_name: "whisper-pill".into(),
                icon_pixmap: vec![
                    create_tray_icon(32, false),
                    create_tray_icon(64, false),
                ],
                title: "Whisper Pill (Super+Strg+P)".into(),
                description: "Minimalistisches Desktop-Diktier-Utility".into(),
            }
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let sender_toggle = self.sender.clone();
        let sender_settings = self.sender.clone();
        let sender_log = self.sender.clone();
        let sender_quit = self.sender.clone();

        let toggle_label = if self.is_transcribing {
            "⏳ Transkription läuft...".to_string()
        } else {
            "Diktat starten / beenden (Super+Strg+P)".to_string()
        };

        vec![
            StandardItem {
                label: toggle_label,
                activate: Box::new(move |_| {
                    let _ = sender_toggle.send(TrayAction::Toggle);
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Einstellungen...".into(),
                activate: Box::new(move |_| {
                    let _ = sender_settings.send(TrayAction::Settings);
                }),
                ..Default::default()
            }
            .into(),
            CheckmarkItem {
                label: "Mit dem System starten (Autostart)".into(),
                checked: is_autostart_enabled(),
                activate: Box::new(|_| {
                    let _ = toggle_autostart();
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Fehlerlog anzeigen...".into(),
                activate: Box::new(move |_| {
                    let _ = sender_log.send(TrayAction::OpenLog);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Hilfe (GitHub)".into(),
                activate: Box::new(|_| {
                    open_url("https://github.com/Sympa1/whisper_pill");
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Whisper Pill Beenden".into(),
                activate: Box::new(move |_| {
                    let _ = sender_quit.send(TrayAction::Quit);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tray_icon_dimensions_and_size() {
        let icon_32 = create_tray_icon(32, false);
        assert_eq!(icon_32.width, 32);
        assert_eq!(icon_32.height, 32);
        assert_eq!(icon_32.data.len(), 32 * 32 * 4);

        let icon_64 = create_tray_icon(64, true);
        assert_eq!(icon_64.width, 64);
        assert_eq!(icon_64.height, 64);
        assert_eq!(icon_64.data.len(), 64 * 64 * 4);
    }

    #[test]
    fn test_create_tray_icon_pixel_content() {
        let icon_idle = create_tray_icon(64, false);

        // 1. Die Ecke (0, 0) muss transparent sein (A = 0)
        let alpha_corner = icon_idle.data[0];
        assert_eq!(alpha_corner, 0, "Ecke muss transparent sein");

        // 2. Das Zentrum (32, 32) muss im Normalzustand der rote REC-Punkt sein (#EF4444: A=255, R=239, G=68, B=68)
        let i_buffy_center_idx = (32 * 64 + 32) * 4;
        let a_center = icon_idle.data[i_buffy_center_idx];
        let r_center = icon_idle.data[i_buffy_center_idx + 1];
        let g_center = icon_idle.data[i_buffy_center_idx + 2];
        let b_center = icon_idle.data[i_buffy_center_idx + 3];

        assert_eq!(a_center, 255);
        assert_eq!(r_center, 239);
        assert_eq!(g_center, 68);
        assert_eq!(b_center, 68);

        // 3. Im Transkriptions-Zustand muss das Zentrum bernsteinfarben sein (#F59E0B: R=245, G=158, B=11)
        let icon_transcribing = create_tray_icon(64, true);
        let a_trans = icon_transcribing.data[i_buffy_center_idx];
        let r_trans = icon_transcribing.data[i_buffy_center_idx + 1];
        let g_trans = icon_transcribing.data[i_buffy_center_idx + 2];
        let b_trans = icon_transcribing.data[i_buffy_center_idx + 3];

        assert_eq!(a_trans, 255);
        assert_eq!(r_trans, 245);
        assert_eq!(g_trans, 158);
        assert_eq!(b_trans, 11);
    }

    #[test]
    fn test_tray_menu_structure() {
        let (tx, _rx) = crossbeam_channel::unbounded();
        let tray = PillTray::new(tx);
        let menu_items = tray.menu();

        // 5 Aktions-Items + 1 Checkmark + 2 Separatoren = 8 Eintraege
        assert_eq!(menu_items.len(), 8);
    }

    #[test]
    fn test_tray_tooltip_content() {
        let (tx, _rx) = crossbeam_channel::unbounded();
        let tray = PillTray::new(tx);
        let tip = tray.tool_tip();

        let s_buffy_title = tip.title;
        assert!(s_buffy_title.contains("Whisper Pill"));
        assert!(s_buffy_title.contains("Super+Strg+P"));
        assert_eq!(tip.description, "Minimalistisches Desktop-Diktier-Utility");
        assert_eq!(tip.icon_pixmap.len(), 2);
    }

    #[test]
    fn test_tray_transcribing_state_updates() {
        let (tx, _rx) = crossbeam_channel::unbounded();
        let mut tray = PillTray::new(tx);

        assert!(!tray.is_transcribing);
        assert_eq!(tray.title(), "Whisper Pill");
        assert_eq!(tray.icon_name(), "whisper-pill");
        assert!(tray.tool_tip().title.contains("Super+Strg+P"));

        tray.set_transcribing(true);
        assert!(tray.is_transcribing);
        assert_eq!(tray.title(), "Whisper Pill (Transkribiert...)");
        assert_eq!(tray.icon_name(), "whisper-pill-transcribing");
        if let MenuItem::Standard(ref item) = tray.menu()[0] {
            assert!(item.label.contains("Transkription"));
        } else {
            panic!("Erstes Menue-Element sollte ein Standard-Item sein");
        }
    }
}
