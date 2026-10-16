//! Konfigurationsverwaltung fuer die whisper-pill Desktop-Anwendung.
//!
//! Verwaltet Einstellungen wie Modellpfade, Erkennungsschwellenwerte sowie
//! die persistente Fensterposition des Pill-Overlays auf dem Bildschirm.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Zentrale Konfigurationsstruktur fuer whisper-pill.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AppConfig {
    /// Pfad zum lokalen Whisper GGML-Modell (z. B. ggml-base.bin oder ggml-small.bin).
    pub model_path: PathBuf,

    /// Modellbezeichnung: "base" (Standard) oder "small" (Genauer bei Nuscheln).
    pub model_name: String,

    /// Erkennungs-Modus: false = Schnell (Greedy), true = Gruendlich (Beam Search).
    pub beam_search: bool,

    /// Automatische Audio-Pegelanpassung (Gain) fuer leise oder undeutliche Sprache.
    pub normalize_audio: bool,

    /// Standardsprache fuer die Transkription (z. B. "de" oder None fuer Auto-Erkennung).
    pub default_language: Option<String>,

    /// Schwellenwert fuer den RMS-Lautstaerkepegel, unter dem ein Frame als Stille gilt.
    pub silence_threshold_rms: f32,

    /// Dauer in Sekunden anhaltender Stille, nach der die Aufnahme automatisch stoppt.
    pub silence_timeout_secs: f32,

    /// Audio-Abtastrate in Hertz (fest auf 16000 Hz fuer Whisper).
    pub sample_rate: u32,

    /// Automatischer Systemstart bei der Benutzeranmeldung.
    pub autostart: bool,

    /// Gespeicherte X-Position des Pill-Fensters auf dem Bildschirm.
    pub window_x: Option<i32>,

    /// Gespeicherte Y-Position des Pill-Fensters auf dem Bildschirm.
    pub window_y: Option<i32>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let default_model = PathBuf::from(s_buffy_home)
            .join(".cache")
            .join("whisper_pill")
            .join("ggml-small.bin");

        Self {
            model_path: default_model,
            model_name: "small".to_string(), // Beste Erkennungsgenauigkeit bei Nuscheln
            beam_search: true,               // Gründliche Beam Search Analyse als Standard
            normalize_audio: true,           // Leise/genuschelte Sprache vor Inferenz anheben
            autostart: true,                 // Systemstart per Default aktiviert
            default_language: Some("de".to_string()),
            silence_threshold_rms: 0.012,
            silence_timeout_secs: 2.5,
            sample_rate: 16_000,
            window_x: None,
            window_y: None,
        }
    }
}

impl AppConfig {
    /// Ermittelt den Pfad zur Konfigurationsdatei config.json.
    pub fn config_path() -> PathBuf {
        if let Ok(s_buffy_xdg) = std::env::var("XDG_CONFIG_HOME")
            && !s_buffy_xdg.is_empty()
        {
            return PathBuf::from(s_buffy_xdg)
                .join("whisper_pill")
                .join("config.json");
        }
        let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(s_buffy_home)
            .join(".config")
            .join("whisper_pill")
            .join("config.json")
    }

    /// Laedt die Konfiguration aus der Datei oder erzeugt Standardwerte, falls keine existiert.
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists()
            && let Ok(s_buffy_content) = std::fs::read_to_string(&path)
            && let Ok(config) = serde_json::from_str::<AppConfig>(&s_buffy_content)
        {
            return config;
        }
        Self::default()
    }

    /// Prueft, ob ein konkretes GGML-Modell ("base" oder "small") lokal im Cache existiert.
    pub fn is_model_installed(model_name: &str) -> bool {
        let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let file_name = if model_name == "small" {
            "ggml-small.bin"
        } else {
            "ggml-base.bin"
        };
        PathBuf::from(s_buffy_home)
            .join(".cache")
            .join("whisper_pill")
            .join(file_name)
            .exists()
    }

    /// Liefert den direkten, unmodifizierten Modellpfad für den konfigurierten Modellnamen.
    pub fn raw_model_path(&self) -> PathBuf {
        let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let file_name = if self.model_name == "small" {
            "ggml-small.bin"
        } else {
            "ggml-base.bin"
        };
        PathBuf::from(s_buffy_home)
            .join(".cache")
            .join("whisper_pill")
            .join(file_name)
    }

    /// Liefert den passenden Modellpfad mit intelligentem Fallback.
    /// Falls das gewünschte Modell (z. B. "small") lokal nicht im Cache existiert,
    /// wird automatisch ein vorhandenes Modell (z. B. "base") gewählt, damit die Inferenz
    /// niemals an einer fehlenden Datei scheitert.
    pub fn resolved_model_path(&self) -> PathBuf {
        let raw_path = self.raw_model_path();

        // 1. Bevorzugtes Modell existiert direkt
        if raw_path.exists() {
            return raw_path;
        }

        // 2. Automatischer Fallback auf alternatives Modell (base <-> small)
        let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let cache_dir = PathBuf::from(&s_buffy_home).join(".cache").join("whisper_pill");
        let fallback_file = if self.model_name == "small" {
            "ggml-base.bin"
        } else {
            "ggml-small.bin"
        };
        let fallback_path = cache_dir.join(fallback_file);
        if fallback_path.exists() {
            eprintln!(
                "Hinweis: Konfiguriertes Modell '{}' nicht gefunden. Verwende lokales Fallback: '{}'.",
                self.model_name, fallback_file
            );
            return fallback_path;
        }

        // 3. Beliebige im Cache auffindbare ggml-*.bin Modelldatei nutzen
        if let Ok(entries) = std::fs::read_dir(&cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension()
                    && ext == "bin"
                    && path.file_name().is_some_and(|n| n.to_string_lossy().starts_with("ggml-"))
                {
                    eprintln!(
                        "Hinweis: Verwende alternatives gefundenes Modell: '{}'.",
                        path.display()
                    );
                    return path;
                }
            }
        }

        // Falls gar kein Modell lokal existiert, den Zielpfad zurückliefern
        raw_path
    }

    /// Speichert die aktuelle Konfiguration im JSON-Format auf die Festplatte.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let s_buffy_json = serde_json::to_string_pretty(self)
            .map_err(std::io::Error::other)?;
        std::fs::write(&path, s_buffy_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.sample_rate, 16_000);
        assert_eq!(config.default_language, Some("de".to_string()));
        assert_eq!(config.model_name, "small");
        assert!(config.beam_search);
        assert!(config.normalize_audio);
        assert!(config.autostart);
        assert_eq!(config.window_x, None);
        assert_eq!(config.window_y, None);
    }

    #[test]
    fn test_raw_model_path() {
        let mut config = AppConfig::default();
        assert!(config.raw_model_path().ends_with("ggml-small.bin"));

        config.model_name = "base".to_string();
        assert!(config.raw_model_path().ends_with("ggml-base.bin"));
    }

    #[test]
    fn test_resolved_model_path_with_fallback() {
        let mut config = AppConfig::default();
        let resolved = config.resolved_model_path();
        // Da ggml-base.bin existiert und ggml-small.bin nicht, muss resolved ggml-base.bin sein oder ggml-small.bin falls vorhanden
        assert!(resolved.ends_with("ggml-base.bin") || resolved.ends_with("ggml-small.bin"));

        config.model_name = "base".to_string();
        let resolved_base = config.resolved_model_path();
        assert!(resolved_base.ends_with("ggml-base.bin") || resolved_base.ends_with("ggml-small.bin"));
    }

    #[test]
    fn test_config_backward_compatibility_missing_fields() {
        // Altes JSON ohne die neuen Einstellungen
        let s_buffy_old_json = r#"{"model_path":"/tmp/model.bin","default_language":"de","silence_threshold_rms":0.012,"silence_timeout_secs":2.5,"sample_rate":16000,"window_x":100,"window_y":200}"#;
        let deserialized: AppConfig = serde_json::from_str(s_buffy_old_json).expect("Deserialization of old json failed");

        assert_eq!(deserialized.model_name, "small");
        assert!(deserialized.beam_search);
        assert!(deserialized.normalize_audio);
        assert!(deserialized.autostart);
        assert_eq!(deserialized.window_x, Some(100));
        assert_eq!(deserialized.window_y, Some(200));
    }

    #[test]
    fn test_config_path_resolution() {
        let path = AppConfig::config_path();
        assert!(path.ends_with("whisper_pill/config.json"));
    }

    #[test]
    fn test_config_serialization_with_window_position() {
        let config = AppConfig {
            window_x: Some(240),
            window_y: Some(80),
            ..Default::default()
        };

        let s_buffy_serialized = serde_json::to_string(&config).expect("Serialization failed");
        assert!(s_buffy_serialized.contains("\"window_x\":240"));
        assert!(s_buffy_serialized.contains("\"window_y\":80"));

        let deserialized: AppConfig =
            serde_json::from_str(&s_buffy_serialized).expect("Deserialization failed");
        assert_eq!(deserialized.window_x, Some(240));
        assert_eq!(deserialized.window_y, Some(80));
        assert_eq!(deserialized, config);
    }
}
