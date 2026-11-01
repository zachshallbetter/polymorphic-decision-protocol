#!/usr/bin/env python3
"""Generate llms.txt and llms-full.txt for the Polymorphic Decision Protocol (PDP) corpus.

`llms.txt` is an index (llmstxt.org): what this protocol is, what it does not claim, and where each
document lives. `llms-full.txt` concatenates the **project-licensed corpus** into one file.

Generates versioned llms index and full codebase corpus files in `.agents/` and top-level root:
  .agents/llms.txt
  .agents/llms-pdp-v<version>.txt
  .agents/llms-full.txt
  .agents/llms-full-pdp-v<version>.txt
"""
from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
AGENTS_DIR = ROOT / ".agents"
SITE = "https://github.com/zachshallbetter/polymorphic-decision-protocol/blob/main"

# Project-licensed docs & specs — safe to inline.
INCLUDE_DIRS = [
    ("docs", "Protocol specifications, architecture, SQL schema, essay draft, pre-registrations, and e-val fixtures"),
]

ROOT_DOCS = [
    "README.md", "AGENTS.md", ".agents/AGENTS.md", "docs/local-toolkit.md"
]

EXCLUDE: dict[str, str] = {}


def get_version() -> str:
    metadata = json.loads((ROOT / ".zenodo.json").read_text(encoding="utf-8"))
    version = metadata.get("version")
    if not isinstance(version, str) or not version.strip():
        raise ValueError(".zenodo.json must declare the repository release version")
    return version.strip().lstrip("v")

def is_excluded(rel: str) -> str | None:
    for path, reason in EXCLUDE.items():
        if rel == path or rel.startswith(path + "/"):
            return reason
    return None


def tracked_markdown_and_docs() -> list[Path]:
    """Tracked files matching docs, schema, and root guidelines."""
    files: set[Path] = set()
    try:
        out = subprocess.run(["git", "-C", str(ROOT), "ls-files", "-z"],
                             capture_output=True, check=True, text=True).stdout
        for n in out.split("\0"):
            if not n:
                continue
            p = ROOT / n
            if p.suffix in (".md", ".sql"):
                files.add(p)
    except (subprocess.CalledProcessError, FileNotFoundError):
        for ext in ("*.md", "*.sql"):
            files.update(ROOT.rglob(ext))

    # Always ensure root docs exist in the candidate list
    for doc in ROOT_DOCS:
        p = ROOT / doc
        if p.exists():
            files.add(p)

    return sorted(list(files))


def h1_of(path: Path) -> str:
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if line.startswith("# "):
            return line[2:].strip()
    return path.stem.replace("_", " ").replace("-", " ").title()


def main() -> int:
    version = get_version()
    agents_dir = AGENTS_DIR
    agents_dir.mkdir(parents=True, exist_ok=True)

    all_docs = tracked_markdown_and_docs()
    included: list[Path] = []
    skipped: list[tuple[str, str]] = []

    allowed_prefixes = [d for d, _ in INCLUDE_DIRS] + [".agents"]

    for p in all_docs:
        rel = p.relative_to(ROOT).as_posix()
        reason = is_excluded(rel)
        if reason:
            skipped.append((rel, reason))
            continue
        if rel in ROOT_DOCS or any(rel.startswith(pre + "/") for pre in allowed_prefixes):
            included.append(p)
        else:
            skipped.append((rel, "unlisted doc path"))

    included.sort(key=lambda p: p.relative_to(ROOT).as_posix())

    # ---- llms.txt (index)
    idx = [
        "# Polymorphic Decision Protocol (PDP)",
        f"> Version: v{version} | Total Included Documents: {len(included)}",
        "",
        "> A procedure for making high-uncertainty decisions with panels of AI systems, where",
        "> consensus is discounted by input provenance, model confidence is tested under controlled",
        "> pressure, and the human owns the weights, the stakes, and the exit.",
        "",
        "The systems supply evidence and argument. The human owns the weights, the stakes, and the exit.",
        "Everything in this package is enforcement of that boundary.",
        "",
        f"- [llms-full.txt]({SITE}/llms-full.txt): the complete PDP corpus concatenated into one file",
        f"- [llms-full-pdp-v{version}.txt]({SITE}/llms-full-pdp-v{version}.txt): versioned corpus payload",
        "",
    ]
    for d, desc in INCLUDE_DIRS:
        docs = [p for p in included if p.relative_to(ROOT).as_posix().startswith(d + "/")]
        if not docs:
            continue
        idx += [f"## {Path(d).name.replace('_', ' ').title()}", "", f"{desc}.", ""]
        for p in docs:
            rel = p.relative_to(ROOT).as_posix()
            idx.append(f"- [{h1_of(p)}]({SITE}/{rel})")
        idx.append("")

    # Root docs
    root_in = [p for p in included if "/" not in p.relative_to(ROOT).as_posix() or p.relative_to(ROOT).as_posix().startswith(".agents/")]
    if root_in:
        idx += ["## Project Guides & Metadata", "", "Root-level specifications and agent rules.", ""]
        for p in root_in:
            rel = p.relative_to(ROOT).as_posix()
            idx.append(f"- [{h1_of(p)}]({SITE}/{rel})")
        idx.append("")

    idx_text = "\n".join(idx)

    # ---- llms-full.txt
    full = [
        "# Polymorphic Decision Protocol — full corpus",
        f"Version: v{version} | Total Included Documents: {len(included)}",
        "",
        "Specification, architecture, schema, case narrative, pre-registrations, and eval fixtures.",
        "",
        "The systems supply evidence and argument. The human owns the weights, the stakes, and the exit.",
        "", "=" * 78, "",
    ]
    for p in included:
        rel = p.relative_to(ROOT).as_posix()
        full += [f"<!-- {rel} -->", "", p.read_text(encoding="utf-8", errors="replace").rstrip(), "", "=" * 78, ""]

    full_text = "\n".join(full)

    # Write to .agents/
    (agents_dir / "llms.txt").write_text(idx_text, encoding="utf-8")
    (agents_dir / f"llms-pdp-v{version}.txt").write_text(idx_text, encoding="utf-8")
    (agents_dir / "llms-full.txt").write_text(full_text, encoding="utf-8")
    (agents_dir / f"llms-full-pdp-v{version}.txt").write_text(full_text, encoding="utf-8")

    # Write to root (for backwards compatibility / direct visibility)
    (ROOT / "llms.txt").write_text(idx_text, encoding="utf-8")
    (ROOT / f"llms-pdp-v{version}.txt").write_text(idx_text, encoding="utf-8")
    (ROOT / "llms-full.txt").write_text(full_text, encoding="utf-8")
    (ROOT / f"llms-full-pdp-v{version}.txt").write_text(full_text, encoding="utf-8")

    kib = (agents_dir / "llms-full.txt").stat().st_size // 1024
    print(f"✓ gen-llms: generated .agents/llms.txt, .agents/llms-pdp-v{version}.txt and .agents/llms-full.txt, .agents/llms-full-pdp-v{version}.txt ({kib} KiB) — {len(included)} documents included")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
