"""Validate the protocol, repository release, and software package versions."""
from __future__ import annotations

import json
import re
import tomllib
from pathlib import Path

STABLE_VERSION = r"(?:0|[1-9][0-9]*)(?:\.(?:0|[1-9][0-9]*)){2}"
SPECIFICATION = "docs/polymorphic-decision-protocol.md"
CRATES = ("pdp-core", "pdp-local")
ARCHIVE_README = "release-artifacts/software-readme.md"


def read_text(root: Path, path: str) -> str:
    return (root / path).read_text(encoding="utf-8")


def read_json(root: Path, path: str) -> dict:
    return json.loads(read_text(root, path))


def read_toml(root: Path, path: str) -> dict:
    return tomllib.loads(read_text(root, path))


def version_field(text: str, pattern: str, location: str) -> str:
    matches = re.findall(pattern, text, re.MULTILINE)
    if len(matches) != 1:
        raise ValueError(f"{location} must declare exactly one version")
    return matches[0]


def readme_version(text: str, label: str, path: str, *, backticks: bool = False) -> str:
    pattern = r"^`([^\x60\r\n]+)`(?:[ \t].*)?$" if backticks else r"^(\S+)$"
    location = f"{path} ({label})"
    field = version_field(text, rf"^- \*\*{re.escape(label)}:\*\* ([^\r\n]+)$", location)
    return version_field(field, pattern, location)


def require_version(location: str, value: object, source: str, expected: str) -> None:
    if value != expected:
        raise ValueError(f"{location} is {value!r}, expected {expected!r} from {source}")


def stable_version(location: str, value: object) -> str:
    if not isinstance(value, str) or re.fullmatch(STABLE_VERSION, value) is None:
        raise ValueError(f"{location} must contain a stable MAJOR.MINOR.PATCH version, got {value!r}")
    return value


def repository_version(root: Path) -> str:
    release = stable_version(".zenodo.json (version)", read_json(root, ".zenodo.json").get("version"))
    citation = version_field(read_text(root, "CITATION.cff"), r"^version:[ \t]*(.*)$", "CITATION.cff")
    if len(citation) >= 2 and citation[0] in "\"'" and citation[-1] == citation[0]:
        citation = citation[1:-1]
    require_version("CITATION.cff (version)", citation, ".zenodo.json", release)
    return release


def protocol_version_file(root: Path) -> str:
    return stable_version("VERSION", read_text(root, "VERSION").removesuffix("\n"))


def software_version(root: Path) -> str:
    software = stable_version("package.json (version)", read_json(root, "package.json").get("version"))
    npm_lock = read_json(root, "package-lock.json")
    require_version("package-lock.json (version)", npm_lock.get("version"), "package.json", software)
    require_version(
        'package-lock.json (packages[""].version)',
        npm_lock.get("packages", {}).get("", {}).get("version"),
        "package.json",
        software,
    )
    cargo_lock = read_toml(root, "Cargo.lock")
    for crate in CRATES:
        path = f"crates/{crate}/Cargo.toml"
        package = read_toml(root, path).get("package", {})
        require_version(f"{path} (package.version)", package.get("version"), "package.json", software)
        entries = [entry for entry in cargo_lock.get("package", []) if entry.get("name") == crate]
        if len(entries) != 1:
            raise ValueError(f"Cargo.lock must contain exactly one entry for {crate}")
        require_version(f"Cargo.lock ({crate})", entries[0].get("version"), path, software)
    return software


def validate_checkout(root: Path) -> tuple[str, str, str]:
    heading = version_field(
        read_text(root, SPECIFICATION),
        r"^# Polymorphic Decision Protocol \(PDP\) v(\S+)$",
        SPECIFICATION,
    )
    protocol = stable_version(SPECIFICATION, heading)
    require_version("VERSION", protocol_version_file(root), SPECIFICATION, protocol)
    release = repository_version(root)
    software = software_version(root)
    readme = read_text(root, "README.md")
    for label, source, expected in (
        ("Protocol version", SPECIFICATION, protocol),
        ("Repository release", ".zenodo.json", release),
        ("Software package version", "package.json", software),
    ):
        actual = readme_version(readme, label, "README.md", backticks=True)
        require_version(f"README.md ({label})", actual, source, expected)
    return protocol, release, software


def validate_archive_versions(root: Path, versions: tuple[str, str, str]) -> None:
    protocol, release, software = versions
    require_version("archive VERSION", protocol_version_file(root), SPECIFICATION, protocol)
    require_version("archive .zenodo.json (version)", repository_version(root), "release tag", release)
    require_version("archive package.json (version)", software_version(root), "source package.json", software)
    readme = read_text(root, "README.md")
    label = "Repository release"
    toolkit = version_field(readme, r"^- \*\*Repository release:\*\* PDP Local Toolkit (\S+)$", ARCHIVE_README)
    require_version(f"{ARCHIVE_README} ({label})", toolkit, "release tag", release)
    for label, source, expected in (
        ("Normative protocol", SPECIFICATION, protocol),
        ("Rust and npm package versions", "package.json", software),
    ):
        actual = readme_version(readme, label, ARCHIVE_README)
        require_version(f"{ARCHIVE_README} ({label})", actual, source, expected)


if __name__ == "__main__":
    try:
        protocol, release, software = validate_checkout(Path(__file__).resolve().parents[1])
    except (OSError, ValueError) as error:
        raise SystemExit(f"Version validation failed: {error}") from None
    print(f"Protocol {protocol}, repository release {release}, software packages {software}")
