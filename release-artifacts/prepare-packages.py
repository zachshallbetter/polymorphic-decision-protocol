#!/usr/bin/env python3
"""Build exact, manifest-controlled PDP software and paper review packages."""
from __future__ import annotations

import csv
import hashlib
import json
import shutil
import zipfile
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]
ARTIFACTS = Path(__file__).resolve().parent
SOFTWARE_MANIFEST = ARTIFACTS / "manifests" / "software-v1.4.0.csv"
PAPER_SOURCE = ROOT / "publication-package"
SOFTWARE_STAGE = ARTIFACTS / "software-v1.4.0"
PAPER_STAGE = ARTIFACTS / "paper-v1.0.0"
ARCHIVE_TIME = (2026, 11, 1, 0, 0, 0)


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def safe_relative(value: str) -> PurePosixPath:
    path = PurePosixPath(value)
    if path.is_absolute() or not path.parts or any(part in (".", "..") for part in path.parts):
        raise ValueError(f"unsafe package path: {value!r}")
    return path


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
    for path in sorted(p for p in package.rglob("*") if p.is_file() and p != manifest):
        rel = path.relative_to(package).as_posix()
        if rel == "archive/checksums.sha256":
            description, kind = "SHA-256 inventory of all package payload files", "archive"
        elif rel.startswith("manuscript/") or rel.startswith("research/paper/"):
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
        writer = csv.writer(stream)
        writer.writerow(("path", "description", "type"))
        writer.writerows(rows)


def build_software() -> None:
    if SOFTWARE_STAGE.exists():
        shutil.rmtree(SOFTWARE_STAGE)
    SOFTWARE_STAGE.mkdir(parents=True)
    seen: set[str] = set()
    manifest_rows = []
    with SOFTWARE_MANIFEST.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        for row in reader:
            rel = safe_relative(row["path"])
            key = rel.as_posix()
            if key in seen:
                raise ValueError(f"duplicate software package path: {key}")
            seen.add(key)
            source_value = row["source"]
            source = ROOT / source_value
            if source_value == "release-artifacts/manifests/software-v1.4.0.csv":
                continue
            if not source.is_file():
                raise FileNotFoundError(f"software manifest source missing: {source_value}")
            destination = SOFTWARE_STAGE.joinpath(*rel.parts)
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
            manifest_rows.append((key, row["description"], row["type"], row["license"]))

    manifest_path = SOFTWARE_STAGE / "metadata" / "manifest.csv"
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    manifest_rows.extend([
        ("metadata/manifest.csv", "Curated software package manifest", "metadata", "MIT"),
        ("archive/checksums.sha256", "SHA-256 inventory of all package payload files", "archive", "MIT"),
    ])
    with manifest_path.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow(("path", "description", "type", "license"))
        writer.writerows(manifest_rows)

    write_checksums(SOFTWARE_STAGE)


def verify_paper_source() -> None:
    source_md = ROOT / "research" / "paper" / "pdp-normative-procedure-v1.0.0.md"
    rpp_md = PAPER_SOURCE / "manuscript" / "paper.md"
    if source_md.read_bytes() != rpp_md.read_bytes():
        raise ValueError("RDP source and RPP compatibility manuscript differ")
    source_bib = ROOT / "research" / "paper" / "latex" / "pdp-normative-procedure-v1.0.0-references.bib"
    rpp_bib = PAPER_SOURCE / "manuscript" / "references.bib"
    if source_bib.read_bytes() != rpp_bib.read_bytes():
        raise ValueError("RDP bibliography and RPP compatibility bibliography differ")


def build_paper() -> None:
    verify_paper_source()
    write_paper_manifest(PAPER_SOURCE)
    write_checksums(PAPER_SOURCE)
    if PAPER_STAGE.exists():
        shutil.rmtree(PAPER_STAGE)
    shutil.copytree(PAPER_SOURCE, PAPER_STAGE)


def build_zip(package: Path, name: str) -> Path:
    output = ARTIFACTS / f"{name}.zip"
    if output.exists():
        output.unlink()
    with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for path in sorted(p for p in package.rglob("*") if p.is_file()):
            rel = path.relative_to(package).as_posix()
            info = zipfile.ZipInfo(f"{name}/{rel}", ARCHIVE_TIME)
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            archive.writestr(info, path.read_bytes(), compress_type=zipfile.ZIP_DEFLATED, compresslevel=9)
    (ARTIFACTS / f"{name}.zip.sha256").write_text(
        f"{digest(output)}  {output.name}\n", encoding="utf-8"
    )
    return output


def main() -> None:
    build_software()
    build_paper()
    software_zip = build_zip(SOFTWARE_STAGE, "PDP-software-v1.4.0")
    paper_zip = build_zip(PAPER_STAGE, "PDP-paper-v1.0.0")
    print(f"software package: {software_zip} ({software_zip.stat().st_size} bytes)")
    print(f"paper package:    {paper_zip} ({paper_zip.stat().st_size} bytes)")
    print("No publication or deposit action was performed.")


if __name__ == "__main__":
    main()
