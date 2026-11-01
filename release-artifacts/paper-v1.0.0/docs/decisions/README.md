# Research and publication decisions

## D-01 — Project profile

Use RDP's Project profile: a real author-reported case informed a protocol, but no prospective empirical test is complete. Study-level claims and apparatus would overstate the evidence.

## D-02 — Type II protocol paper

Prepare one protocol-specification paper. Defer a separate empirical case paper because source records are private and unavailable for independent review. The case may appear only as an explicitly unverified author-reported development illustration.

## D-03 — Baseline freeze

Analyze protocol v1.3.0 at commit `35d945a04e622a560c8e1be53ffc0a796be1aa79`. This is the historical baseline, not a claim that the current working tree is committed or frozen.

## D-04 — RDP/RPP structure composition

RDP Rule V9 requires a semantic-title monograph in `research/paper/` with modular sections 00–10 and matched Markdown/PDF. RPP's mechanical checker expects `manuscript/paper.md` and `manuscript/references.bib`. Keep RDP as the scholarly source of truth and place byte-identical compatibility copies in those RPP paths. Verify SHA-256 parity before archive preparation. The RPP staging package is not a second manuscript authority.

## D-05 — Separate package licenses

The software repository remains MIT. The standalone paper package is CC BY 4.0. No software source is relicensed by copying it into the paper package; package manifests declare the scope of each license.

## D-06 — Private evidence

Private case records and transcripts stay outside the repository and both packages. The paper contains only a bounded author-reported description and assigns it no evidential weight for efficacy or generality.

## D-07 — Archival deposit gate

No Zenodo deposit or site deployment until both independent reviews are received, author declarations are confirmed, cited records are verified, exact manifests and checksums pass review, and the human owner performs final readiness review and deposit. D-13 authorizes the separately labeled GitHub pre-review preprint and is a documented exception to the earlier no-public-release condition.

## D-08 — Current manuscript status

Version 1.0.0 is a public pre-review preprint released through GitHub release v1.4.1 on 2026-11-01. It is not independently reviewed or peer reviewed. DOI is unassigned. Do not mint or claim identifiers in advance.

## D-13 — Public pre-review preprint release

On 2026-11-01, the author explicitly authorized public release of paper version 1.0.0 with PDP software release v1.4.1 as a pre-review preprint, overriding the planned independent-review and author-verification gates for this GitHub release. This decision does not establish peer review, independent review, author verification, completion of CRediT/funding/additional competing-interest declarations, empirical validity, DOI assignment, or Zenodo deposit. The release must identify those limits and preserve the outstanding archival-readiness checks.

## D-09 — CRediT and funding declarations

The author identity and Independent Researcher affiliation are established. Specific CRediT roles, funding status, and any additional competing interests have not been confirmed and remain archival-readiness blockers; no declaration is inferred.

## D-10 — RDP checker section taxonomy

The checker searches for legacy labels such as `methods`, `experiments`, `results`, and `conclusion`, while Rule V9 specifies semantic sections 00–10. Keep Rule V9 source files and report any checker mismatch as a tooling limitation; do not add empty or misleading sections solely to obtain a green mechanical result.

## D-11 — RDP manuscript compiler metadata and bibliography lookup

The documented compiler failed to parse YAML front matter and emitted it as body text; it also added automatic numbering to already numbered RDP headings. Its Markdown reader now enables YAML metadata blocks and no longer adds duplicate automatic numbering. Bibliography lookup now checks the normative `latex/` location, removing the need for a second bibliography copy in the RDP paper directory. These are tool corrections only; no research protocol rule changed.

## D-12 — Paper overview filename

RDP Rule V9 names `Paper Overview.md`, while RPP requires lowercase-kebab-case except for contractual `README.md`. Use `research/paper/README.md` for the overview; the RDP paper-directory checker explicitly accepts `README.md`. This preserves the overview function and avoids a nonconforming filename in the RPP archive.
