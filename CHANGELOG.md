# Changelog

## 1.4.1 — 2026-11-01

- Added manifest-validated GitHub release packaging that builds and attaches the software ZIP, paper/research package ZIP, standalone paper PDF, and SHA-256 sidecars without replacing mismatched assets.
- Added archive checks for exact manifest parity, checksums, ZIP integrity, and version agreement across `.zenodo.json`, `CITATION.cff`, and the package README.
- Added paper version 1.0.0 as a public pre-review preprint. It is not independently reviewed or peer reviewed; no paper DOI or Zenodo deposit is claimed, and author verification and declarations remain incomplete.
- No runtime or normative protocol changes; protocol baseline remains v1.3.1, and Rust/npm package versions remain 0.2.0.

## PDP protocol 1.3.1

- Corrected the ACS denominator to the total number of panel agents, including nonmodal agents. This matches the worked example and the reference implementation; the prior displayed sum-of-weights denominator contradicted both.
- The change clarifies the existing intended calculation. It does not validate ACS as a measure of independence or predictive reliability.

## 1.4.0 — 2026-11-01

- Added a macOS loopback service and browser interface for decision setup, source-linked lane evidence, panel/model pinning, and manual transcript entry.
- Added SQLCipher storage with a database key held in macOS Keychain, append-only hash-chained event records, local-origin checks, and bounded request bodies.
- Added hard checks for evidence lane coverage, role fit, pinned models, and all 12 vendor-role transcripts. G1 computes ACS, records structured human review, and requires approval before ordered stress testing.
- Added per-call preview and approval for OpenAI Responses, Anthropic Messages, and local Ollama; manual prompt/transcript import supports other vendor interfaces. Google is paused. Search uses transient, per-call approved Brave results.
- Added typed self-critique, frozen formal model, pressure-test, G2 review, G3 commitment, entry reconciliation, monitoring, settlement, and audit records. Stage transitions require the prior validated record and human gates.
- The toolkit targets PDP v1.3.1; Rust and npm package versions are 0.2.0. Billing reservations are estimates, search charges are separate, and the application does not execute consequential actions or export encrypted records.
- Unit and package tests do not establish a complete live provider-to-audit run or empirical validity.

## 1.3.3 — prepared 2026-10-31; not released

- Corrected repository descriptions to distinguish normative documentation, design proposals, implemented code, unit tests, and empirical validation.
- Repaired the Docker entry point to invoke the built `pdp-cli` binary.
- Curated a release manifest that excludes drafts, agent instructions, raw case records, and generated LLM corpora.
- No normative protocol rule changes. Protocol version remains 1.3.0; Rust and npm package versions remain 0.1.0.
- No GitHub release, tag, Zenodo deposit, or public site deployment has occurred.
