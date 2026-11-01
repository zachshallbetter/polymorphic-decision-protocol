# Polymorphic Decision Protocol

- **Package:** PDP Local Toolkit 1.4.1
- **Normative protocol:** 1.3.1
- **Rust and npm package versions:** 0.2.0
- **Status:** local implementation; no complete live-run or empirical validation claim
- **License:** MIT

This maintenance package contains the PDP Local Toolkit 1.4.0 implementation with synchronized 1.4.1 release metadata. It makes no runtime or normative protocol changes. It excludes research papers, case narratives and transcripts, internal publication notes, agent instructions, and generated LLM corpora. The protocol specification is available in the [repository source](https://github.com/zachshallbetter/polymorphic-decision-protocol/blob/v1.4.1/docs/polymorphic-decision-protocol.md).

This archive implements local decision workflow controls, but does not show that PDP improves decisions. Provider billing is not capped by the app, and outcome-blind audit is a human attestation rather than a guarantee.

## Contents and status

- `crates/pdp-core/`: mathematical routines, typed workflow records, validation, and hash-chain functions.
- `crates/pdp-local/` and `ui/local-app.*`: loopback application for encrypted records and the gated workflow through audit.
- `docs/local-toolkit.md`: setup, supported adapters, data handling, and limits.
- `docs/pdp-schema.sql`: proposed schema, not evidence of deployed permissions.
- `docs/pdp-test-002-preregistration.md`: an unexecuted research plan, not a study preregistration or result.

## Verification

Run the repository's documented test suite with `npm test`. The test result applies only to exercised software behavior and does not establish protocol conformance or empirical validity.

## Citation and identifiers

The v1.3.2 software snapshot is the historical released record: version DOI [10.5281/zenodo.23798205](https://doi.org/10.5281/zenodo.23798205); concept DOI [10.5281/zenodo.23797875](https://doi.org/10.5281/zenodo.23797875). The 1.4.1 DOI is assigned only after archival deposit.
