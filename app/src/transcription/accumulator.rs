//! Text-Akkumulator fuer schrittweise Transkriptionen und Fortsetzungsdiktate.
//!
//! Stellt Hilfsfunktionen bereit, um bestehende Transkriptionen sauber mit neu
//! eingesprochenen Abschnitten zu verketten.

/// Verbindet einen bestehenden Transkriptionstext mit einem neu transkribierten Textsegment.
///
/// Beruecksichtigt fuehrende und nachfolgende Leerzeichen und fuegt genau ein Trenn-Leerzeichen
/// ein, wenn beide Texte nicht leer sind.
pub fn append_transcript(s_buffy_existing: &str, s_buffy_new: &str) -> String {
    let s_buffy_trimmed_existing = s_buffy_existing.trim();
    let s_buffy_trimmed_new = s_buffy_new.trim();

    if s_buffy_trimmed_existing.is_empty() {
        return s_buffy_trimmed_new.to_string();
    }
    if s_buffy_trimmed_new.is_empty() {
        return s_buffy_trimmed_existing.to_string();
    }

    format!("{} {}", s_buffy_trimmed_existing, s_buffy_trimmed_new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_to_empty_existing() {
        // Neuer Text bei leerem Vorbestand
        let s_buffy_result = append_transcript("", "Hallo Welt");
        assert_eq!(s_buffy_result, "Hallo Welt");

        let s_buffy_result_ws = append_transcript("   ", "Hallo Welt");
        assert_eq!(s_buffy_result_ws, "Hallo Welt");
    }

    #[test]
    fn test_append_empty_new_to_existing() {
        // Leerer neuer Text veraendert bestehenden Text nicht
        let s_buffy_result = append_transcript("Bestehender Text", "");
        assert_eq!(s_buffy_result, "Bestehender Text");

        let s_buffy_result_ws = append_transcript("Bestehender Text", "   \n");
        assert_eq!(s_buffy_result_ws, "Bestehender Text");
    }

    #[test]
    fn test_append_both_non_empty() {
        // Beide Texte vorhanden: Trennung durch einfaches Leerzeichen
        let s_buffy_result = append_transcript("Hallo Welt", "wie geht es dir?");
        assert_eq!(s_buffy_result, "Hallo Welt wie geht es dir?");

        // Mit ueberfluessigen Leerzeichen an den Raendern
        let s_buffy_result_trim = append_transcript("  Hallo Welt  ", "  wie geht es dir?  ");
        assert_eq!(s_buffy_result_trim, "Hallo Welt wie geht es dir?");
    }

    #[test]
    fn test_append_multiple_sequential() {
        // Mehrmaliges schrittweises Anhaengen
        let mut s_buffy_acc = String::new();
        s_buffy_acc = append_transcript(&s_buffy_acc, "Erster Satz.");
        s_buffy_acc = append_transcript(&s_buffy_acc, "Zweiter Satz.");
        s_buffy_acc = append_transcript(&s_buffy_acc, "Dritter Satz.");

        assert_eq!(s_buffy_acc, "Erster Satz. Zweiter Satz. Dritter Satz.");
    }
}
