//! Stille- und Sprachaktivitaetserkennung (VAD - Voice Activity Detection).
//!
//! Berechnet den RMS-Pegel eingehender Audio-Samples und erkennt anhaltende
//! Sprechpausen fuer den automatischen Aufnahme-Stopp.

/// Erkennt Sprechpausen und Stillephasen im Audiosignal.
#[derive(Debug, Clone)]
pub struct SilenceDetector {
    /// Schwellenwert fuer den Effektivwert (RMS), unter dem ein Signal als Stille gilt.
    silence_threshold_rms: f32,
    /// Benoetigte Stille-Dauer in Sekunden, bevor der automatische Stopp ausloest.
    silence_timeout_secs: f32,
    /// Abtastrate in Hertz (z. B. 16000 Hz).
    sample_rate: u32,
    /// Zaehler fuer aufeinanderfolgende Stille-Samples.
    i_buffy_silence_samples: usize,
    /// Zaehler fuer die Gesamtzahl verarbeiteter Samples seit Aufnahmebeginn.
    i_buffy_total_samples: usize,
    /// Mindestanzahl an Audio-Samples, bevor Stille-Timeout greifen darf (z. B. 0.5s Sprache).
    min_record_samples: usize,
    /// Markiert, ob waehrend der aktuellen Sitzung bereits Sprache erkannt wurde.
    has_speech_started: bool,
}

impl SilenceDetector {
    /// Erstellt eine neue Instanz des Stille-Detektors mit den angegebenen Grenzwerten.
    pub fn new(silence_threshold_rms: f32, silence_timeout_secs: f32, sample_rate: u32) -> Self {
        // Mindestens 0.5 Sekunden Audio abwarten, um Fehlstarts vor erstem Sprechen zu vermeiden
        let min_samples = (sample_rate as f32 * 0.5) as usize;

        Self {
            silence_threshold_rms,
            silence_timeout_secs,
            sample_rate,
            i_buffy_silence_samples: 0,
            i_buffy_total_samples: 0,
            min_record_samples: min_samples,
            has_speech_started: false,
        }
    }

    /// Setzt alle internen Puffer und Zaehler auf den Ursprungszustand zurueck.
    pub fn reset(&mut self) {
        self.i_buffy_silence_samples = 0;
        self.i_buffy_total_samples = 0;
        self.has_speech_started = false;
    }

    /// Berechnet den Effektivwert (Root Mean Square) eines Audio-Sample-Puffers.
    pub fn compute_rms(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }

        let mut sum_squared = 0.0f32;
        for &sample in samples {
            sum_squared += sample * sample;
        }

        (sum_squared / samples.len() as f32).sqrt()
    }

    /// Verarbeitet einen Block neuer Samples, aktualisiert die Zaehler und liefert (is_timeout, current_rms).
    pub fn process_chunk(&mut self, samples: &[f32]) -> (bool, f32) {
        let current_rms = Self::compute_rms(samples);
        let chunk_len = samples.len();
        self.i_buffy_total_samples += chunk_len;

        // Pruefen, ob die aktuelle Lautstaerke ueber dem Schwellenwert liegt (aktive Sprache)
        if current_rms >= self.silence_threshold_rms {
            self.has_speech_started = true;
            // Bei aktiver Sprache den Stille-Puffer wieder nullen
            self.i_buffy_silence_samples = 0;
        } else if self.has_speech_started {
            // Nur akkumulieren, wenn vorher bereits gesprochen wurde
            self.i_buffy_silence_samples += chunk_len;
        }

        // Benoetigte Sampleanzahl fuer das konfigurierte Timeout
        let max_silence_samples = (self.sample_rate as f32 * self.silence_timeout_secs) as usize;

        // Auto-Stopp ausloesen, wenn Sprache erkannt wurde, Mindestlaenge erreicht ist und Timeout greift
        let is_timeout = self.has_speech_started
            && self.i_buffy_total_samples >= self.min_record_samples
            && self.i_buffy_silence_samples >= max_silence_samples;

        (is_timeout, current_rms)
    }

    /// Gibt zurueck, ob bereits Sprache in der aktuellen Aufnahme erkannt wurde.
    pub fn has_speech_started(&self) -> bool {
        self.has_speech_started
    }

    /// Liefert die aktuelle Dauer der ununterbrochenen Stille in Sekunden.
    pub fn current_silence_duration_secs(&self) -> f32 {
        self.i_buffy_silence_samples as f32 / self.sample_rate as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_rms_silence() {
        // Bei reinen Nullen muss der RMS-Pegel exakt 0.0 sein
        let samples = vec![0.0f32; 1600];
        let rms = SilenceDetector::compute_rms(&samples);
        assert_eq!(rms, 0.0);
    }

    #[test]
    fn test_compute_rms_active_signal() {
        // Bei konstanter Amplitude muss der RMS-Pegel dem Betrag entsprechen
        let samples = vec![0.5f32; 1600];
        let rms = SilenceDetector::compute_rms(&samples);
        assert!((rms - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_silence_detection_timeout_flow() {
        // 16 kHz, Schwellenwert 0.01, Timeout 1.0 Sekunde
        let mut detector = SilenceDetector::new(0.01, 1.0, 16_000);

        // 1. Initial noch kein Sprechen: Stille soll noch keinen Timeout ausloesen
        let silence_chunk = vec![0.001f32; 1600]; // 0.1s Stille
        let (timeout, _) = detector.process_chunk(&silence_chunk);
        assert!(!timeout);
        assert!(!detector.has_speech_started());

        // 2. Sprache beginnt (0.6s aktive Sprache)
        let speech_chunk = vec![0.05f32; 9600];
        let (timeout, _) = detector.process_chunk(&speech_chunk);
        assert!(!timeout);
        assert!(detector.has_speech_started());

        // 3. Stille folgt: Nach 1.0s (10 Chunks à 1600 Samples) muss Timeout ausloesen
        let mut triggered = false;
        for _ in 0..10 {
            let (is_t, _) = detector.process_chunk(&silence_chunk);
            if is_t {
                triggered = true;
                break;
            }
        }
        assert!(triggered, "Timeout haette nach 1.0s Stille ausloesen muessen");
    }
}
