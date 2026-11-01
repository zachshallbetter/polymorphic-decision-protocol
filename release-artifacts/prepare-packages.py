#!/usr/bin/env python3
"""Build exact, manifest-controlled PDP software and public preprint packages."""
from __future__ import annotations

import csv
import hashlib
import shutil
import subprocess
import sys
from pathlib import Path

from package_archive import build_zip as build_deterministic_zip

ROOT = Path(__file__).resolve().parents[1]
ARTIFACTS = Path(__file__).resolve().parent
PAPER_SOURCE = ROOT / "publication-package"
PAPER_STAGE = ARTIFACTS / "paper-v1.0.0"


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def write_checksums(package: Path) -> None:
    checksum_path = package / "archive" / "checksums.sha256"
    checksum_path.parent.mkdir(parents=True, exist_ok=True)
    files = sorted(p for p in package.rglob("*") if p.is_file() and p != checksum_path)
    checksum_path.write_text(
        "".join(f"{digest(path)}  {path.relative_to(package).as_posix()}\n" for path in files),
        encoding="utf-8",
    )


def write_paper_manifest(package: Path) -> None:
    manifest = package / "metadata" / "manifest.csv"
    manifest.parent.mkdir(parents=True, exist_ok=True)
    rows = []
    for path in sorted(
        p for p in package.rglob("*")
        if p.is_file() and p not in {manifest, package / "archive" / "checksums.sha256"}
    ):
        rel = path.relative_to(package).as_posix()
        if rel.startswith("manuscript/") or rel.startswith("research/paper/"):
            description, kind = "Scholarly manuscript source or compiled rendition", "manuscript"
        elif rel.startswith("research/"):
            description, kind = "RDP Project research record", "research-record"
        elif rel.startswith("docs/"):
            description, kind = "Research documentation", "documentation"
        elif rel.startswith("sources/"):
            description, kind = "Bibliographic source record", "source-register"
        elif rel.startswith("metadata/"):
            description, kind = "Machine-readable package metadata", "metadata"
        elif rel.startswith("archive/"):
            description, kind = "Release archive record", "archive"
        elif rel.startswith("publication/"):
            description, kind = "Publication readiness record", "publication"
        elif rel == "LICENSE":
            description, kind = "CC BY 4.0 paper-package license notice", "legal"
        else:
            description, kind = "Package orientation or citation metadata", "documentation"
        rows.append((rel, description, kind))
    rows.extend([
        ("metadata/manifest.csv", "Curated package file manifest", "metadata"),
        ("archive/checksums.sha256", "SHA-256 integrity list", "archive"),
    ])
    with manifest.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.writer(stream, lineterminator="\n")
        writer.writerow(("path", "description", "type"))
        writer.writerows(sorted(rows, key=lambda row: row[0]))


def verify_paper_source() -> None:
    source_md = ROOT / "research" / "paper" / "pdp-normative-procedure-v1.0.0.md"
    rpp_md = PAPER_SOURCE / "manuscript" / "paper.md"
    if source_md.read_bytes() != rpp_md.read_bytes():
        raise ValueError("RDP source and RPP compatibility manuscript differ")
    source_bib = ROOT / "research" / "paper" / "latex" / "pdp-normative-procedure-v1.0.0-references.bib"
    rpp_bib = PAPER_SOURCE / "manuscript" / "references.bib"
    if source_bib.read_bytes() != rpp_bib.read_bytes():
        raise ValueError("RDP bibliography and RPP compatibility bibliography differ")


def sync_paper_source() -> None:
    source = ROOT / "research" / "paper"
    package_paper = PAPER_SOURCE / "research" / "paper"
    if package_paper.exists():
        shutil.rmtree(package_paper)
    shutil.copytree(source, package_paper)
    for filename in (
        "pdp-normative-procedure-v1.0.0.md",
        "pdp-normative-procedure-v1.0.0.pdf",
        "pdp-normative-procedure-v1.0.0.tex",
    ):
        shutil.copyfile(source / filename, PAPER_SOURCE / "manuscript" / filename)
    shutil.copyfile(source / "pdp-normative-procedure-v1.0.0.md", PAPER_SOURCE / "manuscript" / "paper.md")
    shutil.copyfile(source / "latex" / "pdp-normative-procedure-v1.0.0-references.bib", PAPER_SOURCE / "manuscript" / "references.bib")
    shutil.copyfile(source / "manuscript-front-matter.yml", PAPER_SOURCE / "manuscript" / "front-matter.yml")


def build_paper() -> None:
    sync_paper_source()
    verify_paper_source()
    write_paper_manifest(PAPER_SOURCE)
    write_checksums(PAPER_SOURCE)
    if PAPER_STAGE.exists():
        shutil.rmtree(PAPER_STAGE)
    shutil.copytree(PAPER_SOURCE, PAPER_STAGE)


def build_zip(package: Path, name: str) -> Path:
    expected = {path.relative_to(package).as_posix() for path in package.rglob("*") if path.is_file()}
    output, _ = build_deterministic_zip(package, name, ARTIFACTS, expected)
    return output


def main() -> None:
    subprocess.run([
        sys.executable,
        str(ARTIFACTS / "build-software-release.py"),
        "v1.4.1",
        "--source-dir", str(ROOT),
        "--output-dir", str(ARTIFACTS),
    ], check=True)
    build_paper()
    paper_zip = build_zip(PAPER_STAGE, "PDP-paper-v1.0.0")
    print(f"paper package:    {paper_zip} ({paper_zip.stat().st_size} bytes)")
    print("No publication or deposit action was performed.")


if __name__ == "__main__":
    main()
