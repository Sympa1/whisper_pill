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
use whisper_pill::transcription::append_transcript;
use whisper_pill::transcription::engine::WhisperEngine;

#[cfg(target_os = "linux")]
use whisper_pill::platform::tray::{PillTray, TrayAction};

/// Steuerungsbefehle an den dedizierten Audio-Aufnahme-Thread.
enum AudioCommand {
    Start,
    Stop,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Befehlszeilenparameter pruefen (--version, --help, --capture-screenshots)
    let s_args: Vec<String> = std::env::args().collect();
    if s_args.iter().any(|arg| arg == "--version" || arg == "-v") {
        println!("Whisper Pill v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if s_args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("Whisper Pill v{} - Schlankes, minimalistisches Desktop-Diktier-Utility", env!("CARGO_PKG_VERSION"));
        println!("Verwendung: whisper_pill [OPTIONEN]");
        println!();
        println!("Optionen:");
        println!("  -h, --help                         Zeigt diese Hilfe an");
        println!("  -v, --version                      Zeigt die Versionsnummer an");
        println!("  --capture-screenshots [VERZEICHNIS] Erzeugt UI-Screenshots aller Kapselzustaende");
        return Ok(());
    }
    if let Some(i_buffy_idx) = s_args.iter().position(|arg| arg == "--capture-screenshots") {
        let s_buffy_out_dir = s_args
            .get(i_buffy_idx + 1)
            .cloned()
            .unwrap_or_else(|| "docs/assets".to_string());
        return run_screenshot_capture(std::path::Path::new(&s_buffy_out_dir));
    }

    // 0. Fehler- und Diagnoseprotokollierung initialisieren
    whisper_pill::logging::init_log();
    whisper_pill::logging::log_info("System", "Whisper Pill erfolgreich gestartet.");

    // 1. Konfiguration und Plattform-Dienste laden (inkl. persistenter Fensterposition)
    let config = AppConfig::load();

    // Falls Autostart konfiguriert ist, sicherstellen, dass die .desktop-Datei gueltig existiert
    if config.autostart {
        if !whisper_pill::platform::autostart::is_autostart_enabled() {
            let _ = whisper_pill::platform::autostart::set_autostart(true);
        } else if let Ok(s_buffy_current_exe) = whisper_pill::platform::autostart::get_persistent_executable_path() {
            // Falls sich der Pfad geaendert hat (z. B. von /tmp/.mount_* auf echtes AppImage), aktualisieren
            let autostart_file = whisper_pill::platform::autostart::autostart_desktop_path();
            if let Ok(s_buffy_content) = std::fs::read_to_string(&autostart_file)
                && !s_buffy_content.contains(&s_buffy_current_exe)
            {
                let _ = whisper_pill::platform::autostart::set_autostart(true);
            }
        }
    }

    // KWin-Fensterregeln sicherstellen (verhindert Taskleisten-Icons und Alt+Tab Erscheinen unter KDE)
    whisper_pill::platform::kwin::ensure_kwin_window_rules();

    let clipboard = Arc::new(ClipboardManager::new());
    let engine = Arc::new(WhisperEngine::new(config.resolved_model_path()));
    let beam_search = Arc::new(AtomicBool::new(config.beam_search));
    let normalize_audio = Arc::new(AtomicBool::new(config.normalize_audio));

    // 2. Slint-Fenster instanziieren und Standardgroesse (520x48 px) setzen
    let app_window = AppWindow::new()?;
    let base_avail = AppConfig::is_model_installed("base");
    let small_avail = AppConfig::is_model_installed("small");
    app_window.set_base_model_available(base_avail);
    app_window.set_small_model_available(small_avail);
    app_window.set_autostart_enabled(whisper_pill::platform::autostart::is_autostart_enabled());
    app_window.set_selected_model(config.model_name.clone().into());
    app_window.set_normalize_audio_enabled(config.normalize_audio);
    app_window.set_beam_search_enabled(config.beam_search);
    app_window.set_is_settings(false);
    app_window.set_is_expanded(false);

    app_window
        .window()
        .set_size(slint::LogicalSize::new(520.0, 48.0));

    // Gespeicherte Fensterposition aus config.json wiederherstellen
    if let (Some(x), Some(y)) = (config.window_x, config.window_y) {
        app_window
            .window()
            .set_position(slint::PhysicalPosition::new(x, y));
    }

    // Gespeicherte Position der kompakten Pill (wird vor dem Öffnen der Einstellungen gesichert)
    let saved_pill_pos = Arc::new(std::sync::Mutex::new(
        config.window_x.and_then(|x| config.window_y.map(|y| slint::PhysicalPosition::new(x, y))),
    ));

    // Automatischer Modell-Download beim Programmstart, falls das ausgewählte Modell noch nicht lokal existiert
    let initial_model = config.model_name.clone();
    if !AppConfig::is_model_installed(&initial_model) {
        let engine_download_clone = Arc::clone(&engine);
        let window_weak = app_window.as_weak();
        let model_name_log = initial_model.clone();

        whisper_pill::transcription::downloader::spawn_background_download(
            initial_model,
            move |res| match res {
                Ok(new_path) => {
                    let s_buffy_msg = format!(
                        "Modell '{}' erfolgreich heruntergeladen und einsatzbereit.",
                        model_name_log
                    );
                    eprintln!("Whisper Pill: {}", s_buffy_msg);
                    whisper_pill::logging::log_info("Downloader", &s_buffy_msg);
                    engine_download_clone.set_model_path(&new_path);
                    let _ = window_weak.upgrade_in_event_loop(move |win| {
                        win.set_base_model_available(AppConfig::is_model_installed("base"));
                        win.set_small_model_available(AppConfig::is_model_installed("small"));
                    });
                }
                Err(err) => {
                    let s_buffy_err = format!("Fehler beim automatischen Download von Modell '{}': {}", model_name_log, err);
                    eprintln!("Whisper Pill: {}", s_buffy_err);
                    whisper_pill::logging::log_error("Downloader", &s_buffy_err);
                }
            },
        );
    }

    // Positions-Persistierung: Speichert veraenderte Koordinaten automatisch in config.json
    let last_saved_pos = Arc::new(std::sync::Mutex::new((config.window_x, config.window_y)));
    let save_pos_fn = {
        let last_saved_pos = Arc::clone(&last_saved_pos);
        move |pos: slint::PhysicalPosition| {
            if pos.x != 0 || pos.y != 0 {
                let mut last_pos = last_saved_pos.lock().unwrap();
                if *last_pos != (Some(pos.x), Some(pos.y)) {
                    *last_pos = (Some(pos.x), Some(pos.y));
                    let mut current_config = AppConfig::load();
                    current_config.window_x = Some(pos.x);
                    current_config.window_y = Some(pos.y);
                    let _ = current_config.save();
                }
            }
        }
    };

    // 3. Status-Variablen fuer Aufnahme, Sichtbarkeit und Audio-Daten
    // Hinweis: Die Pill startet im Hintergrund verborgen (nur System-Tray-Icon sichtbar)
    let is_recording = Arc::new(AtomicBool::new(false));
    let is_continuation = Arc::new(AtomicBool::new(false));
    let is_window_visible = Arc::new(AtomicBool::new(false));
    let accumulated_audio = Arc::new(std::sync::Mutex::new(Vec::<f32>::new()));
    let recording_start = Arc::new(std::sync::Mutex::new(None::<Instant>));
    let ready_entered_at = Arc::new(std::sync::Mutex::new(None::<Instant>));

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
    let tray_handle = {
        let tray = PillTray::new(tray_tx);
        tray.spawn_in_thread()
    };

    // --- Helfer: Aufnahme starten ---
    let start_rec_fn = {
        let cmd_tx = cmd_tx.clone();
        let is_recording = Arc::clone(&is_recording);
        let recording_start = Arc::clone(&recording_start);
        let ready_entered_at = Arc::clone(&ready_entered_at);
        let window_weak = app_window.as_weak();
        #[cfg(target_os = "linux")]
        let tray_handle_start = tray_handle.clone();

        move || {
            if !is_recording.load(Ordering::SeqCst) {
                *ready_entered_at.lock().unwrap() = None;
                let _ = cmd_tx.send(AudioCommand::Start);
                is_recording.store(true, Ordering::SeqCst);
                *recording_start.lock().unwrap() = Some(Instant::now());

                #[cfg(target_os = "linux")]
                tray_handle_start.set_transcribing(false);

                let _ = window_weak.upgrade_in_event_loop(|win| {
                    win.set_current_state(1); // Recording
                    win.set_is_expanded(false);
                    win.window().set_size(slint::LogicalSize::new(520.0, 48.0));
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
        #[cfg(target_os = "linux")]
        let tray_handle_stop = tray_handle.clone();

        move || {
            if is_recording.load(Ordering::SeqCst) {
                let _ = cmd_tx.send(AudioCommand::Stop);
                is_recording.store(false, Ordering::SeqCst);
                *recording_start.lock().unwrap() = None;

                #[cfg(target_os = "linux")]
                tray_handle_stop.set_transcribing(true);

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
        let is_continuation = Arc::clone(&is_continuation);
        let ready_entered_at_worker = Arc::clone(&ready_entered_at);
        let beam_search_worker = Arc::clone(&beam_search);
        let normalize_audio_worker = Arc::clone(&normalize_audio);
        let lang = config.default_language.clone();
        let window_weak = app_window.as_weak();
        #[cfg(target_os = "linux")]
        let tray_handle_worker = tray_handle.clone();

        std::thread::Builder::new()
            .name("whisper-inference-worker".to_string())
            .spawn(move || {
                while let Ok(samples) = samples_rx.recv() {
                    let mut all_samples = accumulated_audio.lock().unwrap();
                    all_samples.extend_from_slice(&samples);

                    if all_samples.is_empty() {
                        #[cfg(target_os = "linux")]
                        tray_handle_worker.set_transcribing(false);

                        let _ = window_weak.upgrade_in_event_loop(|win| {
                            win.set_current_state(0); // Idle
                        });
                        continue;
                    }

                    let samples_to_transcribe = std::mem::take(&mut *all_samples);

                    let use_beam = beam_search_worker.load(Ordering::SeqCst);
                    let use_norm = normalize_audio_worker.load(Ordering::SeqCst);
                    let result = engine.transcribe(
                        &samples_to_transcribe,
                        lang.as_deref(),
                        use_beam,
                        use_norm,
                    );

                    // Fluechtigen Audiospeicher nach Inferenz leeren
                    let mut mut_samples = samples_to_transcribe;
                    for s in mut_samples.iter_mut() {
                        *s = 0.0;
                    }
                    mut_samples.clear();

                    let clipboard_clone = Arc::clone(&clipboard);
                    let is_continuation_clone = Arc::clone(&is_continuation);
                    let ready_entered_at_done = Arc::clone(&ready_entered_at_worker);
                    #[cfg(target_os = "linux")]
                    let tray_handle_done = tray_handle_worker.clone();

                    let _ = window_weak.upgrade_in_event_loop(move |win| {
                        #[cfg(target_os = "linux")]
                        tray_handle_done.set_transcribing(false);

                        match result {
                            Ok(s_buffy_raw_text) => {
                                // Text bei Fortsetzung (+ Weiter) anhaengen oder bei Neu-Aufnahme ersetzen
                                let s_buffy_final_text = if is_continuation_clone.load(Ordering::SeqCst) {
                                    let s_buffy_existing = win.get_transcript_string().to_string();
                                    append_transcript(&s_buffy_existing, &s_buffy_raw_text)
                                } else {
                                    s_buffy_raw_text.trim().to_string()
                                };

                                let _ = clipboard_clone.set_text(&s_buffy_final_text);
                                win.set_transcript_string(s_buffy_final_text.into());
                                win.set_current_state(3); // Ready
                                *ready_entered_at_done.lock().unwrap() = Some(Instant::now());
                                is_continuation_clone.store(false, Ordering::SeqCst);
                            }
                            Err(err) => {
                                eprintln!("Transkriptionsfehler: {}", err);
                                whisper_pill::logging::log_error("Transkription", &err);
                                win.set_current_state(0); // Idle
                                *ready_entered_at_done.lock().unwrap() = None;
                                is_continuation_clone.store(false, Ordering::SeqCst);
                            }
                        }
                    });
                }
            })
            .expect("Inference Worker Thread konnte nicht gestartet werden");
    }

    // --- Slint Callbacks verbinden ---
    {
        let start_fn = start_rec_fn.clone();
        let is_continuation_clone = Arc::clone(&is_continuation);
        let window_weak = app_window.as_weak();
        app_window.on_start_recording(move || {
            is_continuation_clone.store(false, Ordering::SeqCst);
            if let Some(win) = window_weak.upgrade() {
                win.set_transcript_string("".into());
            }
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
        // Fortsetzen: Bisherigen Text beibehalten und neues Segment erweitern
        let start_fn = start_rec_fn.clone();
        let is_continuation_clone = Arc::clone(&is_continuation);
        app_window.on_resume_recording(move || {
            is_continuation_clone.store(true, Ordering::SeqCst);
            start_fn();
        });
    }

    {
        // Neu-Aufnahme: Text-Puffer zuruecksetzen und von vorne aufnehmen
        let start_fn = start_rec_fn.clone();
        let is_continuation_clone = Arc::clone(&is_continuation);
        let window_weak = app_window.as_weak();
        app_window.on_new_recording(move || {
            is_continuation_clone.store(false, Ordering::SeqCst);
            if let Some(win) = window_weak.upgrade() {
                win.set_transcript_string("".into());
            }
            start_fn();
        });
    }

    {
        let window_weak = app_window.as_weak();
        let ready_entered_at_details = Arc::clone(&ready_entered_at);
        app_window.on_toggle_details(move || {
            *ready_entered_at_details.lock().unwrap() = None;
            if let Some(win) = window_weak.upgrade() {
                let current = win.get_is_expanded();
                let next = !current;
                win.set_is_expanded(next);
                let h = if next { 240.0 } else { 48.0 };
                win.window().set_size(slint::LogicalSize::new(520.0, h));
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
        let ready_entered_at_dismiss = Arc::clone(&ready_entered_at);
        let save_pos = save_pos_fn.clone();
        #[cfg(target_os = "linux")]
        let tray_handle_dismiss = tray_handle.clone();

        app_window.on_dismiss(move || {
            *ready_entered_at_dismiss.lock().unwrap() = None;
            #[cfg(target_os = "linux")]
            tray_handle_dismiss.set_transcribing(false);

            is_window_visible.store(false, Ordering::SeqCst);
            if let Some(win) = window_weak.upgrade() {
                if !win.get_is_settings() {
                    save_pos(win.window().position());
                }
                win.set_current_state(0); // Idle
                win.set_is_expanded(false);
                win.window().set_size(slint::LogicalSize::new(520.0, 48.0));
                let _ = win.hide();
            }
        });
    }

    {
        let window_weak = app_window.as_weak();
        let save_pos = save_pos_fn.clone();
        app_window.on_quit_app(move || {
            if let Some(win) = window_weak.upgrade()
                && !win.get_is_settings()
            {
                save_pos(win.window().position());
            }
            let _ = slint::quit_event_loop();
        });
    }

    // --- Hintergrund-Thread: Hotkey-Events verarbeiten (Super+Strg+P) ---
    {
        let is_recording = Arc::clone(&is_recording);
        let is_continuation = Arc::clone(&is_continuation);
        let is_window_visible = Arc::clone(&is_window_visible);
        let saved_pill_pos = Arc::clone(&saved_pill_pos);
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
                        let saved_pos_clone = Arc::clone(&saved_pill_pos);
                        let _ = window_weak.upgrade_in_event_loop(move |win| {
                            win.set_is_settings(false);
                            win.set_is_expanded(false);
                            win.window().set_size(slint::LogicalSize::new(520.0, 48.0));
                            if let Some(pos) = *saved_pos_clone.lock().unwrap() {
                                win.window().set_position(pos);
                            }
                            let _ = win.hide();
                        });
                    } else {
                        // Fenster unsichtbar: Pill einblenden und Aufnahme sofort starten
                        is_continuation.store(false, Ordering::SeqCst);
                        is_window_visible.store(true, Ordering::SeqCst);
                        let saved_pos_clone = Arc::clone(&saved_pill_pos);
                        let _ = window_weak.upgrade_in_event_loop(move |win| {
                            win.set_is_settings(false);
                            win.set_is_expanded(false);
                            win.window().set_size(slint::LogicalSize::new(520.0, 48.0));
                            if let Some(pos) = *saved_pos_clone.lock().unwrap() {
                                win.window().set_position(pos);
                            }
                            let _ = win.show();
                            whisper_pill::platform::kwin::ensure_x11_window_properties();
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
        let is_continuation = Arc::clone(&is_continuation);
        let is_window_visible = Arc::clone(&is_window_visible);
        let saved_pill_pos = Arc::clone(&saved_pill_pos);
        let start_fn = start_rec_fn.clone();
        let stop_fn = stop_rec_fn.clone();
        let window_weak = app_window.as_weak();

        let save_pos = save_pos_fn.clone();

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
                                let saved_pos_clone = Arc::clone(&saved_pill_pos);
                                let _ = window_weak.upgrade_in_event_loop(move |win| {
                                    win.set_is_settings(false);
                                    win.set_is_expanded(false);
                                    win.window().set_size(slint::LogicalSize::new(520.0, 48.0));
                                    if let Some(pos) = *saved_pos_clone.lock().unwrap() {
                                        win.window().set_position(pos);
                                    }
                                    let _ = win.hide();
                                });
                            } else {
                                is_continuation.store(false, Ordering::SeqCst);
                                is_window_visible.store(true, Ordering::SeqCst);
                                let saved_pos_clone = Arc::clone(&saved_pill_pos);
                                let _ = window_weak.upgrade_in_event_loop(move |win| {
                                    win.set_is_settings(false);
                                    win.set_is_expanded(false);
                                    win.window().set_size(slint::LogicalSize::new(520.0, 48.0));
                                    if let Some(pos) = *saved_pos_clone.lock().unwrap() {
                                        win.window().set_position(pos);
                                    }
                                    let _ = win.show();
                                    whisper_pill::platform::kwin::ensure_x11_window_properties();
                                });
                                start_fn();
                            }
                        }
                        TrayAction::Settings => {
                            let current_cfg = AppConfig::load();
                            is_window_visible.store(true, Ordering::SeqCst);
                            let saved_pos_clone = Arc::clone(&saved_pill_pos);
                            let _ = window_weak.upgrade_in_event_loop(move |win| {
                                // 1. Aktuelle Position der Pill sichern
                                let cur_pos = win.window().position();
                                *saved_pos_clone.lock().unwrap() = Some(cur_pos);

                                // 2. Modell-Verfügbarkeit auf der Festplatte prüfen
                                let base_avail = AppConfig::is_model_installed("base");
                                let small_avail = AppConfig::is_model_installed("small");
                                win.set_base_model_available(base_avail);
                                win.set_small_model_available(small_avail);

                                win.set_autostart_enabled(whisper_pill::platform::autostart::is_autostart_enabled());
                                win.set_selected_model(current_cfg.model_name.into());
                                win.set_normalize_audio_enabled(current_cfg.normalize_audio);
                                win.set_beam_search_enabled(current_cfg.beam_search);
                                win.set_is_expanded(false);
                                win.set_is_settings(true);

                                // 3. Großzügige Fenstergröße für Einstellungen (580x480 px)
                                win.window().set_size(slint::LogicalSize::new(580.0, 480.0));

                                // 4. Intelligente Verschiebung nach oben und zentrierter Versatz
                                let adjusted_x = (cur_pos.x - 30).max(10);
                                let adjusted_y = if cur_pos.y > 100 {
                                    (cur_pos.y - 432).max(40)
                                } else {
                                    cur_pos.y
                                };
                                win.window().set_position(slint::PhysicalPosition::new(adjusted_x, adjusted_y));

                                let _ = win.show();
                                whisper_pill::platform::kwin::ensure_x11_window_properties();
                            });
                        }
                        TrayAction::OpenLog => {
                            whisper_pill::logging::open_log_file();
                        }
                        TrayAction::Quit => {
                            let _ = window_weak.upgrade_in_event_loop({
                                let save_pos = save_pos.clone();
                                move |win| {
                                    if !win.get_is_settings() {
                                        save_pos(win.window().position());
                                    }
                                    let _ = slint::quit_event_loop();
                                }
                            });
                        }
                    }
                }
            })
            .expect("Tray-Handler Thread konnte nicht gestartet werden");
    }

    // --- Settings Callbacks verbinden ---
    {
        let window_weak = app_window.as_weak();
        let is_window_visible = Arc::clone(&is_window_visible);
        let saved_pos_clone = Arc::clone(&saved_pill_pos);
        app_window.on_cancel_settings(move || {
            is_window_visible.store(false, Ordering::SeqCst);
            if let Some(win) = window_weak.upgrade() {
                win.set_is_settings(false);
                win.window().set_size(slint::LogicalSize::new(520.0, 48.0));
                if let Some(pos) = *saved_pos_clone.lock().unwrap() {
                    win.window().set_position(pos);
                }
                let _ = win.hide();
            }
        });
    }

    {
        let window_weak = app_window.as_weak();
        let is_window_visible = Arc::clone(&is_window_visible);
        let engine_clone = Arc::clone(&engine);
        let beam_search_clone = Arc::clone(&beam_search);
        let normalize_audio_clone = Arc::clone(&normalize_audio);
        let saved_pos_clone = Arc::clone(&saved_pill_pos);

        app_window.on_save_settings(move |autostart_flag, model_choice, norm_flag, beam_flag| {
            // 1. Autostart einrichten bzw. vollstaendig entfernen
            let _ = whisper_pill::platform::autostart::set_autostart(autostart_flag);

            // 2. Flags fuer Audio-Worker aktualisieren
            beam_search_clone.store(beam_flag, Ordering::SeqCst);
            normalize_audio_clone.store(norm_flag, Ordering::SeqCst);

            // 3. Konfiguration auf Festplatte sichern
            let mut current_cfg = AppConfig::load();
            current_cfg.model_name = model_choice.to_string();
            current_cfg.beam_search = beam_flag;
            current_cfg.normalize_audio = norm_flag;
            current_cfg.autostart = autostart_flag;
            current_cfg.model_path = current_cfg.resolved_model_path();
            let _ = current_cfg.save();

            // 4. Modell in der Transkriptions-Engine dynamisch aktualisieren
            engine_clone.set_model_path(&current_cfg.model_path);

            // 5. Falls das gewählte Modell noch nicht lokal existiert: Hintergrund-Download anstoßen
            if !AppConfig::is_model_installed(&current_cfg.model_name) {
                let engine_download_clone = Arc::clone(&engine_clone);
                let window_weak_download = window_weak.clone();
                let model_name_log = current_cfg.model_name.clone();

                whisper_pill::transcription::downloader::spawn_background_download(
                    current_cfg.model_name.clone(),
                    move |res| match res {
                        Ok(new_path) => {
                            let s_buffy_msg = format!(
                                "Modell '{}' erfolgreich heruntergeladen und aktiv.",
                                model_name_log
                            );
                            eprintln!("Whisper Pill: {}", s_buffy_msg);
                            whisper_pill::logging::log_info("Downloader", &s_buffy_msg);
                            engine_download_clone.set_model_path(&new_path);
                            let _ = window_weak_download.upgrade_in_event_loop(move |win| {
                                win.set_base_model_available(AppConfig::is_model_installed("base"));
                                win.set_small_model_available(AppConfig::is_model_installed("small"));
                            });
                        }
                        Err(err) => {
                            let s_buffy_err = format!("Fehler beim Download von Modell '{}': {}", model_name_log, err);
                            eprintln!("Whisper Pill: {}", s_buffy_err);
                            whisper_pill::logging::log_error("Downloader", &s_buffy_err);
                        }
                    },
                );
            }

            // 6. Fenster zuruecksetzen, ursprüngliche Position wiederherstellen und verbergen
            is_window_visible.store(false, Ordering::SeqCst);
            if let Some(win) = window_weak.upgrade() {
                win.set_is_settings(false);
                win.window().set_size(slint::LogicalSize::new(520.0, 48.0));
                if let Some(pos) = *saved_pos_clone.lock().unwrap() {
                    win.window().set_position(pos);
                }
                let _ = win.hide();
            }
        });
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
                            whisper_pill::logging::log_error("Audio", &err);
                        }
                    }
                }
            })
            .expect("Audio-Event Thread konnte nicht gestartet werden");
    }

    // --- Slint Timer fuer Zeitanzeige, Positions-Aktualisierung und Auto-Dismiss ---
    let timer = slint::Timer::default();
    {
        let is_recording = Arc::clone(&is_recording);
        let is_window_visible = Arc::clone(&is_window_visible);
        let recording_start = Arc::clone(&recording_start);
        let ready_entered_at = Arc::clone(&ready_entered_at);
        let window_weak = app_window.as_weak();
        let save_pos = save_pos_fn.clone();

        timer.start(
            slint::TimerMode::Repeated,
            Duration::from_millis(500),
            move || {
                if let Some(win) = window_weak.upgrade() {
                    // Position nur sichern, wenn die Pill sichtbar ist und NICHT im Einstellungsmodus
                    if is_window_visible.load(Ordering::SeqCst) && !win.get_is_settings() {
                        save_pos(win.window().position());
                    }

                    // Auto-Dismiss fuer Zustand 3 (Ready): Nach 8 Sekunden Inaktivitaet automatisch verbergen,
                    // damit die Pill nicht dauerhaft auf dem Desktop verbleibt und Dropdown-Terminals (Yakuake)
                    // oder Startmenues (Super-Taste) ueberdeckt.
                    if win.get_current_state() == 3 && !win.get_is_expanded() && !win.get_is_settings() {
                        let should_hide = if let Some(entered) = *ready_entered_at.lock().unwrap() {
                            entered.elapsed().as_secs() >= 8
                        } else {
                            false
                        };
                        if should_hide {
                            *ready_entered_at.lock().unwrap() = None;
                            win.set_current_state(0); // Zurueck zu Idle
                            is_window_visible.store(false, Ordering::SeqCst);
                            let _ = win.hide();
                        }
                    }

                    if is_recording.load(Ordering::SeqCst)
                        && let Some(start) = *recording_start.lock().unwrap()
                    {
                        let i_buffy_secs = start.elapsed().as_secs();
                        let s_buffy_time = format!(
                            "{:02}:{:02}",
                            i_buffy_secs / 60,
                            i_buffy_secs % 60
                        );
                        win.set_timer_string(s_buffy_time.into());
                    }
                }
            },
        );
    }

    // 8. Slint Event-Loop ausfuehren (die Pill laeuft im Hintergrund und reagiert auf Tray/Hotkey)
    slint::run_event_loop_until_quit()?;

    Ok(())
}

/// Startet die Slint-Oberflaeche im automatisierten Screenshot-Modus, durchlaeuft alle
/// Kernzustaende der Rust-App und speichert sie als transparente PNG-Dateien.
fn run_screenshot_capture(out_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(out_dir)?;

    let app_window = AppWindow::new()?;
    app_window.set_is_settings(false);
    app_window.set_is_expanded(false);
    app_window.set_current_state(0);
    app_window
        .window()
        .set_size(slint::LogicalSize::new(520.0, 48.0));
    app_window.show()?;

    let i_buffy_step = std::rc::Rc::new(std::cell::Cell::new(0u32));
    let out_path_buf = out_dir.to_path_buf();
    let window_weak = app_window.as_weak();

    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(280),
        move || {
            let Some(win) = window_weak.upgrade() else {
                let _ = slint::quit_event_loop();
                return;
            };
            let i_buffy_cur = i_buffy_step.get();
            i_buffy_step.set(i_buffy_cur + 1);

            match i_buffy_cur {
                0 => {
                    // Zustand 0 (Idle) ist gerendert -> speichern und auf Zustand 1 (Recording) schalten
                    if let Ok(buf) = win.window().take_snapshot() {
                        let _ = save_rgba_as_png(
                            &out_path_buf.join("state_0_idle.png"),
                            buf.width(),
                            buf.height(),
                            buf.as_bytes(),
                        );
                    }
                    win.set_current_state(1);
                    win.set_timer_string("00:04".into());
                    win.set_bar0(6.0);
                    win.set_bar1(10.0);
                    win.set_bar2(16.0);
                    win.set_bar3(22.0);
                    win.set_bar4(26.0);
                    win.set_bar5(28.0);
                    win.set_bar6(24.0);
                    win.set_bar7(18.0);
                    win.set_bar8(12.0);
                    win.set_bar9(8.0);
                    win.set_bar10(5.0);
                }
                1 => {
                    // Zustand 1 (Recording) speichern und auf Zustand 3 (Ready) schalten
                    if let Ok(buf) = win.window().take_snapshot() {
                        let _ = save_rgba_as_png(
                            &out_path_buf.join("state_1_recording.png"),
                            buf.width(),
                            buf.height(),
                            buf.as_bytes(),
                        );
                    }
                    win.set_current_state(3);
                }
                2 => {
                    // Zustand 3 (Ready) speichern und auf Zustand 4 (Expanded) schalten
                    if let Ok(buf) = win.window().take_snapshot() {
                        let _ = save_rgba_as_png(
                            &out_path_buf.join("state_2_ready.png"),
                            buf.width(),
                            buf.height(),
                            buf.as_bytes(),
                        );
                    }
                    win.set_current_state(4);
                    win.set_is_expanded(true);
                    win.set_transcript_string(
                        "Dies ist ein lokal transkribierter Diktat-Text der nativen Rust-App.\n\
                         Der erkannte Text liegt sofort in der Zwischenablage bereit und kann \
                         hier im ausklappbaren Editor direkt geprüft oder angepasst werden."
                            .into(),
                    );
                    win.window().set_size(slint::LogicalSize::new(520.0, 240.0));
                }
                3 => {
                    // Zustand 4 (Expanded) speichern und auf Einstellungs-Dialog schalten
                    if let Ok(buf) = win.window().take_snapshot() {
                        let _ = save_rgba_as_png(
                            &out_path_buf.join("state_3_expanded.png"),
                            buf.width(),
                            buf.height(),
                            buf.as_bytes(),
                        );
                    }
                    win.set_is_expanded(false);
                    win.set_is_settings(true);
                    win.set_base_model_available(true);
                    win.set_small_model_available(true);
                    win.set_selected_model("base".into());
                    win.set_beam_search_enabled(true);
                    win.set_normalize_audio_enabled(true);
                    win.set_autostart_enabled(true);
                    win.window().set_size(slint::LogicalSize::new(580.0, 480.0));
                }
                4 => {
                    // Einstellungs-Dialog speichern und beenden
                    if let Ok(buf) = win.window().take_snapshot() {
                        let _ = save_rgba_as_png(
                            &out_path_buf.join("state_4_settings.png"),
                            buf.width(),
                            buf.height(),
                            buf.as_bytes(),
                        );
                    }
                    println!(
                        "[Screenshot-Generator] Rust-UI-Screenshots erfolgreich in '{}' gespeichert.",
                        out_path_buf.display()
                    );
                    let _ = slint::quit_event_loop();
                }
                _ => {
                    let _ = slint::quit_event_loop();
                }
            }
        },
    );

    slint::run_event_loop_until_quit()?;
    Ok(())
}

/// Schreibt einen RGBA8-Pixelpuffer ohne externe Crate-Abhaengigkeiten als standardkonforme PNG-Datei.
fn save_rgba_as_png(
    path: &std::path::Path,
    width: u32,
    height: u32,
    rgba: &[u8],
) -> std::io::Result<()> {
    let mut out = Vec::new();
    // PNG-Signatur
    out.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);

    // IHDR-Chunk (Farbtyp 6 = RGBA, 8 Bit Farbtiefe)
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    write_png_chunk(&mut out, b"IHDR", &ihdr);

    // Roh-Scanlines mit Filterbyte 0 (None) pro Zeile aufbauen
    let i_buffy_row_bytes = (width as usize) * 4;
    let mut raw_data = Vec::with_capacity((i_buffy_row_bytes + 1) * (height as usize));
    for i_buffy_y in 0..(height as usize) {
        raw_data.push(0u8);
        let i_buffy_start = i_buffy_y * i_buffy_row_bytes;
        raw_data.extend_from_slice(&rgba[i_buffy_start..i_buffy_start + i_buffy_row_bytes]);
    }

    // Zlib-Stream mit unkomprimierten Deflate-Bloecken erzeugen
    let mut zlib = Vec::new();
    zlib.extend_from_slice(&[0x78, 0x01]);
    let chunks: Vec<&[u8]> = raw_data.chunks(65535).collect();
    for (i_buffy_idx, block) in chunks.iter().enumerate() {
        let is_last = i_buffy_idx + 1 == chunks.len();
        zlib.push(if is_last { 0x01 } else { 0x00 });
        let i_buffy_len = block.len() as u16;
        zlib.extend_from_slice(&i_buffy_len.to_le_bytes());
        zlib.extend_from_slice(&(!i_buffy_len).to_le_bytes());
        zlib.extend_from_slice(block);
    }
    zlib.extend_from_slice(&compute_adler32(&raw_data).to_be_bytes());
    write_png_chunk(&mut out, b"IDAT", &zlib);

    // IEND-Chunk
    write_png_chunk(&mut out, b"IEND", &[]);
    std::fs::write(path, out)
}

/// Fuegt einen einzelnen PNG-Chunk inklusive Laenge und CRC32-Pruefsumme an den Ausgabepuffer an.
fn write_png_chunk(out: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(chunk_type);
    out.extend_from_slice(data);
    let mut i_buffy_crc: u32 = 0xFFFF_FFFF;
    for &b in chunk_type.iter().chain(data.iter()) {
        i_buffy_crc ^= b as u32;
        for _ in 0..8 {
            if i_buffy_crc & 1 != 0 {
                i_buffy_crc = (i_buffy_crc >> 1) ^ 0xEDB8_8320;
            } else {
                i_buffy_crc >>= 1;
            }
        }
    }
    out.extend_from_slice(&(!i_buffy_crc).to_be_bytes());
}

/// Berechnet die Adler-32-Pruefsumme fuer den Zlib-Datenstrom eines PNGs.
fn compute_adler32(data: &[u8]) -> u32 {
    let mut i_buffy_a: u32 = 1;
    let mut i_buffy_b: u32 = 0;
    for &byte in data {
        i_buffy_a = (i_buffy_a + byte as u32) % 65521;
        i_buffy_b = (i_buffy_b + i_buffy_a) % 65521;
    }
    (i_buffy_b << 16) | i_buffy_a
}

