#!/usr/bin/env python3
"""
Icon-Generator fuer Whisper Pill: Rendert das SVG-Vektor-Icon in hochaufloesende PNG-Dateien.
Nutzt PySide6 zur verlustfreien Rasterung fuer Linux AppImage und Desktop-Standards.
"""

import sys
from pathlib import Path
from PySide6.QtCore import QSize, Qt
from PySide6.QtGui import QImage, QPainter, QColor
from PySide6.QtSvg import QSvgRenderer


def render_svg_to_png(svg_path: Path, output_png_path: Path, i_size: int = 256) -> None:
    """
    Rendert eine SVG-Vektordatei in eine quadratische PNG-Rastergrafik der gewuenschten Groesse.
    """
    renderer = QSvgRenderer(str(svg_path))
    if not renderer.isValid():
        raise RuntimeError(f"Ungueltige SVG-Datei: {svg_path}")

    # Erzeuge ein 32-Bit RGBA-Bild mit transparentem Hintergrund
    image = QImage(i_size, i_size, QImage.Format.Format_ARGB32_Premultiplied)
    image.fill(QColor(0, 0, 0, 0))

    painter = QPainter(image)
    painter.setRenderHint(QPainter.RenderHint.Antialiasing, True)
    painter.setRenderHint(QPainter.RenderHint.SmoothPixmapTransform, True)
    renderer.render(painter)
    painter.end()

    output_png_path.parent.mkdir(parents=True, exist_ok=True)
    if not image.save(str(output_png_path), "PNG"):
        raise RuntimeError(f"Konnte PNG-Datei nicht speichern: {output_png_path}")
    print(f"Icon erfolgreich generiert: {output_png_path} ({i_size}x{i_size})")


def main() -> None:
    """Hauptfunktion zum Erzeugen der Icon-Formate fuer das AppImage."""
    base_dir = Path(__file__).resolve().parent
    svg_icon_path = base_dir / "whisper-pill.svg"

    # Zielgroessen fuer Freedesktop / AppImage Icon-Themen
    target_sizes = [64, 128, 256, 512]

    for iBuffy in target_sizes:
        target_png = base_dir / f"whisper-pill_{iBuffy}x{iBuffy}.png"
        render_svg_to_png(svg_icon_path, target_png, iBuffy)

    # Standard 256x256 Icon als Referenz
    render_svg_to_png(svg_icon_path, base_dir / "whisper-pill.png", 256)


if __name__ == "__main__":
    main()
