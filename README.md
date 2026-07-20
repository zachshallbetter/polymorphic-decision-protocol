# Polymorphic Decision Protocol (PDP)

A procedure for making high-uncertainty decisions with panels of AI systems, where consensus is discounted by input provenance, model confidence is tested under controlled pressure, and the human owns the weights, the stakes, and the exit.

The protocol came out of a settled real-money case: a 2026 World Cup thesis formed across Claude, Gemini, and ChatGPT, staked as an eight-market ladder on Kalshi ($10 → $58.86), and held through a documented event in which one panel system abandoned the correct pick under nothing but expressed displeasure, retracted when challenged, four days before the market it had switched to settled at zero. The protocol formalizes what held, closes what broke, and pre-registers the next test.

## The one-sentence version

The systems supply evidence and argument. The human owns the weights, the stakes, and the exit. Everything in this package is enforcement of that boundary.

## Package contents

| File | What it is | Read it if |
|---|---|---|
| `polymorphic-decision-protocol.md` | **The specification (v1.3).** Four phases plus portfolio governance: panel formation with two-axis consensus discounting (ACS), ordered stress testing with pressure-response classification, single-window commitment with pre-committed invalidation, outcome-blind audit with a persistent calibration ledger. Formulas, decision matrix, failure taxonomy, prompt library, and the full case woven through every section. | You want the method |
| `pdp-architecture.md` | **The reference implementation design (v0.3).** Rust workspace (axum, sqlx, tokio), Postgres system of record, Web Components UI, multi-provider panel engine with evidence-class tool allowlists, cross-vendor flip classification, hash-chained event log, failover matrix, degraded-mode ladder, eval framework, cost model. Kalshi integration is read-only by design; order execution has no code path. | You want to build it |
| `pdp-schema.sql` | **The database schema.** Postgres DDL matching the architecture: pipeline state, weight custody (one frozen row per decision, no UPDATE grant), pressure tests with no-self-grading vendor constraint, portfolio correlation groups, append-only ledger/events/fills enforced by grants, hash chain, costs. | You want the data model |
| `pdp-essay-draft.md` | **The case narrative.** "The Model Was Right Until I Disagreed." Long-form account of the panel, the thesis, the ladder, the July 10 reversal and retraction, and the three conditions that made the capture possible. Includes the evidence note and references. | You want the story |
| `pdp-protocol-article.md` | **The methodology article.** The protocol explained on its own terms, phase by phase, with the case as compressed reference. Companion to the essay. | You want the argument |
| `pdp-test-002-preregistration.md` | **The pre-registration.** Panel, sequence, capture rules, classification scheme, counterfactual, and audit for the next protocol-governed decision, fixed before the decision exists, with failure conditions named in advance and a publish-regardless commitment. | You want the falsifiability |
| `pdp-fixtures-capture.md` | **The eval fixture record.** Fixture 001 (the documented Claude flip, human-verified) plus the LOST markers for the Gemini/ChatGPT pressure transcripts and the prospective capture protocol that replaces retrospective recovery. Seed data for `pdp-evals/fixtures/flips/`. | You want the training data |
| `pdp-publication-brief.md` | **The publication plan.** Novelty positioning against the located literature, claim-scope decisions, venue sequencing, risk register. | You want the context |

## Reading order

For the method: essay → protocol article → specification. For the build: specification → architecture → schema. The pre-registration stands alone and is the package's strongest validity instrument: it converts a retrospective story into a prospective, falsifiable program.

## Status and honest limits

- The documented pressure-driven flip is one system, one instance (n=1). The cross-vendor pressure comparison is designed but not yet run under capture discipline; Test 002 runs it.
- The performance claim (protocol picks beat naive picks) is a hypothesis with a recording apparatus, not a result. Every decision records its naive counterfactual at thesis time; Brier scores accumulate across settlements.
- The case win is, by the protocol's own required accounting, 80% variance.
- Individual components have research neighbors (see references in the essay). The assembly is, as far as located, novel. Corrections welcome.

## Terminology

"Polymorphic" here means the panel construction: one question, asked in many forms, across systems and across evidence classes. It is unrelated to the object-oriented-programming and malware senses of the word.

## License and citation

Specification and articles © 2026 Zach Shallbetter. A DOI for the specification (v1.3) is planned via the author's deposit pipeline; until minted, cite the specification by title, version, and https://zachshallbetter.com. ORCID: 0009-0009-9450-3429.
