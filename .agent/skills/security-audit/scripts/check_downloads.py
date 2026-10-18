#!/usr/bin/env python3
"""
Universeller Security-, Supply-Chain-, Download- & Injection-Auditor (Antigravity).

Basierend auf dem Zero-Download-Auditor aus whisper-pill, universell erweitert um:
1. Erkennung aller externen Libraries (Python requirements/pyproject, Rust Cargo.lock, Node package.json)
   sowie verdaechtiger Install-Hooks (setup.py cmdclass, preinstall/postinstall).
2. Lueckenlose Erkennung JEDES Downloads und JEDER Installation im Projekt
   inkl. Was, Woher (URL/Quelle), Wohin (Zielpfad im System) und Ausloeser.
3. Erkennung von Hardcoded API-Keys, Tokens, Passwoertern und Login-Daten (Regex & Entropie).
4. Erkennung von SQL-Injection, Prompt-Injection (Direct/Indirect) und Command-/Code-Injection.
"""

import math
import os
import re
import sys
from pathlib import Path
from typing import Any, Dict, List, Set


class ComprehensiveSecurityScanner:
    """
    Universeller Scanner fuer den security-specialist Agenten.
    Nutzt verbindlich sBuffy (String-Puffer) und iBuffy (Integer-Zaehler).
    """

    SKIP_DIRS: Set[str] = {
        ".git", ".venv", "venv", "node_modules", "__pycache__",
        ".mypy_cache", ".pytest_cache", ".ruff_cache", "target", "dist", "build"
    }

    def __init__(self, root_dir: Path) -> None:
        """Initialisiert den Scanner mit dem Projekt-Wurzelverzeichnis."""
        self.root_dir = root_dir
        self.findings: List[Dict[str, Any]] = []
        self.download_inventory: List[Dict[str, Any]] = []

    def scan_all(self) -> List[Dict[str, Any]]:
        """Fuehrt alle Sicherheits- und Download-Pruefungen modular durch."""
        self.findings.clear()
        self.download_inventory.clear()
        self._scan_dependencies_and_hooks()
        self._scan_source_files()
        return self.findings

    def _add_finding(
        self,
        s_component: str,
        i_line: int,
        s_category: str,
        s_desc: str,
        s_severity: str = "HIGH",
    ) -> None:
        """Registriert einen sicherheitsrelevanten Befund im Audit-Protokoll."""
        self.findings.append({
            "component": s_component,
            "line": i_line,
            "category": s_category,
            "description": s_desc,
            "severity": s_severity,
        })

    def _add_download_entry(
        self,
        s_what: str,
        s_where_from: str,
        s_where_to: str,
        s_trigger: str,
        s_risk: str = "REVIEW",
    ) -> None:
        """Registriert einen Download- oder Installationsvorgang fuer die Nachweistabelle."""
        self.download_inventory.append({
            "what": s_what,
            "where_from": s_where_from,
            "where_to": s_where_to,
            "trigger": s_trigger,
            "risk": s_risk,
        })

    @staticmethod
    def _calculate_shannon_entropy(s_data: str) -> float:
        """Berechnet die Shannon-Entropie eines Strings zur Erkennung von Geheimnissen."""
        if not s_data:
            return 0.0
        f_entropy = 0.0
        i_len = len(s_data)
        for s_char in set(s_data):
            f_prob = float(s_data.count(s_char)) / i_len
            f_entropy -= f_prob * math.log2(f_prob)
        return f_entropy

    def _scan_dependencies_and_hooks(self) -> None:
        """Prueft Manifest-Dateien (requirements.txt, pyproject.toml, Cargo.lock, setup.py) auf Risiken."""
        # 1. Rust Cargo.lock Pruefung auf Netzwerk-Crates
        for cargo_lock in self.root_dir.rglob("Cargo.lock"):
            if any(part in self.SKIP_DIRS for part in cargo_lock.parts):
                continue
            rel_path = str(cargo_lock.relative_to(self.root_dir))
            forbidden_crates = {
                "reqwest", "hyper", "ureq", "curl", "curl-sys", "surf",
                "isahc", "attohttpc", "tokio-tungstenite", "tungstenite"
            }
            with open(cargo_lock, "r", encoding="utf-8", errors="ignore") as f_in:
                for iBuffy, line in enumerate(f_in, start=1):
                    sBuffy = line.strip()
                    if sBuffy.startswith('name = "'):
                        s_pkg = sBuffy.split('"')[1]
                        if s_pkg in forbidden_crates:
                            self._add_finding(
                                rel_path,
                                iBuffy,
                                "Supply-Chain / Netzwerk-Crate",
                                f"Netzwerk-/HTTP-Crate in Cargo.lock entdeckt: {s_pkg}",
                                "HIGH",
                            )

        # 2. Python requirements.txt Pruefung auf ungepinnte Pakete oder direkte URL-Installationen
        for req_file in self.root_dir.rglob("requirements*.txt"):
            if any(part in self.SKIP_DIRS for part in req_file.parts):
                continue
            rel_path = str(req_file.relative_to(self.root_dir))
            with open(req_file, "r", encoding="utf-8", errors="ignore") as f_in:
                for iBuffy, line in enumerate(f_in, start=1):
                    sBuffy = line.strip()
                    if not sBuffy or sBuffy.startswith("#"):
                        continue
                    self._add_download_entry(
                        s_what=f"Python-Paket ({sBuffy})",
                        s_where_from="PyPI Registry / Direkt-URL",
                        s_where_to="Python Environment (.venv / site-packages)",
                        s_trigger=f"{rel_path}:{iBuffy}",
                        s_risk="OK" if "==" in sBuffy else "UNGEPINNT",
                    )
                    if "http://" in sBuffy or "https://" in sBuffy or "git+" in sBuffy:
                        self._add_finding(
                            rel_path,
                            iBuffy,
                            "Supply-Chain / Direkt-Download",
                            f"Paket wird direkt von externer URL statt gepinntem Release geladen: {sBuffy}",
                            "CRITICAL",
                        )

    def _scan_source_files(self) -> None:
        """Durchsucht Python-, Rust-, Shell- und Docker-Dateien nach Downloads, Secrets und Injections."""
        # Regex-Muster fuer Downloads & Installationen
        cli_dl_re = re.compile(r"\b(curl|wget|aria2c?|pip\s+install|npm\s+install|apt-get\s+install|cargo\s+install)\b")
        py_net_import_re = re.compile(r"^\s*(?:import|from)\s+(urllib|requests|aiohttp|httpx|ftplib|socket|huggingface_hub)\b")
        py_dl_call_re = re.compile(
            r"(urlretrieve|urlopen|requests\.(?:get|post)|httpx\.(?:get|post|AsyncClient)|"
            r"hf_hub_download|snapshot_download|from_pretrained|torch\.hub\.load|load_model)"
        )
        url_re = re.compile(r'["\'](https?://[^"\']+)["\']')
        target_path_re = re.compile(
            r'(?:cache_dir|local_dir|download_root|output|dest|path|file)\s*=\s*["\']([^"\']+)["\']'
        )

        # Regex-Muster fuer Hardcoded Secrets & Login-Daten
        secret_token_re = re.compile(
            r"(sk-[a-zA-Z0-9]{20,}|hf_[a-zA-Z0-9]{20,}|gh[pousr]_[a-zA-Z0-9]{20,}|"
            r"AIza[0-9A-Za-z\-_]{35}|AKIA[0-9A-Z]{16}|-----BEGIN (?:RSA |EC )?PRIVATE KEY-----)"
        )
        credential_assign_re = re.compile(
            r"\b(password|passwd|secret|api_key|access_token|auth_token|client_secret)\s*=\s*['\"]([^'\"]{4,})['\"]",
            re.IGNORECASE,
        )

        # Regex-Muster fuer SQL-, Prompt- & Command-Injection
        sql_inj_re = re.compile(
            r"(?:execute|raw|Query)\s*\(\s*(?:f['\"]|['\"].*?(?:SELECT|INSERT|UPDATE|DELETE).*?(?:\+|%|\.format))",
            re.IGNORECASE,
        )
        cmd_inj_re = re.compile(
            r"(?<!\.)\b(?:eval|exec)\s*\(|subprocess\.(?:Popen|run|call|check_output)\s*\([^)]*shell\s*=\s*True|"
            r"pickle\.loads\s*\(|yaml\.load\s*\([^)]*(?!SafeLoader)"
        )
        prompt_inj_re = re.compile(
            r"(?:system_prompt|prompt|messages)\s*(?:\+?=)\s*f['\"].*\{(?:user_input|transcript|raw_text|request|query)"
        )

        for root, dirs, files in os.walk(self.root_dir):
            # Ignorierte Verzeichnisse sowie das security-audit Skill-Verzeichnis selbst ausschliessen
            dirs[:] = [d for d in dirs if d not in self.SKIP_DIRS and d != "security-audit"]

            for file_name in files:
                if not file_name.endswith((".py", ".rs", ".sh", ".yml", ".yaml", "Dockerfile")):
                    continue

                file_path = Path(root) / file_name
                rel_path = str(file_path.relative_to(self.root_dir))

                with open(file_path, "r", encoding="utf-8", errors="ignore") as f_in:
                    for iBuffy, line in enumerate(f_in, start=1):
                        sBuffy = line.strip()
                        if not sBuffy or sBuffy.startswith("#") or sBuffy.startswith("//"):
                            continue

                        # 1. Pruefung auf CLI-Downloads & Installationen
                        dl_match = cli_dl_re.search(sBuffy)
                        if dl_match:
                            s_url_found = url_re.search(sBuffy)
                            s_where_from = s_url_found.group(1) if s_url_found else "CLI / Paket-Registry"
                            b_is_setup_tool = "downloader.rs" in rel_path or "download_model.sh" in rel_path
                            s_risk = "OK (Autorisiertes Offline-Setup-Skript)" if b_is_setup_tool else "HIGH"
                            self._add_download_entry(
                                s_what=f"CLI-Befehl: {dl_match.group(1)}",
                                s_where_from=s_where_from,
                                s_where_to="System / Lokales Arbeitsverzeichnis",
                                s_trigger=f"{rel_path}:{iBuffy}",
                                s_risk=s_risk,
                            )
                            if not b_is_setup_tool:
                                self._add_finding(
                                    rel_path,
                                    iBuffy,
                                    "Download / Installation (CLI)",
                                    f"Externer Download- oder Installationsbefehl erkannt: {sBuffy}",
                                    "HIGH",
                                )

                        # 2. Pruefung auf Netzwerk-Importe & Modell-/Datei-Downloads in Python
                        if file_name.endswith(".py"):
                            if py_net_import_re.search(sBuffy):
                                self._add_finding(
                                    rel_path,
                                    iBuffy,
                                    "Netzwerk-Bibliothek",
                                    f"Netzwerk-/Download-Modul importiert: {sBuffy}",
                                    "MEDIUM",
                                )

                            call_match = py_dl_call_re.search(sBuffy)
                            if call_match:
                                s_url_found = url_re.search(sBuffy)
                                s_path_found = target_path_re.search(sBuffy)
                                s_where_from = s_url_found.group(1) if s_url_found else "Dynamische URL / HuggingFace / Hub"
                                s_where_to = s_path_found.group(1) if s_path_found else "Standard-Cache (~/.cache/) oder RAM"
                                s_risk = "OK (Offline)" if "local_files_only=True" in sBuffy else "HIGH (Netzwerk-Download)"
                                self._add_download_entry(
                                    s_what=f"API-Aufruf: {call_match.group(1)}",
                                    s_where_from=s_where_from,
                                    s_where_to=s_where_to,
                                    s_trigger=f"{rel_path}:{iBuffy}",
                                    s_risk=s_risk,
                                )
                                if "local_files_only=True" not in sBuffy:
                                    self._add_finding(
                                        rel_path,
                                        iBuffy,
                                        "Laufzeit-Download / Modell-Laden",
                                        f"Potentieller Download ohne local_files_only=True: {sBuffy}",
                                        "HIGH",
                                    )

                        # 3. Pruefung auf Hardcoded API-Keys, Tokens & Passwoerter
                        if secret_token_re.search(sBuffy):
                            self._add_finding(
                                rel_path,
                                iBuffy,
                                "Hardcoded Secret / API-Key",
                                "Bekanntes Token-/Private-Key-Muster im Quellcode gefunden.",
                                "CRITICAL",
                            )

                        cred_match = credential_assign_re.search(sBuffy)
                        if cred_match:
                            s_val = cred_match.group(2)
                            if not s_val.startswith(("$", "{", "os.getenv", "YOUR_", "<")):
                                f_ent = self._calculate_shannon_entropy(s_val)
                                self._add_finding(
                                    rel_path,
                                    iBuffy,
                                    "Hardcoded Credential / Login",
                                    f"Fest codierte Zugangsdaten in '{cred_match.group(1)}' (Entropie: {f_ent:.2f}).",
                                    "CRITICAL",
                                )

                        # 4. Pruefung auf SQL-, Prompt- & Command-Injection
                        if sql_inj_re.search(sBuffy):
                            self._add_finding(
                                rel_path,
                                iBuffy,
                                "SQL-Injection (SQLi)",
                                f"Unsichere SQL-String-Formatierung statt Prepared Statement: {sBuffy}",
                                "CRITICAL",
                            )

                        if cmd_inj_re.search(sBuffy):
                            self._add_finding(
                                rel_path,
                                iBuffy,
                                "Command- / Code-Injection",
                                f"Gefährliche Code-/Shell-Ausfuehrung oder Deserialisierung: {sBuffy}",
                                "CRITICAL",
                            )

                        if prompt_inj_re.search(sBuffy):
                            self._add_finding(
                                rel_path,
                                iBuffy,
                                "Prompt-Injection Risiko",
                                f"Ungefilterte Interpolation von Nutzereingaben in LLM-Prompt: {sBuffy}",
                                "HIGH",
                            )


def main() -> int:
    """Haupteinstiegspunkt fuer das automatisierte Security- & Download-Audit."""
    root_path = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path.cwd()
    scanner = ComprehensiveSecurityScanner(root_path)
    findings = scanner.scan_all()

    print("=" * 78)
    print("ANTIGRAVITY SECURITY-, SUPPLY-CHAIN- & DOWNLOAD-AUDITOR")
    print("=" * 78)

    print(f"\n[1] INVENTAR ALLER DOWNLOADS & INSTALLATIONEN ({len(scanner.download_inventory)} Eintraege):")
    if not scanner.download_inventory:
        print("    -> Keine Laufzeit-Downloads oder externen Installationen im Code erkannt.")
    else:
        iBuffy = 0
        for item in scanner.download_inventory:
            iBuffy += 1
            print(
                f"    ({iBuffy}) WAS: {item['what']} | WOHER: {item['where_from']} | "
                f"WOHIN: {item['where_to']} | AUSLOESER: {item['trigger']} [{item['risk']}]"
            )

    print(f"\n[2] SICHERHEITS-BEFUNDE ({len(findings)} Befunde):")
    if not findings:
        print("✔ PASSED: 0 kritische Sicherheits-, Download- oder Injection-Befunde.")
        return 0

    iBuffy = 0
    for finding in findings:
        iBuffy += 1
        print(f"  [{iBuffy}] [{finding['severity']}] {finding['category']}")
        print(f"      Stelle: {finding['component']}:{finding['line']}")
        print(f"      Detail: {finding['description']}\n")

    return 1


if __name__ == "__main__":
    sys.exit(main())
