//! Mikrofon-Aufnahme und Audio-Pufferung via cpal.
//!
//! Liest PCM-Audiodaten vom Standard-Mikrofon, konvertiert Kanaele und Abtastrate
//! auf 16 kHz Mono (f32) und speichert diese fluechtig im Arbeitsspeicher.

use crate::audio::vad::SilenceDetector;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use crossbeam_channel::Sender;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Ereignisse, die vom Audio-Recorder an den Haupt-UI-Thread gesendet werden.
#[derive(Debug, Clone)]
pub enum AudioEvent {
    /// Aktueller Lautstaerke-Pegel (RMS) fuer die animierte UI-Waveform (0.0 bis 1.0).
    RmsLevel(f32),
    /// Automatische Stille-Erkennung hat ausgeloest (Auto-Stopp).
    SilenceTimeout,
    /// Aufnahme wurde erfolgreich gestartet.
    Started,
    /// Ein Audio-Hardwarefehler ist aufgetreten.
    Error(String),
}

/// Verwaltet die Audioaufnahme vom Systemsound-Treiber.
pub struct AudioRecorder {
    /// Aktiver cpal Audio-Eingabestrom.
    stream: Option<Stream>,
    /// Thread-sicherer Puffer fuer gesammelte 16-kHz-Mono-Samples.
    audio_buffer: Arc<Mutex<Vec<f32>>>,
    /// Stille-Erkennung fuer automatisches Beenden bei Sprechpausen.
    silence_detector: Arc<Mutex<SilenceDetector>>,
    /// Flag, ob die Aufnahme aktuell laeuft.
    is_recording: Arc<AtomicBool>,
    /// Sender fuer Status- und Lautstaerke-Events an die UI.
    event_sender: Option<Sender<AudioEvent>>,
}

impl AudioRecorder {
    /// Erstellt eine neue Instanz des Audio-Recorders.
    pub fn new(silence_threshold_rms: f32, silence_timeout_secs: f32) -> Self {
        let detector = SilenceDetector::new(silence_threshold_rms, silence_timeout_secs, 16_000);

        Self {
            stream: None,
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
            silence_detector: Arc::new(Mutex::new(detector)),
            is_recording: Arc::new(AtomicBool::new(false)),
            event_sender: None,
        }
    }

    /// Weist einen Event-Kanal fuer Lautstaerke- und Statusrueckmeldungen zu.
    pub fn set_event_sender(&mut self, sender: Sender<AudioEvent>) {
        self.event_sender = Some(sender);
    }

    /// Startet die Audioaufnahme vom Standard-Mikrofon.
    pub fn start(&mut self) -> Result<(), String> {
        if self.is_recording.load(Ordering::SeqCst) {
            return Ok(());
        }

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| "Kein Standard-Mikrofon am System gefunden.".to_string())?;

        let supported_config = device
            .default_input_config()
            .map_err(|e| format!("Eingabekonfiguration konnte nicht ermittelt werden: {}", e))?;

        let channels = supported_config.channels() as usize;
        let native_sample_rate = supported_config.sample_rate().0;

        let config: StreamConfig = supported_config.into();

        let buffer_clone = Arc::clone(&self.audio_buffer);
        let detector_clone = Arc::clone(&self.silence_detector);
        let is_rec_clone = Arc::clone(&self.is_recording);
        let event_tx = self.event_sender.clone();

        // Stille-Detektor fuer die neue Aufnahme zuruecksetzen
        {
            let mut det = detector_clone.lock().unwrap();
            det.reset();
        }

        self.is_recording.store(true, Ordering::SeqCst);

        let err_fn = {
            let tx = event_tx.clone();
            move |err| {
                if let Some(ref sender) = tx {
                    let _ = sender.send(AudioEvent::Error(format!("Audio-Stream-Fehler: {}", err)));
                }
            }
        };

        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if !is_rec_clone.load(Ordering::SeqCst) {
                        return;
                    }

                    // 1. Mehrkanal-Audio (z. B. Stereo) zu Mono heruntermischen
                    let mono_samples = Self::downmix_to_mono(data, channels);

                    // 2. Auf Ziel-Abtastrate 16 kHz resamplen
                    let resampled = Self::resample_linear(&mono_samples, native_sample_rate, 16_000);

                    // 3. RMS-Pegel berechnen und Stille-Status pruefen
                    let mut det = detector_clone.lock().unwrap();
                    let (is_timeout, current_rms) = det.process_chunk(&resampled);

                    // 4. Samples an den RAM-Puffer anhaengen
                    let mut buf = buffer_clone.lock().unwrap();
                    buf.extend_from_slice(&resampled);

                    // 5. Lautstaerke-Event und ggf. Timeout an die UI melden
                    if let Some(ref sender) = event_tx {
                        let _ = sender.send(AudioEvent::RmsLevel(current_rms));
                        if is_timeout {
                            let _ = sender.send(AudioEvent::SilenceTimeout);
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| format!("Audio-Stream konnte nicht aufgebaut werden: {}", e))?;

        stream
            .play()
            .map_err(|e| format!("Audio-Stream konnte nicht gestartet werden: {}", e))?;

        self.stream = Some(stream);

        if let Some(ref sender) = self.event_sender {
            let _ = sender.send(AudioEvent::Started);
        }

        Ok(())
    }

    /// Beendet die Aufnahme und liefert die gesammelten 16-kHz-Audiosamples.
    pub fn stop(&mut self) -> Vec<f32> {
        self.is_recording.store(false, Ordering::SeqCst);
        self.stream = None;

        let mut buf = self.audio_buffer.lock().unwrap();
        let samples = buf.clone();
        // Fluechtigen Speicher leeren
        buf.clear();
        samples
    }

    /// Haelt die Aufnahme an, ohne den bisherigen Puffer zu loeschen (fuer Fortsetzen).
    pub fn pause(&mut self) {
        self.is_recording.store(false, Ordering::SeqCst);
        self.stream = None;
    }

    /// Ueberschreibt den Audiospeicher zu Sicherheits- und Datenschutzzwecken mit Nullen.
    pub fn flush_memory(&mut self) {
        let mut buf = self.audio_buffer.lock().unwrap();
        for sample in buf.iter_mut() {
            *sample = 0.0;
        }
        buf.clear();
    }

    /// Gibt an, ob aktuell Audio aufgenommen wird.
    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }

    /// Wandelt Mehrkanal-Audiodaten durch Durchschnittsbildung in Monosignale um.
    fn downmix_to_mono(interleaved_samples: &[f32], channel_count: usize) -> Vec<f32> {
        if channel_count <= 1 {
            return interleaved_samples.to_vec();
        }

        let frame_count = interleaved_samples.len() / channel_count;
        let mut mono_samples = Vec::with_capacity(frame_count);

        for i_buffy_frame in 0..frame_count {
            let mut sum = 0.0f32;
            let offset = i_buffy_frame * channel_count;
            for i_buffy_channel in 0..channel_count {
                sum += interleaved_samples[offset + i_buffy_channel];
            }
            mono_samples.push(sum / channel_count as f32);
        }

        mono_samples
    }

    /// Fuehrt ein lineares Resampling von der Quellrate auf die Zielrate durch.
    fn resample_linear(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
        if from_rate == to_rate || samples.is_empty() {
            return samples.to_vec();
        }

        let ratio = from_rate as f64 / to_rate as f64;
        let target_len = ((samples.len() as f64) / ratio).floor() as usize;
        let mut output = Vec::with_capacity(target_len);

        for i_buffy_target in 0..target_len {
            let source_pos = i_buffy_target as f64 * ratio;
            let index_low = source_pos.floor() as usize;
            let index_high = (index_low + 1).min(samples.len() - 1);
            let fraction = (source_pos - index_low as f64) as f32;

            let interpolated =
                samples[index_low] * (1.0 - fraction) + samples[index_high] * fraction;
            output.push(interpolated);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downmix_to_mono() {
        // Stereo-Signal: [L1, R1, L2, R2]
        let stereo = vec![1.0f32, 0.0f32, 0.5f32, 0.5f32];
        let mono = AudioRecorder::downmix_to_mono(&stereo, 2);
        assert_eq!(mono.len(), 2);
        assert_eq!(mono[0], 0.5);
        assert_eq!(mono[1], 0.5);
    }

    #[test]
    fn test_resample_linear() {
        // 48 kHz nach 16 kHz (Verhaeltnis 3:1)
        let samples_48k = vec![0.0f32, 0.33f32, 0.66f32, 1.0f32, 0.66f32, 0.33f32];
        let samples_16k = AudioRecorder::resample_linear(&samples_48k, 48_000, 16_000);
        assert_eq!(samples_16k.len(), 2);
    }
}
