#!/usr/bin/env python3
"""Assemble the review monograph from its RDP Rule V9 modular sections."""
from pathlib import Path

ROOT = Path(__file__).resolve().parent
OUTPUT = ROOT / "pdp-normative-procedure-v1.0.0.md"
ORDER = [
    "sections/00-implementation-status-matrix.md",
    "sections/01-abstract.md",
    "sections/02-introduction.md",
    "sections/02-scope-and-exclusions.md",
    "sections/03-related-work.md",
    "sections/04-methods.md",
    "sections/04-formal-model.md",
    "sections/05-experiments.md",
    "sections/06-results.md",
    "sections/07-discussion.md",
    "sections/08-conclusion.md",
    "sections/09-examples.md",
    "sections/10-concept-implementation-audit.md",
]

front_matter = (ROOT / "manuscript-front-matter.yml").read_text(encoding="utf-8").rstrip()
sections = [
    (ROOT / path).read_text(encoding="utf-8").strip()
    for path in ORDER
]
OUTPUT.write_text("---\n" + front_matter + "\n---\n\n" + "\n\n".join(sections) + "\n", encoding="utf-8")
print(OUTPUT)
