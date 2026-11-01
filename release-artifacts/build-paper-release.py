#!/usr/bin/env python3
"""Validate the curated paper package and build its deterministic release assets."""
from __future__ import annotations

import argparse
import csv
import hashlib
import json
import re
import shutil
import sys
from pathlib import Path, PurePosixPath

from package_archive import build_zip, sha256_file

PACKAGE_PATH = Path("release-artifacts/paper-v1.0.0")
PDF_PATH = "manuscript/pdp-normative-procedure-v1.0.0.pdf"
FIELDS = ("path", "description", "type")


def safe_path(value: str) -> PurePosixPath:
    path = PurePosixPath(value)
    if (
        not value or "\\" in value or ":" in value or "\x00" in value
        or path.is_absolute() or path.as_posix() != value
        or any(part in ("", ".", "..") for part in path.parts)
    ):
        raise ValueError(f"unsafe or non-canonical package path: {value!r}")
    return path


def verify_package(package: Path) -> set[str]:
    manifest_path = package / "metadata/manifest.csv"
    with manifest_path.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        if tuple(reader.fieldnames or ()) != FIELDS:
            raise ValueError(f"paper manifest header must be {','.join(FIELDS)}")
        rows = list(reader)

    listed: set[str] = set()
    folded: set[str] = set()
    for row in rows:
        if None in row or any(not (row.get(field) or "").strip() for field in FIELDS):
            raise ValueError("paper manifest has an unexpected column or blank field")
        relative = safe_path(row["path"].strip()).as_posix()
        if relative in listed or relative.casefold() in folded:
            raise ValueError(f"duplicate or case-colliding paper package path: {relative}")
        listed.add(relative)
        folded.add(relative.casefold())

    actual = {path.relative_to(package).as_posix() for path in package.rglob("*") if path.is_file()}
    if listed != actual:
        raise ValueError(f"paper manifest mismatch; unlisted={sorted(actual-listed)}, missing={sorted(listed-actual)}")
    if "archive/checksums.sha256" not in listed or "metadata/manifest.csv" not in listed:
        raise ValueError("paper package must include its manifest and checksum inventory")
    if any(PurePosixPath(path).name.startswith("llms") or ".env" in PurePosixPath(path).name for path in listed):
        raise ValueError("paper package contains generated LLM context or environment material")
    if any(re.search(r"private|transcript|trade-record", path, re.I) for path in listed):
        raise ValueError("paper package path signals private case records")

    checksum_path = package / "archive/checksums.sha256"
    checked: set[str] = set()
    for line in checksum_path.read_text(encoding="utf-8").splitlines():
        expected, relative = line.split("  ", maxsplit=1)
        path = safe_path(relative).as_posix()
        candidate = package.joinpath(*PurePosixPath(path).parts)
        if path in checked or not candidate.is_file() or sha256_file(candidate) != expected:
            raise ValueError(f"paper package checksum mismatch: {path}")
        checked.add(path)
    if checked != actual - {"archive/checksums.sha256"}:
        raise ValueError("paper checksum inventory does not exactly match package contents")

    pdf = package / PDF_PATH
    if not pdf.is_file() or pdf.stat().st_size >= 5_000_000 or pdf.read_bytes()[:5] != b"%PDF-":
        raise ValueError("paper PDF is missing, invalid, or exceeds the 5 MB limit")
    citation = (package / "CITATION.cff").read_text(encoding="utf-8")
    if 'version: "1.0.0"' not in citation or "releases/tag/v1.4.1" not in citation:
        raise ValueError("paper citation metadata lacks the expected version or release URL")
    metadata = json.loads((package / ".zenodo.json").read_text(encoding="utf-8"))
    if metadata.get("version") != "1.0.0" or "pre-review preprint" not in metadata.get("description", ""):
        raise ValueError("paper archival metadata does not identify the pre-review manuscript")
    readme = (package / "README.md").read_text(encoding="utf-8").lower()
    if "pre-review preprint" not in readme or "not peer reviewed" not in readme or "no doi" not in readme:
        raise ValueError("paper README must disclose review and DOI status")
    return actual


def build(source_root: Path, output_dir: Path) -> list[Path]:
    package = (source_root / PACKAGE_PATH).resolve(strict=True)
    if not package.is_relative_to(source_root.resolve(strict=True)):
        raise ValueError("paper package path escapes the source tree")
    expected_paths = verify_package(package)
    stem = "PDP-paper-v1.0.0"
    zip_path, checksum_path = build_zip(package, stem, output_dir, expected_paths)

    pdf_path = output_dir / f"{stem}.pdf"
    pdf_checksum_path = output_dir / f"{pdf_path.name}.sha256"
    shutil.copyfile(package / PDF_PATH, pdf_path)
    pdf_checksum_path.write_text(f"{sha256_file(pdf_path)}  {pdf_path.name}\n", encoding="utf-8")
    if pdf_path.read_bytes() != (package / PDF_PATH).read_bytes():
        raise ValueError("standalone PDF differs from the manifest-controlled paper package")
    return [zip_path, checksum_path, pdf_path, pdf_checksum_path]


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path, required=True, help="extracted source tree for the release tag")
    parser.add_argument("--output-dir", type=Path, required=True, help="directory for paper assets")
    args = parser.parse_args()
    for path in build(args.source_dir, args.output_dir):
        print(f"{path} ({path.stat().st_size} bytes)")


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, json.JSONDecodeError, csv.Error) as error:
        print(f"paper release build failed: {error}", file=sys.stderr)
        raise SystemExit(1)
