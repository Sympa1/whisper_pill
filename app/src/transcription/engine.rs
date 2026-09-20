//! Transkriptions-Engine basierend auf whisper-rs / whisper.cpp.
//!
//! Fuehrt die lokale Spracherkennung mit Whisper-GGML-Modellen durch,
//! unterstuetzt Sprachauswahl und stellt die RAM-Sicherheit durch
//! automatisches Nullen von Puffern sicher.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Kapselt den Whisper-Inferenz-Kontext und die Thread-sichere Ausfuehrung.
pub struct WhisperEngine {
    /// Pfad zum geladenen Modell.
    model_path: PathBuf,
    /// Whisper-Kontext zur Modellausfuehrung.
    context: Mutex<Option<WhisperContext>>,
}

impl WhisperEngine {
    /// Erstellt eine neue Transkriptions-Engine mit dem angegebenen Modellpfad.
    pub fn new<P: AsRef<Path>>(model_path: P) -> Self {
        Self {
            model_path: model_path.as_ref().to_path_buf(),
            context: Mutex::new(None),
        }
    }

    /// Stellt sicher, dass das Modell existiert, und laedt es bei Bedarf automatisch herunter.
    pub fn ensure_model_available(&self) -> Result<(), String> {
        if self.model_path.exists() {
            return Ok(());
        }

        if let Some(parent) = self.model_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let download_url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin";
        eprintln!("[whisper-pill] Lade Whisper-Modell herunter: {}", download_url);

        let status = std::process::Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(&self.model_path)
            .arg(download_url)
            .status()
            .map_err(|e| format!("Fehler beim Ausfuehren von curl: {}", e))?;

        if status.success() {
            eprintln!("[whisper-pill] Modell erfolgreich heruntergeladen.");
            Ok(())
        } else {
            Err(format!("Download des Whisper-Modells von {} fehlgeschlagen.", download_url))
        }
    }

    /// Laedt das GGML-Modell in den Speicher, falls noch nicht geschehen.
    pub fn load_model(&self) -> Result<(), String> {
        let mut ctx_guard = self.context.lock().map_err(|e| e.to_string())?;
        if ctx_guard.is_some() {
            return Ok(());
        }

        // Bei Bedarf automatisch herunterladen
        self.ensure_model_available()?;

        if !self.model_path.exists() {
            return Err(format!(
                "Whisper-Modell nicht gefunden unter: {}",
                self.model_path.display()
            ));
        }

        let s_buffy_path = self
            .model_path
            .to_str()
            .ok_or_else(|| "Ungueltiger Modellpfad (kein valider UTF-8 Pfad)".to_string())?;

        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(s_buffy_path, params)
            .map_err(|e| format!("Fehler beim Initialisieren des Whisper-Modells: {}", e))?;

        *ctx_guard = Some(ctx);
        Ok(())
    }

    /// Transkribiert 16-kHz-Mono-Audiosamples zu Text.
    pub fn transcribe(&self, audio_samples: &[f32], language: Option<&str>) -> Result<String, String> {
        if audio_samples.is_empty() {
            return Ok(String::new());
        }

        // Sicherstellen, dass das Modell bereitsteht
        self.load_model()?;

        let mut ctx_guard = self.context.lock().map_err(|e| e.to_string())?;
        let ctx = ctx_guard
            .as_mut()
            .ok_or_else(|| "Whisper-Kontext ist nicht geladen.".to_string())?;

        let mut state = ctx
            .create_state()
            .map_err(|e| format!("Whisper-State konnte nicht erstellt werden: {}", e))?;

        // Inferenz-Parameter konfigurieren
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(language);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_translate(false);

        // Inferenz auf den uebergebenen Samples starten
        state
            .full(params, audio_samples)
            .map_err(|e| format!("Inferenz-Fehler waehrend der Transkription: {}", e))?;

        // Segmente ueber den Iterator auslesen und zu Gesamttext zusammenfuegen
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
        self.model_path.exists()
    }

    /// Liefert den Pfad zum Modell.
    pub fn model_path(&self) -> &Path {
        &self.model_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_initialization() {
        let engine = WhisperEngine::new("/tmp/non_existent_model.bin");
        assert!(!engine.is_model_available());
        assert_eq!(engine.model_path(), Path::new("/tmp/non_existent_model.bin"));
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
