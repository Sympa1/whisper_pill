//! Konfigurationsverwaltung fuer die whisper-pill Desktop-Anwendung.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Zentrale Konfigurationsstruktur fuer whisper-pill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Pfad zum lokalen Whisper GGML-Modell (z. B. ggml-base.bin).
    pub model_path: PathBuf,

    /// Standardsprache fuer die Transkription (z. B. "de" oder None fuer Auto-Erkennung).
    pub default_language: Option<String>,

    /// Schwellenwert fuer den RMS-Lautstaerkepegel, unter dem ein Frame als Stille gilt.
    pub silence_threshold_rms: f32,

    /// Dauer in Sekunden anhaltender Stille, nach der die Aufnahme automatisch stoppt.
    pub silence_timeout_secs: f32,

    /// Audio-Abtastrate in Hertz (fest auf 16000 Hz fuer Whisper).
    pub sample_rate: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let default_model = PathBuf::from(home_dir)
            .join(".cache")
            .join("whisper_pill")
            .join("ggml-base.bin");

        Self {
            model_path: default_model,
            default_language: Some("de".to_string()),
            silence_threshold_rms: 0.012,
            silence_timeout_secs: 2.5,
            sample_rate: 16_000,
        }
    }
}
