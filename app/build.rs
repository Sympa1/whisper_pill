//! Build-Skript fuer Slint: Kompiliert ui/appwindow.slint in Rust-Code.

fn main() {
    slint_build::compile("ui/appwindow.slint").expect("Fehler beim Kompilieren der Slint-Datei");
}
