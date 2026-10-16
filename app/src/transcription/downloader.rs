//! Modell-Downloader fuer whisper-pill.
//!
//! Ermoeglicht den automatischen Bezug benoetigter GGML-Sprachmodelle
//! (z. B. ggml-base.bin oder ggml-small.bin) von der offiziellen
//! whisper.cpp HuggingFace-Quelle, falls diese lokal noch nicht existieren.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Ermittelt das Standard-Cache-Verzeichnis fuer GGML-Modelle.
pub fn model_cache_dir() -> PathBuf {
    let s_buffy_home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(s_buffy_home).join(".cache").join("whisper_pill")
}

/// Liefert die offizielle Download-URL fuer ein bestimmtes Modell ("base", "small" etc.).
pub fn model_download_url(model_name: &str) -> String {
    format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin",
        model_name
    )
}

/// Prueft, ob das angegebene Modell existiert, und laedt es synchron herunter, falls nicht vorhanden.
/// Verwendet curl oder wget mit atomarer .tmp-Datei, um korrumpierte Downloads zu vermeiden.
pub fn download_model_if_missing(model_name: &str) -> Result<PathBuf, String> {
    let cache_dir = model_cache_dir();
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Cache-Verzeichnis konnte nicht erstellt werden: {}", e))?;
    }

    let target_file = cache_dir.join(format!("ggml-{}.bin", model_name));
    if target_file.exists() {
        return Ok(target_file);
    }

    let tmp_file = cache_dir.join(format!("ggml-{}.bin.tmp", model_name));
    let s_buffy_url = model_download_url(model_name);

    eprintln!(
        "Whisper Pill: Lade Modell '{}' herunter von {} ...",
        model_name, s_buffy_url
    );

    // 1. Download via curl versuchen
    let s_buffy_target_tmp = tmp_file.to_string_lossy().to_string();
    let curl_status = Command::new("curl")
        .args([
            "-L",
            "--fail",
            "--silent",
            "--show-error",
            "-o",
            &s_buffy_target_tmp,
            &s_buffy_url,
        ])
        .status();

    let download_success = match curl_status {
        Ok(status) => status.success(),
        Err(_) => {
            // 2. Fallback auf wget, falls curl nicht verfuegbar ist
            let wget_status = Command::new("wget")
                .args(["-q", "-O", &s_buffy_target_tmp, &s_buffy_url])
                .status();
            match wget_status {
                Ok(status) => status.success(),
                Err(e) => {
                    return Err(format!(
                        "Weder curl noch wget konnten ausgefuehrt werden: {}",
                        e
                    ));
                }
            }
        }
    };

    if !download_success {
        let _ = fs::remove_file(&tmp_file);
        return Err(format!(
            "Download fuer Modell '{}' fehlgeschlagen.",
            model_name
        ));
    }

    // 3. Atomares Verschieben von .tmp auf die Zieldatei
    fs::rename(&tmp_file, &target_file)
        .map_err(|e| format!("Konnte temporaere Modelldatei nicht umbenennen: {}", e))?;

    eprintln!(
        "Whisper Pill: Modell '{}' erfolgreich heruntergeladen: {}",
        model_name,
        target_file.display()
    );

    Ok(target_file)
}

/// Startet den Modell-Download asynchron in einem separaten Hintergrund-Thread,
/// damit die GUI und Audio-Pipeline niemals blockiert werden.
pub fn spawn_background_download<F>(model_name: String, on_complete: F)
where
    F: FnOnce(Result<PathBuf, String>) + Send + 'static,
{
    std::thread::Builder::new()
        .name(format!("whisper-model-downloader-{}", model_name))
        .spawn(move || {
            let res = download_model_if_missing(&model_name);
            on_complete(res);
        })
        .expect("Downloader-Thread konnte nicht gestartet werden");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_cache_dir_ends_with_whisper_pill() {
        let dir = model_cache_dir();
        assert!(dir.ends_with(".cache/whisper_pill") || dir.ends_with("whisper_pill"));
    }

    #[test]
    fn test_model_download_url_format() {
        let url_base = model_download_url("base");
        assert_eq!(
            url_base,
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin"
        );
        let url_small = model_download_url("small");
        assert_eq!(
            url_small,
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin"
        );
    }
}
