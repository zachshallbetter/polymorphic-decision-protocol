# Workspace Agent Rules: Polymorphic Decision Protocol (PDP)

These rules guide research, specification design, software engineering, database modeling, and eval pre-registration within the **Polymorphic Decision Protocol (PDP)** repository.

---

## 1. Guiding Principle

The core boundary of this protocol is invariant:
> **The systems supply evidence and argument. The human owns the weights, the stakes, and the exit. Everything in this package is enforcement of that boundary.**

Do not evaluate PDP merely as a post-hoc case study or an LLM consensus aggregator. Evaluate whether it succeeds in establishing a rigorous, falsifiable, and auditable procedure for high-uncertainty decision-making across heterogeneous AI panels. All editorial and implementation decisions must reinforce that objective.

---

## 2. Architectural Layers

The repository corpus is framed around three distinct layers:

1. **Layer 1 — The Protocol Specification (`docs/polymorphic-decision-protocol.md`):** The normative specification (v1.x). Defines the four core phases plus portfolio governance: panel formation with two-axis consensus discounting (ACS), ordered pressure testing with pressure-response classification, single-window commitment with pre-committed invalidation triggers, and outcome-blind audit with a persistent calibration ledger.
2. **Layer 2 — The Reference Implementation Architecture (`docs/pdp-architecture.md`, `docs/pdp-schema.sql`):** The software and database substrate. Rust workspace (`crates/`), Postgres system of record (`pdp-schema.sql`), Web Components UI (`ui/`), multi-provider panel engine with evidence-class tool allowlists, cross-vendor flip classification, hash-chained event log, and read-only market integration design.
3. **Layer 3 — Empirical Evals & Preregistration (`docs/pdp-test-002-preregistration.md`, `docs/pdp-fixtures-capture.md`):** Prospective empirical trials, capture rules, eval fixtures (`pdp-evals/fixtures/`), pressure transcripts, counterfactual tracking, and Brier score calibration accounting.

---

## 3. The Epistemological Questions

Every component of PDP answers a necessary epistemological question to ensure decision making under uncertainty remains scientifically meaningful and auditable:

* **Panel Formation & Discounting:** *How is consensus discounted by evidence-class provenance and cross-model vendor dependence?*
* **Adversarial Probing:** *How is model confidence tested under controlled pressure without human bias or self-grading?*
* **Weight Custody:** *How does frozen weight custody prevent post-hoc rationalization while guaranteeing a single human exit window?*
* **Observation & Evidence:** *What constitutes an empirical flip, and what boundary separates system argument from human decision ownership?*
* **Calibration & Auditability:** *What can be measured and compared over time across settlement horizons?*

---

## 4. Editorial Guidelines & Status Honesty

### 4.1 Strict Four-Tier Claim Taxonomy
Always enforce strict status honesty across all documentation, commits, and PRs:
$$\text{Documented} \neq \text{Implemented} \neq \text{Tested} \neq \text{Empirically Validated}$$

* **Documented:** Features, protocols, or schemas written in markdown specs or RFCs.
* **Implemented:** Code or DDL schemas written and integrated into the repository.
* **Tested:** Units, fixtures, or integration tests passing in CI/eval suites.
* **Empirically Validated:** Preregistered real-world trials with recorded settlements and Brier score audits.

When in doubt, state the weaker claim. Never describe a feature as "empirically validated" based solely on an $n=1$ retrospective case study.

### 4.2 Professional Scientific Language
* Avoid hype, marketing fluff, or uncritical AI hype.
* Prefer precise protocol terminology: *ACS consensus discounting*, *pressure-response classification*, *single-window commitment*, *frozen weight custody*, *Brier score calibration*.

---

## 5. Research Program & Scientific Boundaries

### 5.1 Mission & Central Question
* **Mission:** Establish whether multi-model decision processes under high uncertainty can be governed by pre-registered, falsifiable rules that resist model sycophancy and enforce human exit ownership.
* **Central Question:** Can AI panel recommendations be systematically discounted by input provenance and tested under adversarial pressure to produce provably superior calibrated decisions compared to naive single-model or un-discounted consensus?

### 5.2 Negative Space — What PDP Is Not
* PDP is **not**:
  * An automated trading bot or order-execution engine (Kalshi integration is strictly read-only by design; order execution has no code path).
  * A generic agent orchestration framework or workflow builder.
  * An LLM voting or un-weighted polling mechanism.
  * A collection of prompt engineering heuristics or conversational UI wrappers.

---

## 6. Repository Rules & Infrastructure

### 6.1 Authority Hierarchy
When documentation, code, generated artifacts, or AI outputs conflict, resolve in strict canonical order:
$$\text{Protocol Specification} \to \text{Architecture Design} \to \text{Database Schema (DDL)} \to \text{Rust/UI Code} \to \text{Generated Artifacts}$$

Code or generated artifacts **never** silently alter protocol specifications or database immutability constraints.

### 6.2 Database Schema Discipline
* Weight custody rows are strictly **frozen upon creation**: Postgres DDL must enforce `INSERT`-only access with no `UPDATE` grant on weight custody tables.
* Append-only event logs, pressure test records, and fill records must be secured via database permissions and hash chains.

### 6.3 Agent-First Publishing Architecture (`llms.txt`)
* Maintain programmatically generated `llms.txt` and `llms-full.txt` single-file context corpora using `python3 scripts/gen-llms.py`.
* Run `python3 scripts/gen-llms.py` whenever documentation, specifications, or schemas are updated.

---

## 7. Quality Gates & Publication Rules

* **Preregistration Mandatory:** Any new decision evaluation trial must be pre-registered (stating panel, sequence, capture rules, classification scheme, counterfactuals, and failure conditions) *before* execution.
* **Outcome-Blind Audit:** Post-settlement calibration audits must evaluate predictions against pre-registered counterfactuals without retroactive modification of weights or hypotheses.
* **Negative Results Permanent:** Disproven hypotheses or failed panel configurations must be recorded permanently in evals and calibration ledgers.

---

## 8. Agent Execution Methodology & Forward-Thinking Completion Engine

To ensure all research, coding, documentation, and refactoring tasks are executed with maximum intelligence and completed to end-to-end working state, agents must strictly follow these 5 core execution rules:

### 8.1 Ambient Adaptation & Engineering Judgment
* **Style & Convention Matching**: Read surrounding files before editing. Match comment density, naming conventions, architectural idioms, and structural patterns.
* **Root-Cause Resolution**: Inspect actual log/test outputs and trace upstream data flow. Address underlying logic errors rather than applying superficial symptom patches or suppressing errors.

### 8.2 Autonomous Execution to Completion
* **End-to-End Ownership**: Do not halt halfway or stop at draft code/plans. Drive tasks through research, implementation, build validation, and verification until the requested outcome is fully realized.
* **Self-Correction & Autonomous Momentum**: If a chosen path hits a blocker, evaluate alternative approaches immediately. Log deviations in `scratch/implementation-notes.md` and proceed with the best alternative rather than halting execution to ask trivial questions.

### 8.3 Unknowns Discovery Framework (Blind Spot Pass)
Map tasks across the 4 Context Quadrants before and during execution:
1. **Known Knowns**: Explicit user requirements in the prompt.
2. **Known Unknowns**: Identified gaps requiring investigation before coding.
3. **Unknown Knowns**: Implicit expectations and visual/architectural standards.
4. **Unknown Unknowns**: Unconsidered architectural traps, security boundaries, or edge cases.

* **Pre-Implementation Blind Spot Pass**: Scan the codebase for hidden dependencies and domain risks before proposing solutions.

### 8.4 Live Implementation Ledger (`scratch/implementation-notes.md`)
For non-trivial tasks, maintain a scratch ledger at `<appDataDir>/brain/<conversation-id>/scratch/implementation-notes.md` recording decisions, rationale, deviations, and verification proof.

### 8.5 Empirical Runtime Verification
* **No Premature Success Claims**: Never declare completion based solely on un-verified code edits.
* **Validation Suite**: Always run empirical verification (e.g. `python3 scripts/gen-llms.py`, `cargo test`, `npm test`, DDL validation) before finishing a task.
