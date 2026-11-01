#!/usr/bin/env python3
"""Shared deterministic ZIP writer for curated release packages."""
from __future__ import annotations

import hashlib
import shutil
import zipfile
from pathlib import Path

ARCHIVE_TIME = (2026, 11, 1, 0, 0, 0)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def build_zip(package_root: Path, archive_stem: str, output_dir: Path, expected_paths: set[str]) -> tuple[Path, Path]:
    files = {path.relative_to(package_root).as_posix() for path in package_root.rglob("*") if path.is_file()}
    if files != expected_paths:
        raise ValueError("package tree differs from the validated manifest")

    output_dir.mkdir(parents=True, exist_ok=True)
    zip_path = output_dir / f"{archive_stem}.zip"
    checksum_path = output_dir / f"{zip_path.name}.sha256"
    with zipfile.ZipFile(zip_path, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for path in sorted(package_root.rglob("*")):
            if not path.is_file():
                continue
            relative = path.relative_to(package_root).as_posix()
            info = zipfile.ZipInfo(f"{archive_stem}/{relative}", ARCHIVE_TIME)
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            with path.open("rb") as source, archive.open(info, "w") as target:
                shutil.copyfileobj(source, target, 1024 * 1024)

    checksum_path.write_text(f"{sha256_file(zip_path)}  {zip_path.name}\n", encoding="utf-8")
    with zipfile.ZipFile(zip_path) as archive:
        if archive.testzip() is not None:
            raise ValueError("generated release archive failed its CRC integrity check")
        expected_names = {f"{archive_stem}/{relative}" for relative in expected_paths}
        if set(archive.namelist()) != expected_names:
            raise ValueError("generated release archive file list differs from the manifest")
    if checksum_path.read_text(encoding="utf-8") != f"{sha256_file(zip_path)}  {zip_path.name}\n":
        raise ValueError("release ZIP sidecar checksum does not match the ZIP")
    return zip_path, checksum_path
