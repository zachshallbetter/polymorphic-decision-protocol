# Polymorphic Decision Protocol (PDP)

- **Identifier:** `pdp`
- **Protocol version:** `1.3.1`
- **Repository release:** `1.4.1` (curated GitHub release with separate software and paper archives; no Zenodo deposit)
- **Software package version:** `0.2.0` (Rust and npm packages; versioned independently from the protocol)
- **Status:** Normative Candidate; local workflow implemented; no empirical validation
- **Question:** How should a high-uncertainty decision be made with a panel of systems while preserving human ownership of weights, stakes, and exit?

A procedure for making high-uncertainty decisions with panels of AI systems, where consensus is discounted by input provenance, model confidence is tested under controlled pressure, and the human owns the weights, the stakes, and the exit.

Stable GitHub releases build and attach the software ZIP from that version's curated manifest. Release v1.4.1 also carries the separately licensed paper v1.0.0 and its research records, plus a standalone PDF. The paper is a public pre-review preprint, not peer reviewed; author verification and some declarations remain incomplete, and no paper DOI or Zenodo deposit is claimed. The workflow does not deposit to Zenodo; archival deposits remain a separate RPP-controlled human action.

PDP is a normative protocol candidate. Its rules are documented, portions of its mathematical routines are implemented and tested, and its proposed end-to-end decision procedure has not been empirically evaluated. A private, author-reported World Cup decision informed the protocol's development; the underlying records are not included in this repository and do not establish that the protocol caused the outcome or generalizes beyond that case. Test 002 remains an unexecuted prospective plan.

## The one-sentence version

The protocol assigns evidence and argument to systems and reserves weights, stakes, and exit authority to the human. The repository documents that boundary; only a subset is implemented in code, and no end-to-end enforcement claim is made.

## Package contents

| File | What it is | Read it if |
|---|---|---|
| `docs/polymorphic-decision-protocol.md` | Normative protocol specification, version 1.3.1. This is a candidate specification, not an empirically validated intervention. | You want the protocol rules |
| `docs/pdp-architecture.md` | Design document. Components described there are not implied to be implemented. | You want the proposed architecture |
| `docs/pdp-schema.sql` | Proposed database schema. Verify its status and permissions before treating it as an operational system. | You want the proposed data model |
| `crates/pdp-core/` | Mathematical routines, typed workflow records, validation, and canonical event hashing. | You want the tested core |
| `crates/pdp-local/` and `ui/local-app.*` | macOS loopback app for encrypted decision records, evidence lanes, approved provider role calls, stress testing, human G1/G2/G3 gates, monitoring, settlement, and audit. Vendors without a supported API adapter use manual prompt and transcript import. `PDP_LOCAL_PREVIEW_ONLY=1 npm run start:local` opens a Keychain-free, in-memory demo. | You want to try the local preview |
| `docs/local-toolkit.md` | Start instructions, data handling, supported workflow, and current limits for the local preview. | You want to run or assess the toolkit |
| `docs/pdp-test-002-preregistration.md` | Prospective research plan; execution and registration commit are pending. It is not a completed study or preregistered result. | You want the planned test |

## Reading order

Read the normative specification in the repository source, then the architecture and schema as proposals. Read Test 002 as an unexecuted research plan. The curated software package does not include case narratives or research papers.

## Status and honest limits

- The originating case is an author-reported illustration with private source records unavailable for independent inspection. No empirical claim about efficacy, sycophancy prevalence, or generality is made from it.
- The performance comparison with a naive baseline is a hypothesis. No prospective test result is reported here.
- The local toolkit stores decisions and evidence in SQLCipher with its key in macOS Keychain, records validated manual transcripts and workflow records in an append-only hash-chained event history, computes ACS and formal-model summaries, and supports explicitly approved calls through OpenAI Responses, Anthropic Messages, and local Ollama. Other vendor IDs use manual prompt and transcript import; Google is paused. Brave search is transient and separately billed. Ordered stress testing, human G1/G2/G3 gates, commitment proposals, manual entry reconciliation and monitoring, settlement, and process audit are implemented. The app does not execute consequential actions or export encrypted records. Model-call cost reservations are estimates rather than provider-enforced billing caps, and outcome-blind audit is an auditor attestation. PDP v1.3.1 corrects the ACS normalization equation; toolkit behavior is not evidence of protocol effectiveness. The curated GitHub release is 1.4.1; Zenodo DOI assignment follows separate archival deposit.
- The protocol's novelty and contribution remain subject to independent literature review and expert assessment.

## Terminology

"Polymorphic" here means the panel construction: one question, asked in many forms, across systems and across evidence classes. It is unrelated to the object-oriented-programming and malware senses of the word.

## License and citation

Software and protocol repository materials are MIT-licensed. The Zenodo v1.3.2 software record (version DOI [10.5281/zenodo.23798205](https://doi.org/10.5281/zenodo.23798205), concept DOI [10.5281/zenodo.23797875](https://doi.org/10.5281/zenodo.23797875)) remains an immutable historical snapshot. The public v1.4.1 release contains a separate CC BY 4.0 preprint package: [paper ZIP](https://github.com/zachshallbetter/polymorphic-decision-protocol/releases/download/v1.4.1/PDP-paper-v1.0.0.zip), [searchable PDF](https://github.com/zachshallbetter/polymorphic-decision-protocol/releases/download/v1.4.1/PDP-paper-v1.0.0.pdf). The paper has no DOI; its Zenodo deposit is pending human review and deposit. ORCID: 0009-0009-9450-3429.
