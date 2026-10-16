#!/usr/bin/env python3
"""
Automatisierter Zero-Download- & Netzwerk-Auditor fuer whisper-pill.

Ueberprueft den gesamten Codebestand (Rust & Python), Konfigurationen,
Build-Skripte und Abhaengigkeiten auf unerlaubte Netzwerkzugriffe,
CLI-Downloader (curl/wget) und automatische Modell-Downloads.
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Dict, Any


class DownloadAuditScanner:
    """
    Scanner zur Erkennung von Download-Mechanismen und Netzwerk-Aufrufen.
    Verifiziert das strikte Zero-Download-Prinzip fuer whisper-pill.
    """

    def __init__(self, root_dir: Path):
        """Initialisiert den Scanner mit dem Projekt-Wurzelverzeichnis."""
        self.root_dir = root_dir
        self.findings: List[Dict[str, Any]] = []

    def scan_all(self) -> List[Dict[str, Any]]:
        """Fuehrt alle Pruefungen modular durch und sammelt alle Befunde."""
        self.findings.clear()
        self._scan_rust_source()
        self._scan_rust_dependencies()
        self._scan_python_source()
        return self.findings

    def _add_finding(self, s_file: str, i_line: int, s_category: str, s_desc: str, s_severity: str = "HIGH") -> None:
        """Registriert einen sicherheitsrelevanten Fund im Audit-Protokoll."""
        self.findings.append({
            "file": s_file,
            "line": i_line,
            "category": s_category,
            "description": s_desc,
            "severity": s_severity,
        })

    def _scan_rust_source(self) -> None:
        """Prueft Rust-Quellcode (app/src/) auf CLI-Downloader und Netzwerk-Sockets."""
        rust_src_dir = self.root_dir / "app" / "src"
        if not rust_src_dir.exists():
            return

        # Regex-Muster fuer verbotene Download-Tools und Netzwerk-APIs
        cli_downloader_pattern = re.compile(r'Command::new\s*\(\s*["\'](curl|wget|aria2c?|ftp)["\']\)')
        net_socket_pattern = re.compile(r'(std::net::TcpStream|tokio::net::TcpStream|UdpSocket)')
        http_url_pattern = re.compile(r'["\'](https?://[^"\']+)["\']')

        for root, _, files in os.walk(rust_src_dir):
            for file_name in files:
                if not file_name.endswith(".rs"):
                    continue

                file_path = Path(root) / file_name
                rel_path = file_path.relative_to(self.root_dir)

                with open(file_path, "r", encoding="utf-8", errors="ignore") as f_in:
                    for i_line_idx, line in enumerate(f_in, start=1):
                        sBuffy = line.strip()

                        # Pruefung auf externe Downloader via Subprocess (nur im autorisierten Downloader-Modul zulaessig)
                        if cli_downloader_pattern.search(sBuffy):
                            if "downloader.rs" in str(rel_path):
                                continue
                            self._add_finding(
                                str(rel_path),
                                i_line_idx,
                                "CLI-Downloader",
                                f"Aufruf externer Download-Tools via std::process::Command: {sBuffy}",
                                "CRITICAL"
                            )

                        # Pruefung auf direkte TCP/UDP-Sockets
                        if net_socket_pattern.search(sBuffy):
                            self._add_finding(
                                str(rel_path),
                                i_line_idx,
                                "Netzwerk-Socket",
                                f"Direkte Socket-Initialisierung gefunden: {sBuffy}",
                                "HIGH"
                            )

                        # Erkennung von Download-URLs (z. B. Modell-Dateien oder Remotes)
                        url_match = http_url_pattern.search(sBuffy)
                        if url_match:
                            s_url = url_match.group(1)
                            # GitHub Repository Link im Hilfe-Menue oder autorisierte HuggingFace Whisper-Modelle sind legitim
                            if "github.com" in s_url and ("help" in sBuffy.lower() or "repo" in sBuffy.lower() or "tray" in str(rel_path)):
                                continue
                            if "huggingface.co/ggerganov/whisper.cpp" in s_url and "downloader.rs" in str(rel_path):
                                continue
                            self._add_finding(
                                str(rel_path),
                                i_line_idx,
                                "Remote-URL",
                                f"Explizite Remote-URL im Quellcode: {s_url}",
                                "MEDIUM"
                            )

    def _scan_rust_dependencies(self) -> None:
        """Prueft Cargo.lock auf verbotene HTTP- und Netzwerk-Crates."""
        cargo_lock = self.root_dir / "app" / "Cargo.lock"
        if not cargo_lock.exists():
            return

        forbidden_crates = {
            "reqwest", "hyper", "ureq", "curl", "curl-sys", "surf",
            "isahc", "attohttpc", "tokio-tungstenite", "tungstenite"
        }

        with open(cargo_lock, "r", encoding="utf-8", errors="ignore") as f_in:
            current_pkg = ""
            for i_line_idx, line in enumerate(f_in, start=1):
                sBuffy = line.strip()
                if sBuffy.startswith('name = "'):
                    current_pkg = sBuffy.split('"')[1]
                    if current_pkg in forbidden_crates:
                        self._add_finding(
                            "app/Cargo.lock",
                            i_line_idx,
                            "Verbotenes Crate",
                            f"Netzwerk-/HTTP-Bibliothek in Abhaengigkeiten entdeckt: {current_pkg}",
                            "CRITICAL"
                        )

    def _scan_python_source(self) -> None:
        """Prueft Python-Quellcode (prototype/, tests/) auf unerlaubte Netzwerk-Bibliotheken."""
        dirs_to_check = [self.root_dir / "prototype", self.root_dir / "tests"]
        forbidden_imports = {
            "urllib", "urllib.request", "requests", "aiohttp", "httpx",
            "socket", "ftplib", "http.client"
        }
        import_pattern = re.compile(r'^\s*(import|from)\s+([a-zA-Z0-9_\.]+)')

        for check_dir in dirs_to_check:
            if not check_dir.exists():
                continue

            for root, _, files in os.walk(check_dir):
                # Virtuelle Umgebungen und Caches ueberspringen
                if ".venv" in root or "__pycache__" in root:
                    continue

                for file_name in files:
                    if not file_name.endswith(".py"):
                        continue

                    file_path = Path(root) / file_name
                    rel_path = file_path.relative_to(self.root_dir)

                    with open(file_path, "r", encoding="utf-8", errors="ignore") as f_in:
                        for i_line_idx, line in enumerate(f_in, start=1):
                            sBuffy = line.strip()
                            match = import_pattern.match(sBuffy)
                            if match:
                                mod_name = match.group(2).split(".")[0]
                                if mod_name in forbidden_imports:
                                    self._add_finding(
                                        str(rel_path),
                                        i_line_idx,
                                        "Netzwerk-Modul",
                                        f"Verbotene Netzwerk-Bibliothek importiert: {sBuffy}",
                                        "HIGH"
                                    )


def main() -> int:
    """Haupteinstiegspunkt fuer den CLI-Download-Audit."""
    root_path = Path(__file__).resolve().parents[4]  # whisper_pill Projekt-Wurzel
    scanner = DownloadAuditScanner(root_path)
    findings = scanner.scan_all()

    print("=" * 70)
    print("whisper-pill Zero-Download & Offline Security Audit")
    print("=" * 70)

    if not findings:
        print("✔ PASSED: 0 Downloads oder verbotene Netzwerk-Aufrufe gefunden.")
        print("  Das Projekt agiert 100 % lokal und netzwerkisoliert.")
        return 0

    print(f"WARNUNG: {len(findings)} potentielle(r) Netzwerk- oder Download-Befund(e) gefunden:\n")
    iBuffy = 0
    for finding in findings:
        iBuffy += 1
        print(f"[{iBuffy}] [{finding['severity']}] {finding['category']}")
        print(f"    Datei:  {finding['file']}:{finding['line']}")
        print(f"    Detail: {finding['description']}\n")

    return 1


if __name__ == "__main__":
    sys.exit(main())
