//! whisper-pill: Minimalistisches, schwebendes Desktop-Diktier-Utility.
//!
//! Haupteinsprungpunkt mit Slint UI Framework: Realisiert die schwebende Kapsel,
//! die Audio-Aufnahme, Sprachpausenerkennung (VAD), Whisper-Transkription,
//! globale KDE-Hotkeys (Super+Strg+P) und System-Tray-Integration.

slint::include_modules!();

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use whisper_pill::audio::recorder::{AudioEvent, AudioRecorder};
use whisper_pill::config::AppConfig;
use whisper_pill::platform::clipboard::ClipboardManager;
use whisper_pill::platform::hotkey::HotkeyManager;
use whisper_pill::transcription::engine::WhisperEngine;

#[cfg(target_os = "linux")]
use whisper_pill::platform::tray::{PillTray, TrayAction};

/// Steuerungsbefehle an den dedizierten Audio-Aufnahme-Thread.
enum AudioCommand {
    Start,
    Stop,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Konfiguration und Plattform-Dienste laden
    let config = AppConfig::default();
    let clipboard = Arc::new(ClipboardManager::new());
    let engine = Arc::new(WhisperEngine::new(&config.model_path));

    // 2. Slint-Fenster instanziieren und Standardgroesse (480x48 px) setzen
    let app_window = AppWindow::new()?;
    app_window
        .window()
        .set_size(slint::LogicalSize::new(480.0, 48.0));

    // 3. Status-Variablen fuer Aufnahme, Sichtbarkeit und Audio-Daten
    let is_recording = Arc::new(AtomicBool::new(false));
    let is_window_visible = Arc::new(AtomicBool::new(true));
    let accumulated_audio = Arc::new(std::sync::Mutex::new(Vec::<f32>::new()));
    let recording_start = Arc::new(std::sync::Mutex::new(None::<Instant>));

    // 4. Kanaele fuer Audio-Worker und Events
    let (audio_tx, audio_rx) = crossbeam_channel::unbounded();
    let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded::<AudioCommand>();
    let (samples_tx, samples_rx) = crossbeam_channel::unbounded::<Vec<f32>>();

    // 5. Dedizierter Audio-Aufnahme-Thread (besitzt cpal AudioRecorder exklusiv)
    {
        let silence_rms = config.silence_threshold_rms;
        let silence_timeout = config.silence_timeout_secs;
        let audio_tx_clone = audio_tx.clone();

        std::thread::Builder::new()
            .name("whisper-audio-worker".to_string())
            .spawn(move || {
                let mut recorder = AudioRecorder::new(silence_rms, silence_timeout);
                recorder.set_event_sender(audio_tx_clone);

                while let Ok(cmd) = cmd_rx.recv() {
                    match cmd {
                        AudioCommand::Start => {
                            let _ = recorder.start();
                        }
                        AudioCommand::Stop => {
                            let samples = recorder.stop();
                            let _ = samples_tx.send(samples);
                        }
                    }
                }
            })
            .expect("Audio Worker Thread konnte nicht gestartet werden");
    }

    // 6. Globaler Hotkey (Super+Strg+P) initialisieren
    let (hotkey_tx, hotkey_rx) = crossbeam_channel::unbounded();
    let hotkey_manager = HotkeyManager::new(hotkey_tx);
    hotkey_manager.start_listener();

    // 7. System-Tray-Icon (KDE StatusNotifierItem) initialisieren
    #[cfg(target_os = "linux")]
    let (tray_tx, tray_rx) = crossbeam_channel::unbounded::<TrayAction>();
    #[cfg(target_os = "linux")]
    {
        let tray = PillTray::new(tray_tx);
        tray.spawn_in_thread();
    }

    // --- Helfer: Aufnahme starten ---
    let start_rec_fn = {
        let cmd_tx = cmd_tx.clone();
        let is_recording = Arc::clone(&is_recording);
        let recording_start = Arc::clone(&recording_start);
        let window_weak = app_window.as_weak();

        move || {
            if !is_recording.load(Ordering::SeqCst) {
                let _ = cmd_tx.send(AudioCommand::Start);
                is_recording.store(true, Ordering::SeqCst);
                *recording_start.lock().unwrap() = Some(Instant::now());

                let _ = window_weak.upgrade_in_event_loop(|win| {
                    win.set_current_state(1); // Recording
                    win.set_is_expanded(false);
                    win.window().set_size(slint::LogicalSize::new(480.0, 48.0));
                    win.set_timer_string("00:00".into());
                });
            }
        }
    };

    // --- Helfer: Aufnahme stoppen und Transkription anstossen ---
    let stop_rec_fn = {
        let cmd_tx = cmd_tx.clone();
        let is_recording = Arc::clone(&is_recording);
        let recording_start = Arc::clone(&recording_start);
        let window_weak = app_window.as_weak();

        move || {
            if is_recording.load(Ordering::SeqCst) {
                let _ = cmd_tx.send(AudioCommand::Stop);
                is_recording.store(false, Ordering::SeqCst);
                *recording_start.lock().unwrap() = None;

                let _ = window_weak.upgrade_in_event_loop(|win| {
                    win.set_current_state(2); // Processing
                });
            }
        }
    };

    // --- Hintergrund-Thread: Samples empfangen und Whisper-Inferenz durchfuehren ---
    {
        let accumulated_audio = Arc::clone(&accumulated_audio);
        let engine = Arc::clone(&engine);
        let clipboard = Arc::clone(&clipboard);
        let lang = config.default_language.clone();
        let window_weak = app_window.as_weak();

        std::thread::Builder::new()
            .name("whisper-inference-worker".to_string())
            .spawn(move || {
                while let Ok(samples) = samples_rx.recv() {
                    let mut all_samples = accumulated_audio.lock().unwrap();
                    all_samples.extend_from_slice(&samples);

                    if all_samples.is_empty() {
                        let _ = window_weak.upgrade_in_event_loop(|win| {
                            win.set_current_state(0); // Idle
                        });
                        continue;
                    }

                    let samples_to_transcribe = std::mem::take(&mut *all_samples);

                    let result = engine.transcribe(&samples_to_transcribe, lang.as_deref());

                    // Fluechtigen Audiospeicher nach Inferenz leeren
                    let mut mut_samples = samples_to_transcribe;
                    for s in mut_samples.iter_mut() {
                        *s = 0.0;
                    }
                    mut_samples.clear();

                    let clipboard_clone = Arc::clone(&clipboard);
                    let _ = window_weak.upgrade_in_event_loop(move |win| match result {
                        Ok(text) => {
                            let _ = clipboard_clone.set_text(&text);
                            win.set_transcript_string(text.into());
                            win.set_current_state(3); // Ready
                        }
                        Err(err) => {
                            eprintln!("Transkriptionsfehler: {}", err);
                            win.set_current_state(0); // Idle
                        }
                    });
                }
            })
            .expect("Inference Worker Thread konnte nicht gestartet werden");
    }

    // --- Slint Callbacks verbinden ---
    {
        let start_fn = start_rec_fn.clone();
        app_window.on_start_recording(move || {
            start_fn();
        });
    }

    {
        let stop_fn = stop_rec_fn.clone();
        app_window.on_stop_recording(move || {
            stop_fn();
        });
    }

    {
        let start_fn = start_rec_fn.clone();
        app_window.on_resume_recording(move || {
            start_fn();
        });
    }

    {
        let window_weak = app_window.as_weak();
        app_window.on_toggle_details(move || {
            if let Some(win) = window_weak.upgrade() {
                let current = win.get_is_expanded();
                let next = !current;
                win.set_is_expanded(next);
                let h = if next { 240.0 } else { 48.0 };
                win.window().set_size(slint::LogicalSize::new(480.0, h));
            }
        });
    }

    {
        let clipboard = Arc::clone(&clipboard);
        let window_weak = app_window.as_weak();
        app_window.on_copy_text(move || {
            if let Some(win) = window_weak.upgrade() {
                let s_buffy_text = win.get_transcript_string();
                let _ = clipboard.set_text(&s_buffy_text);
            }
        });
    }

    {
        let window_weak = app_window.as_weak();
        let is_window_visible = Arc::clone(&is_window_visible);
        app_window.on_dismiss(move || {
            is_window_visible.store(false, Ordering::SeqCst);
            if let Some(win) = window_weak.upgrade() {
                win.set_current_state(0); // Idle
                win.set_is_expanded(false);
                win.window().set_size(slint::LogicalSize::new(480.0, 48.0));
                let _ = win.hide();
            }
        });
    }

    app_window.on_quit_app(|| {
        std::process::exit(0);
    });

    // --- Hintergrund-Thread: Hotkey-Events verarbeiten (Super+Strg+P) ---
    {
        let is_recording = Arc::clone(&is_recording);
        let is_window_visible = Arc::clone(&is_window_visible);
        let start_fn = start_rec_fn.clone();
        let stop_fn = stop_rec_fn.clone();
        let window_weak = app_window.as_weak();

        std::thread::Builder::new()
            .name("whisper-hotkey-handler".to_string())
            .spawn(move || {
                while let Ok(_event) = hotkey_rx.recv() {
                    if is_window_visible.load(Ordering::SeqCst) {
                        // Fenster sichtbar: Aufnahme beenden (falls aktiv) und Pill ausblenden
                        if is_recording.load(Ordering::SeqCst) {
                            stop_fn();
                        }
                        is_window_visible.store(false, Ordering::SeqCst);
                        let _ = window_weak.upgrade_in_event_loop(|win| {
                            let _ = win.hide();
                        });
                    } else {
                        // Fenster unsichtbar: Pill einblenden und Aufnahme sofort starten
                        is_window_visible.store(true, Ordering::SeqCst);
                        let _ = window_weak.upgrade_in_event_loop(|win| {
                            win.set_is_expanded(false);
                            win.window().set_size(slint::LogicalSize::new(480.0, 48.0));
                            let _ = win.show();
                        });
                        start_fn();
                    }
                }
            })
            .expect("Hotkey-Handler Thread konnte nicht gestartet werden");
    }

    // --- Hintergrund-Thread: System-Tray Events verarbeiten ---
    #[cfg(target_os = "linux")]
    {
        let is_recording = Arc::clone(&is_recording);
        let is_window_visible = Arc::clone(&is_window_visible);
        let start_fn = start_rec_fn.clone();
        let stop_fn = stop_rec_fn.clone();
        let window_weak = app_window.as_weak();

        std::thread::Builder::new()
            .name("whisper-tray-handler".to_string())
            .spawn(move || {
                while let Ok(action) = tray_rx.recv() {
                    match action {
                        TrayAction::Toggle => {
                            if is_window_visible.load(Ordering::SeqCst) {
                                if is_recording.load(Ordering::SeqCst) {
                                    stop_fn();
                                }
                                is_window_visible.store(false, Ordering::SeqCst);
                                let _ = window_weak.upgrade_in_event_loop(|win| {
                                    let _ = win.hide();
                                });
                            } else {
                                is_window_visible.store(true, Ordering::SeqCst);
                                let _ = window_weak.upgrade_in_event_loop(|win| {
                                    win.set_is_expanded(false);
                                    win.window().set_size(slint::LogicalSize::new(480.0, 48.0));
                                    let _ = win.show();
                                });
                                start_fn();
                            }
                        }
                        TrayAction::Quit => {
                            std::process::exit(0);
                        }
                    }
                }
            })
            .expect("Tray-Handler Thread konnte nicht gestartet werden");
    }

    // --- Hintergrund-Thread: Audio-Events (RMS Pegel & Silence Timeout) verarbeiten ---
    {
        let stop_fn = stop_rec_fn.clone();
        let window_weak = app_window.as_weak();

        std::thread::Builder::new()
            .name("whisper-audio-events".to_string())
            .spawn(move || {
                while let Ok(event) = audio_rx.recv() {
                    match event {
                        AudioEvent::RmsLevel(rms) => {
                            let scaled = (rms * 16.0).clamp(0.0, 1.0);
                            let base_height = 4.0f32;
                            let max_extra = 24.0f32;

                            // Glockenkurve fuer die 11 Balken
                            let factors = [0.35, 0.55, 0.75, 0.90, 1.0, 1.05, 1.0, 0.90, 0.75, 0.55, 0.35];

                            let _ = window_weak.upgrade_in_event_loop(move |win| {
                                win.set_bar0(base_height + max_extra * scaled * factors[0]);
                                win.set_bar1(base_height + max_extra * scaled * factors[1]);
                                win.set_bar2(base_height + max_extra * scaled * factors[2]);
                                win.set_bar3(base_height + max_extra * scaled * factors[3]);
                                win.set_bar4(base_height + max_extra * scaled * factors[4]);
                                win.set_bar5(base_height + max_extra * scaled * factors[5]);
                                win.set_bar6(base_height + max_extra * scaled * factors[6]);
                                win.set_bar7(base_height + max_extra * scaled * factors[7]);
                                win.set_bar8(base_height + max_extra * scaled * factors[8]);
                                win.set_bar9(base_height + max_extra * scaled * factors[9]);
                                win.set_bar10(base_height + max_extra * scaled * factors[10]);
                            });
                        }
                        AudioEvent::SilenceTimeout => {
                            stop_fn();
                        }
                        AudioEvent::Started => {}
                        AudioEvent::Error(err) => {
                            eprintln!("Audio-Fehler: {}", err);
                        }
                    }
                }
            })
            .expect("Audio-Event Thread konnte nicht gestartet werden");
    }

    // --- Slint Timer fuer die Zeitanzeige waehrend der Aufnahme ---
    let timer = slint::Timer::default();
    {
        let is_recording = Arc::clone(&is_recording);
        let recording_start = Arc::clone(&recording_start);
        let window_weak = app_window.as_weak();

        timer.start(
            slint::TimerMode::Repeated,
            Duration::from_millis(500),
            move || {
                if is_recording.load(Ordering::SeqCst) {
                    if let Some(start) = *recording_start.lock().unwrap() {
                        let i_buffy_secs = start.elapsed().as_secs();
                        let s_buffy_time = format!(
                            "{:02}:{:02}",
                            i_buffy_secs / 60,
                            i_buffy_secs % 60
                        );
                        if let Some(win) = window_weak.upgrade() {
                            win.set_timer_string(s_buffy_time.into());
                        }
                    }
                }
            },
        );
    }

    // 8. Fenster anzeigen und sofortige Aufnahme beim Start (One-Touch Voice Workflow)
    app_window.show()?;
    start_rec_fn();

    // 9. Slint Event-Loop ausfuehren (bleibt aktiv, selbst wenn das Fenster ausgeblendet ist)
    slint::run_event_loop_until_quit()?;

    Ok(())
}
