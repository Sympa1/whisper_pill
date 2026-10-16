//! Transkriptions-Engine basierend auf whisper-rs / whisper.cpp.
//!
//! Fuehrt die lokale Spracherkennung mit Whisper-GGML-Modellen durch,
//! unterstuetzt Sprachauswahl, Audio-Normalisierung (Gain), Beam-Search
//! sowie deutsche Initial-Prompts fuer optimale Erkennung bei undeutlicher Sprache.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Normalisiert Audiosamples auf einen optimalen Zielpegel (ca. 0.95 Peak, max 5x Gain),
/// um leises oder genuscheltes Sprechen optimal fuer Whisper verstaendlich zu machen.
pub fn normalize_audio_samples(samples: &[f32]) -> Vec<f32> {
    if samples.is_empty() {
        return Vec::new();
    }

    let mut max_abs: f32 = 0.0;
    for &s in samples {
        let abs_val = s.abs();
        if abs_val > max_abs {
            max_abs = abs_val;
        }
    }

    // Bei Stille oder extrem geringem Grundrauschen nicht hochskalieren
    if max_abs < 0.001 {
        return samples.to_vec();
    }

    // Skalierungsfaktor mit Obergrenze (max 5.0x), um Rauschen nicht ueberzubetonen
    let scale = (0.95 / max_abs).min(5.0);
    samples
        .iter()
        .map(|&s| (s * scale).clamp(-1.0, 1.0))
        .collect()
}

/// Kapselt den Whisper-Inferenz-Kontext und die Thread-sichere Ausfuehrung.
pub struct WhisperEngine {
    /// Thread-sicherer Pfad zum Modell (erlaubt Modellwechsel zur Laufzeit).
    model_path: Mutex<PathBuf>,
    /// Whisper-Kontext zur Modellausfuehrung.
    context: Mutex<Option<WhisperContext>>,
}

impl WhisperEngine {
    /// Erstellt eine neue Transkriptions-Engine mit dem angegebenen Modellpfad.
    pub fn new<P: AsRef<Path>>(model_path: P) -> Self {
        Self {
            model_path: Mutex::new(model_path.as_ref().to_path_buf()),
            context: Mutex::new(None),
        }
    }

    /// Aktualisiert den Modellpfad und invalidiert den Kontext (fuer Modellwechsel base <-> small).
    pub fn set_model_path<P: AsRef<Path>>(&self, new_path: P) {
        let new_buf = new_path.as_ref().to_path_buf();
        let mut path_guard = self.model_path.lock().unwrap();
        if *path_guard != new_buf {
            *path_guard = new_buf;
            let mut ctx_guard = self.context.lock().unwrap();
            *ctx_guard = None; // Altes Modell aus dem RAM entladen
        }
    }

    /// Stellt sicher, dass das Modell existiert (strikte Zero-Download-Pruefung).
    pub fn ensure_model_available(&self) -> Result<(), String> {
        let path = self.model_path.lock().unwrap().clone();
        if path.exists() {
            return Ok(());
        }

        Err(format!(
            "Whisper-Modell nicht gefunden unter: '{}'. Bitte legen Sie die Modelldatei lokal ab.",
            path.display()
        ))
    }

    /// Laedt das GGML-Modell in den Speicher, falls noch nicht geschehen.
    pub fn load_model(&self) -> Result<(), String> {
        let mut ctx_guard = self.context.lock().map_err(|e| e.to_string())?;
        if ctx_guard.is_some() {
            return Ok(());
        }

        // Sicherstellen, dass die Datei existiert
        self.ensure_model_available()?;

        let current_path = self.model_path.lock().unwrap().clone();
        let s_buffy_path = current_path
            .to_str()
            .ok_or_else(|| "Ungueltiger Modellpfad (kein valider UTF-8 Pfad)".to_string())?;

        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(s_buffy_path, params)
            .map_err(|e| format!("Fehler beim Initialisieren des Whisper-Modells: {}", e))?;

        *ctx_guard = Some(ctx);
        Ok(())
    }

    /// Transkribiert 16-kHz-Mono-Audiosamples zu Text.
    pub fn transcribe(
        &self,
        audio_samples: &[f32],
        language: Option<&str>,
        beam_search: bool,
        normalize: bool,
    ) -> Result<String, String> {
        if audio_samples.is_empty() {
            return Ok(String::new());
        }

        // 1. Audio-Pegel-Normalisierung (Gain fuer leises/genuscheltes Sprechen)
        let s_buffy_normalized: Vec<f32>;
        let final_samples: &[f32] = if normalize {
            s_buffy_normalized = normalize_audio_samples(audio_samples);
            &s_buffy_normalized
        } else {
            audio_samples
        };

        // 2. Sicherstellen, dass das Modell im RAM bereitsteht
        self.load_model()?;

        let mut ctx_guard = self.context.lock().map_err(|e| e.to_string())?;
        let ctx = ctx_guard
            .as_mut()
            .ok_or_else(|| "Whisper-Kontext ist nicht geladen.".to_string())?;

        let mut state = ctx
            .create_state()
            .map_err(|e| format!("Whisper-State konnte nicht erstellt werden: {}", e))?;

        // 3. Decoding-Strategie konfigurieren: Greedy (schnell) vs. Beam Search (genauer)
        let strategy = if beam_search {
            SamplingStrategy::BeamSearch {
                beam_size: 5,
                patience: -1.0,
            }
        } else {
            SamplingStrategy::Greedy { best_of: 1 }
        };

        let mut params = FullParams::new(strategy);
        params.set_language(language);

        // Deutscher Initial-Prompt: Weist das Modell auf deutsche Sprache hin (ohne Vorgaben zu Grammatik oder Rechtschreibung)
        params.set_initial_prompt("Hallo, dies ist ein deutsches Diktat.");

        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_translate(false);

        // 4. Inferenz auf den Samples starten
        state
            .full(params, final_samples)
            .map_err(|e| format!("Inferenz-Fehler waehrend der Transkription: {}", e))?;

        // 5. Segmente ueber den Iterator auslesen und zusammenfuegen
        let mut s_buffy_result = String::new();
        for segment in state.as_iter() {
            if let Ok(segment_text) = segment.to_str() {
                s_buffy_result.push_str(segment_text);
            }
        }

        // Fuehrende und nachfolgende Leerzeichen bereinigen
        Ok(s_buffy_result.trim().to_string())
    }

    /// Gibt zurueck, ob das Modell auf der Festplatte existiert.
    pub fn is_model_available(&self) -> bool {
        self.model_path.lock().unwrap().exists()
    }

    /// Liefert den aktuellen Pfad zum Modell.
    pub fn model_path(&self) -> PathBuf {
        self.model_path.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_initialization() {
        let engine = WhisperEngine::new("/tmp/non_existent_model.bin");
        assert!(!engine.is_model_available());
        assert_eq!(engine.model_path(), PathBuf::from("/tmp/non_existent_model.bin"));
    }

    #[test]
    fn test_set_model_path_updates_path() {
        let engine = WhisperEngine::new("/tmp/model_base.bin");
        assert_eq!(engine.model_path(), PathBuf::from("/tmp/model_base.bin"));

        engine.set_model_path("/tmp/model_small.bin");
        assert_eq!(engine.model_path(), PathBuf::from("/tmp/model_small.bin"));
    }

    #[test]
    fn test_normalize_audio_samples_scaling() {
        // Leises Signal mit Peak 0.1
        let quiet_signal = vec![0.05, -0.1, 0.08, -0.04];
        let normalized = normalize_audio_samples(&quiet_signal);

        // Peak muss deutlich verstaerkt worden sein
        let max_val = normalized.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(max_val > 0.4, "Leises Signal muss skaliert werden");
        assert!(max_val <= 1.0, "Signal darf 1.0 nicht ueberschreiten");
    }

    #[test]
    fn test_normalize_audio_samples_silence_preserved() {
        let silence = vec![0.0, 0.00001, -0.00001, 0.0];
        let normalized = normalize_audio_samples(&silence);
        assert_eq!(silence, normalized, "Stille darf nicht hochskaliert werden");
    }

    #[test]
    fn test_ensure_model_available_offline_error() {
        let engine = WhisperEngine::new("/tmp/strictly_missing_ggml.bin");
        let res = engine.ensure_model_available();
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Whisper-Modell nicht gefunden"));
    }

    #[test]
    fn test_cached_model_detection() {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let cached = PathBuf::from(home).join(".cache/whisper_pill/ggml-base.bin");
        if cached.exists() {
            let engine = WhisperEngine::new(&cached);
            assert!(engine.is_model_available());
        }
    }
}
