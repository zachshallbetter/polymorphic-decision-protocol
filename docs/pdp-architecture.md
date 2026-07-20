# PDP Engine — Architecture v0.3

Rust service implementing the Polymorphic Decision Protocol (PDP v1.2) as an automated pipeline with hard human gates. Postgres as the system of record. Web Components UI. Multi-provider agent panel (Anthropic, OpenAI, Google). Kalshi read-only market integration; order execution deliberately out of scope for v1.

v0.2 adds: failure-domain map and failover matrix (§11), degraded-mode ladder (§12), evaluation framework (§13), testing strategy (§14), operational runbook triggers (§15).
v0.3 adds: cost model and budget enforcement (§16); portfolio governance support for PDP v1.3 §4A (decision_groups, shared-observable dedupe) noted for the schema in a future revision.

---

## 1. System overview

```
┌────────────────────────────────────────────────────────────────┐
│                        pdp-web  (UI)                           │
│   Web Components, no framework · SSE for live pipeline state   │
└──────────────────────────┬─────────────────────────────────────┘
                           │ HTTP/JSON + SSE
┌──────────────────────────▼─────────────────────────────────────┐
│                      pdp-server (Rust)                         │
│  axum HTTP API · pipeline state machine · gate enforcement     │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────────────────┐ │
│  │ panel    │ │ stress   │ │ commit   │ │ monitor            │ │
│  │ engine   │ │ engine   │ │ composer │ │ daemon (tokio)     │ │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └─────────┬──────────┘ │
└───────┼────────────┼────────────┼─────────────────┼────────────┘
        │            │            │                 │
┌───────▼────────────▼───┐  ┌─────▼─────┐  ┌────────▼──────────┐
│  provider adapters     │  │  Kalshi   │  │  evidence APIs    │
│  anthropic │ openai │  │  │  (read)   │  │  search / odds    │
│  google                │  └───────────┘  └───────────────────┘
└────────────────────────┘
        │
┌───────▼────────────────────────────────────────────────────────┐
│                     Postgres (system of record)                │
│  decisions · agents · elicitations · models · pressure_tests   │
│  invalidations · structures · ledger · audits · calibration    │
│  append-only event log with hash chain                         │
└────────────────────────────────────────────────────────────────┘
```

Single deployable binary (`pdp-server`) embedding the static UI assets. The monitor daemon runs as a tokio task inside the same process; no separate workers in v1.

---

## 2. Crate layout (Cargo workspace)

```
pdp/
├── crates/
│   ├── pdp-core        # domain types, state machine, protocol math (no I/O)
│   ├── pdp-providers   # LLM provider adapters behind one trait
│   ├── pdp-evidence    # Kalshi client, search adapters, odds normalization
│   ├── pdp-store       # sqlx Postgres layer, event log, migrations
│   ├── pdp-judge       # elicitation validator, flip classifier, audit scorer
│   └── pdp-server      # axum API, SSE, gate endpoints, embedded UI, monitor
├── ui/                 # Web Components source (esbuild → embedded assets)
└── migrations/
```

**Dependency rule:** `pdp-core` has zero I/O dependencies — protocol math (ACS, Flip Distance, Kelly, matrix) is pure functions over domain types, unit-testable without network or DB. Everything else depends on core; core depends on nothing internal.

---

## 3. pdp-core — domain model and protocol math

### 3.1 Pipeline state machine

One decision is a state machine; transitions are the protocol's gates. Illegal transitions are unrepresentable — gate checks live in the transition functions, not in handlers.

```rust
pub enum DecisionState {
    Draft,
    Eliciting,               // panel calls in flight
    G1Pending { acs: Acs },  // human reviews convergence
    StressTesting,           // self-critique → model → sensitivity → pressure
    G2Pending {              // human reviews thesis + invalidation
        flip_distance: FlipDistance,
        invalidation: InvalidationCondition,
    },
    ComposingStructure,      // matrix cell + sizing + ladder proposal
    G3Pending { proposal: CommitmentStructure },  // human confirms entries
    Locked {                 // frozen; monitor-only
        structure: CommitmentStructure,
        invalidation: InvalidationCondition,
    },
    InvalidationTriggered { exit: PreCommittedExit },
    Settled { outcome: Outcome },
    Audited { process_score: ProcessScore },
    Killed { at_gate: Gate, reason: String },
}
```

Key invariants enforced by construction:

- `Locked` exposes no method that mutates `structure`. The only transitions out are `InvalidationTriggered` (world-state event from the monitor) and `Settled`.
- Weights are set in a `FrozenWeights` type created only via `Weights::freeze(human_actor_id)` — provider adapters cannot construct it. §3.3 weight custody is a type-system property.
- `PressureFlip` classification must be recorded before `G2Pending` can transition forward.

### 3.2 Protocol math (pure)

```rust
// Two-axis ACS (PDP §2.3)
pub fn acs(agents: &[AgentVerdict]) -> Acs;
// weight per agreeing agent: 1.0 both axes independent, 0.5 one, 0.2 none

// Flip Distance (PDP §3.4): min single-weight perturbation reversing top-2
pub fn flip_distance(model: &ScoredModel) -> FlipDistance;

// Decision matrix (PDP §4.1)
pub fn matrix_cell(acs: Acs, fd: FlipDistance) -> SizingBand;  // Full|Half|Quarter|Kill

// Fractional Kelly with ruin cap (PDP §4.2)
pub fn committed_fraction(p: Prob, b: NetOdds, k: KellyMult, m: SizingBand,
                          ruin_cap: Money, bankroll: Money) -> Money;

// Ladder allocation (PDP §4.3): terminal 40–60, checkpoints 40–60, events ≤10
pub fn allocate_ladder(total: Money, legs: &[LegSpec]) -> Result<Ladder, AllocError>;
```

All functions total over their input types; allocation violations (single-event legs over 10%) are `AllocError`, not clamps — the composer surfaces them to the human rather than silently fixing them.

### 3.3 Hash identity

Every persisted protocol artifact (elicitation, frozen weights, scored model, invalidation condition, commitment structure) carries a content hash computed by a single canonical serializer: explicit field order, no map iteration, no `Timestamp::now()` inside hashed content. Hashes chain in the event log (§6.3) so the decision record is tamper-evident — the audit reads a decision whose history provably wasn't rewritten after settlement.

---

## 4. pdp-providers — the panel engine

### 4.1 Provider trait

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    fn vendor(&self) -> Vendor;                    // Anthropic | OpenAI | Google
    async fn elicit(&self, req: ElicitationRequest) -> Result<RawResponse, ProviderError>;
    fn capabilities(&self) -> Capabilities;        // tool use, search, max context
}
```

Adapters: `anthropic` (Messages API), `openai` (Responses API), `google` (Gemini API). Each maps one canonical `ElicitationRequest` (system prompt from the PDP §7 library + evidence-class restriction + required-fields schema) to the vendor's wire format and back to one canonical `RawResponse`. Vendor quirks stay inside the adapter; core never sees them.

### 4.2 Evidence-class enforcement

Restriction is enforced by tool allowlist, not by prompt hope:

| Agent role | Tools granted |
|---|---|
| Quant | none (or a stats-dataset tool if configured) |
| Market | `kalshi_prices`, `odds_lookup` only |
| Structural | `web_search` with betting/odds domains excluded |
| Contrarian | full toolset + adversarial mandate prompt |

Tool calls are executed server-side by `pdp-evidence`; the provider adapters expose them as vendor-native tool/function definitions. An agent physically cannot fetch outside its class. Every tool invocation is logged per agent — this is the raw data the ACS axis-2 weight is computed from, replacing self-declared evidence sources with observed ones.

### 4.3 Panel fan-out

Panel run = `N vendors × M roles` elicitations, launched concurrently (`tokio::JoinSet`), each in an isolated context (no shared conversation state — decorrelation by construction). Per-call: timeout, bounded retries on transient errors (typed `Retryable` vs `Fatal`), response validated by `pdp-judge` against the 5 required fields. A hedge triggers exactly one structured re-prompt (F2); second failure marks the agent `Dropped { decision_id }` and the panel proceeds if ≥3 agents remain, else the run fails to `G1Pending` with a degraded-panel flag.

Cost note: full 3×4 panel ≈ 12 elicitations + tool calls; config supports per-decision panel shape (`panel = "3x4" | "3x1" | "1x4"`) so cheap decisions can run cheap panels.

---

## 5. pdp-judge — validation, classification, audit

Three judge functions, each a single LLM call with structured output, each running on a **different vendor than the agent being judged** (a Claude flip is classified by GPT or Gemini — no self-grading):

1. **Elicitation validator** — checks the 5 required fields, extracts pick/probability/load-bearing facts into typed columns
2. **Flip classifier** — input: pre-pressure response, post-pressure response, the §3.5 classification table. Output: `Robust | EvidenceFlip { new_evidence } | PressureFlip { reweighted_dims } | Collapse`. For `EvidenceFlip`, the claimed new evidence is checked against the agent's tool-call log — evidence the agent never actually retrieved reclassifies the flip as `PressureFlip` (the case's Williams-datum-as-cover pattern, §3.5, detected mechanically)
3. **Audit scorer** — outcome-redacted decision log in, 8-criterion process score out (PDP §5.3)

Judge outputs are schema-validated (`serde` + JSON schema in the request); malformed judge output is a hard error and re-run, never a silent default — no-fallback discipline on the critical path.

---

## 6. pdp-store — Postgres schema

### 6.1 Core tables

```sql
decisions        (id, question, domain, state, created_by, created_at)
agents           (id, decision_id, vendor, role, model_string, status)
elicitations     (id, agent_id, round, raw_response jsonb, pick, stated_p,
                  alternatives jsonb, load_bearing_facts jsonb,
                  tool_calls jsonb, content_hash, created_at)
frozen_weights   (id, decision_id, weights jsonb, justifications jsonb,
                  frozen_by, frozen_at, content_hash)
scored_models    (id, decision_id, weights_id, scores jsonb,
                  flip_distance numeric, content_hash)
pressure_tests   (id, agent_id, pre_elicitation_id, post_response jsonb,
                  classification, classified_by_vendor, evidence_check jsonb)
invalidations    (id, decision_id, condition_text, observable_inputs jsonb,
                  exit_action, status)          -- armed | triggered | expired
structures       (id, decision_id, legs jsonb, total_committed numeric,
                  matrix_cell, kelly_inputs jsonb, confirmed_by, frozen_at,
                  content_hash)
fills            (id, structure_id, leg_ref, external_id, price, qty,
                  reconciled boolean, discrepancy text)
ledger           (id, decision_id, event_at, checkpoint_ref, expected,
                  actual, thesis_impact)         -- none|supports|damages|invalidates
audits           (id, decision_id, criterion_scores jsonb, total smallint,
                  outcome_redacted boolean, scored_by_vendor)
calibration      (id, agent_ref, vendor, evidence_class, pick, stated_p,
                  outcome, pressure_response, surfaced_invalidation boolean,
                  facts_verified boolean, decision_id)
```

Operationally queried fields are typed columns; `jsonb` is reserved for snapshots and raw responses (audit blobs), versioned via `content_hash`.

### 6.2 Constraints enforcing the protocol

```sql
-- one frozen weight set per decision, ever
CREATE UNIQUE INDEX one_weights_per_decision ON frozen_weights (decision_id);

-- structures immutable after freeze: no UPDATE grants; corrections are new
-- rows with superseded_by, and only before state = 'locked'

-- ledger append-only: no UPDATE/DELETE grants to the app role

-- state transitions validated by trigger against the legal-transition table
-- (belt-and-suspenders under the type-level state machine)
```

Idempotency: pipeline steps (panel fan-out, judge calls, monitor polls) are keyed by `(decision_id, step, round)` with `ON CONFLICT DO NOTHING` on their result rows — a crashed/retried step never double-writes.

### 6.3 Event log

```sql
events (seq bigserial, decision_id, event_type, payload jsonb,
        payload_hash, prev_hash, created_at)
```

Every state transition and artifact creation appends an event; `prev_hash` chains per decision. The audit and calibration views are derived from typed tables, but the chain is the proof the record wasn't retro-edited — directly serving invariant 7 (outcome-blind audit) against the subtlest failure mode: quietly revising the thesis after settlement.

---

## 7. pdp-server — API and gates

### 7.1 Endpoints

```
POST   /decisions                       create draft
POST   /decisions/:id/elicit            launch panel (async; SSE progress)
GET    /decisions/:id/events            SSE stream: pipeline state + artifacts
POST   /decisions/:id/gates/g1          human verdict: proceed | kill
PUT    /decisions/:id/weights           human sets + freezes weights (only writer)
POST   /decisions/:id/stress            run self-critique → score → sensitivity → pressure
POST   /decisions/:id/gates/g2          proceed | kill (requires invalidation on file)
POST   /decisions/:id/compose           matrix + sizing + ladder proposal
POST   /decisions/:id/gates/g3          confirm structure → Locked
POST   /decisions/:id/fills             record external fills; reconcile (F6)
POST   /decisions/:id/settle            record outcome → trigger audit
GET    /calibration                     cross-decision agent/vendor calibration
```

Gate endpoints require an authenticated human actor (single-user v1: static token + actor id; deny-by-default middleware on every route). No endpoint mutates a `Locked` structure; `POST /fills` records external reality, it does not place orders.

### 7.2 Monitor daemon

Tokio task, per `Locked` decision:

- Polls Kalshi settlement status for each leg (bounded interval + jitter; timeouts and size caps on all external I/O)
- Polls the invalidation condition's observable inputs (configured per decision: e.g. a squad-news search query whose judge-parsed result maps to condition variables)
- Writes `ledger` rows; on `thesis_impact = invalidates`, transitions the decision to `InvalidationTriggered` and alerts (webhook/ntfy) — it never executes an exit; the pre-committed exit is presented for one-click human confirmation
- F10 detection: ≥3 `damages` rows without invalidation → flags the invalidation design for audit

### 7.3 Provider/config hygiene

API keys env-only, validated at boot alongside all config (fail fast, explicit defaults); structured logs (`tracing`) carry `decision_id` + `content_hash` on every span; prompts and raw responses logged truncated with full versions only in the DB; metrics (Prometheus) cover per-vendor error rates, latency, retries, judge disagreement rate, and gate dwell times.

---

## 8. ui/ — Web Components

No framework. Native custom elements + `<template>`, esbuild bundle, embedded in the binary via `include_dir!`. State flows in via the SSE stream; commands are plain `fetch` to the gate endpoints.

```
<pdp-app>                      router/shell, SSE subscription
├── <pdp-decision-list>        open + settled decisions
├── <pdp-pipeline>             state-machine visual; gates as actionable stops
│   ├── <pdp-panel-board>      per-agent cards: vendor × role grid, picks,
│   │                          stated p, tool-call counts, ACS gauge
│   ├── <pdp-weight-editor>    the ONLY weight input in the system;
│   │                          freeze action; justification required per weight
│   ├── <pdp-model-view>       scored matrix, per-dimension bars,
│   │                          Flip Distance meter with the flip-axis highlighted
│   ├── <pdp-pressure-view>    pre/post responses side-by-side,
│   │                          judge classification badge, evidence-check result
│   ├── <pdp-invalidation>     condition editor with the 4-test checklist
│   ├── <pdp-ladder>           proposed legs, allocation bands, sizing math
│   │                          shown (Kelly inputs → cap), G3 confirm
│   └── <pdp-ledger>           checkpoint timeline; invalidation status lamp
└── <pdp-calibration>          cross-decision vendor/agent table:
                               stated p vs hit rate, pressure-response history
```

Components carry stable identity attributes (`decision-id`, `agent-id`, `leg-ref`) for testability. Each component owns its shadow DOM and renders from a single `state` property set by the app shell — one canonical state shape, no per-component fetching.

---

## 9. Security posture

- **No order execution.** v1 integrates Kalshi read-only. The confirm-to-execute step is the human on the exchange's own UI; `POST /fills` reconciles after the fact. This is a protocol decision (§4.4 human gate), not a missing feature.
- Deny-by-default authn on all routes; single-actor v1.
- External I/O: allowlisted hosts (provider APIs, Kalshi, configured search), private-IP deny, timeouts, response size caps, schema validation of every vendor response before critical use (malformed Kalshi settlement data fails the poll, never fakes a ledger row).
- Judge cross-vendor rule (§5) doubles as prompt-injection containment: content fetched by one agent's tools never becomes instructions to the judge grading it — judges receive it as quoted data in a fixed schema.
- Secrets never in repo, logs, or DB; `sqlx` compile-time-checked queries; migrations expand/contract.

## 10. Build order

1. **`pdp-core`** — types, state machine, protocol math + property tests (allocation bounds, matrix totality, Kelly caps). Pure, fast, the contract everything else compiles against
2. **`pdp-judge` flip classifier** — the novel component; standalone binary first, testable against this conversation's actual pre/post-pressure transcripts as fixtures
3. **`pdp-store` + migrations** — schema, event chain, idempotent step keys
4. **`pdp-providers`** — Anthropic adapter first, then OpenAI, Google; evidence-class tool allowlists
5. **`pdp-server`** — API, gates, SSE; monitor daemon
6. **`ui/`** — pipeline + weight editor + pressure view first (the human-gate surfaces), calibration view last
7. **`pdp-evidence` Kalshi client** — markets, settlements, positions (read)

Milestone 1 = steps 1–2: the math and the sycophancy detector, runnable against recorded fixtures before any network or DB exists.

---

## 11. Failure domains and failover matrix

Failure handling follows one rule inherited from the protocol: **degrade toward the human gate, never around it.** Every failover lands the pipeline in a state where either the decision waits for a human or the frozen structure holds untouched. No failure path auto-commits, auto-exits, or auto-relaxes a gate.

### 11.1 Failure domain map

```
Domain A: LLM providers        (outage, rate limit, model deprecation, schema drift)
Domain B: Evidence sources     (Kalshi API, search APIs — outage, schema change, stale data)
Domain C: Judge layer          (misclassification, malformed output, vendor unavailability)
Domain D: Store                (Postgres unavailability, migration failure, chain break)
Domain E: Pipeline process     (crash mid-step, duplicate execution, poisoned retry)
Domain F: Monitor daemon       (missed poll, false invalidation signal, alert failure)
Domain G: Human layer          (gate abandonment, fat-finger at G3, actor unavailable)
```

### 11.2 Failover matrix

| # | Domain | Failure | Detection | Failover | Protocol mapping |
|---|---|---|---|---|---|
| A1 | Provider | Vendor outage during panel fan-out | Request timeout / 5xx after bounded retries | Panel proceeds if ≥3 agents across ≥2 vendors remain; else park in `Eliciting` with `degraded_panel` flag and alert. Never substitute a same-vendor duplicate to fake the axis | PDP F9; ACS computed on actual panel, not intended |
| A2 | Provider | Rate limit mid-panel | 429 + retry-after | Token-bucket per vendor; queue and resume. Panel round is idempotent-keyed, so resumed calls never double-write | §6.2 idempotency |
| A3 | Provider | Model deprecated / model string invalid | 4xx at call time | Boot-time model validation per vendor; runtime failure pins the agent `Dropped`, logs `model_string` mismatch, alerts. Config change required — no silent auto-upgrade to a newer model (calibration history is per model string) | §5.4 calibration integrity |
| A4 | Provider | Response schema drift (vendor changes wire format) | Adapter deserialization failure | Typed error, agent `Dropped`, raw payload preserved in `elicitations.raw_response` for adapter fix. Never regex-salvage a partial parse on the critical path | No-fallback discipline |
| B1 | Kalshi | API outage during monitoring | Poll timeout | Exponential backoff with cap; `ledger` gains `poll_gap` rows so audit sees the blind window. Structure holds (lock is default-safe under blindness) | §7.2 |
| B2 | Kalshi | Settlement schema change | Schema validation failure on poll | Poll fails closed: no ledger row written, alert raised. A malformed settlement can never fabricate `thesis_impact` | §9 schema-validate-before-critical-use |
| B3 | Search | Evidence API returns empty/garbage during invalidation watch | Judge-parse confidence below threshold | Condition variables marked `stale`, staleness age shown in UI lamp; invalidation can still be triggered manually by the human on outside knowledge | §3.6 asymmetric-to-pressure preserved: staleness never auto-triggers |
| C1 | Judge | Judge vendor unavailable | Call failure | Fall over to the second non-self vendor (Claude flip → GPT judge → Gemini judge). Both down: classification `Pending`, pipeline parks before G2 — a flip is never classified by its own vendor and never defaulted | §5 cross-vendor rule |
| C2 | Judge | Malformed judge output | JSON-schema validation | One re-run with error appended; second failure parks the step `Pending` + alert. Never coerce or default a classification | §5 |
| C3 | Judge | Suspected misclassification (human disagrees) | Human override at G2 review | Override recorded as its own event with actor + rationale; original classification retained (never overwritten). Both feed judge-eval set (§13.2) | Event log §6.3 |
| D1 | Store | Postgres unavailable | Connection failure | Process refuses new work; in-flight LLM responses buffered to local WAL spill file, replayed on reconnect under idempotency keys. Monitor suspends (safe: lock holds) | §6.2 |
| D2 | Store | Migration failure on deploy | Migration tool exit | Expand/contract discipline: failed expand aborts deploy, old binary keeps running against old schema. Contract migrations run only after a full release cycle | §9 |
| D3 | Store | Event-chain hash mismatch on read | Chain verification job | Decision flagged `chain_broken`, frozen read-only, alert. Audit for that decision reports the break instead of trusting the record | §6.3 tamper evidence |
| E1 | Pipeline | Process crash mid-step | Restart + step-state scan | Steps resume from `(decision_id, step, round)` keys; completed steps skip, partial steps re-run cleanly. Crash between judge call and write re-runs the judge (results row conflict-guarded) | §6.2 |
| E2 | Pipeline | Poisoned retry (same input fails deterministically) | Retry budget exhausted with identical error hash | Step parks `Pending` with the error surfaced; retry budget per step, typed `Fatal` short-circuits immediately. No catch-all-continue anywhere in Phase 1–3 | Typed-error discipline |
| F1 | Monitor | Missed poll window (daemon stall) | Heartbeat gap metric | Watchdog restarts the tokio task; ledger `poll_gap` recorded. Missed settlement is caught on next successful poll — settlements are facts, not events that expire | §7.2 |
| F2 | Monitor | False invalidation signal (judge misparse of news) | Human review on the one-click confirm | The daemon only *proposes* `InvalidationTriggered`; confirmation is human. False positive costs one notification, never an exit | §7.2 never-executes rule |
| F3 | Monitor | Alert channel down | Alert send failure | Secondary channel (webhook → email fallback); both down: UI banner + red lamp on next load. Alerts are at-least-once with dedupe keys | — |
| G1 | Human | Gate abandoned (decision parked at G1/G2/G3 for N days) | Gate dwell-time metric | Auto-expire to `Killed { at_gate, reason: "gate timeout" }` after configurable TTL (default 14d). A stale thesis is worse than no thesis — evidence rots | Entry-at-ignorance principle §4.3 |
| G2 | Human | Fat-finger at G3 confirm | Post-confirm diff screen | G3 is two-step: confirm proposal → review rendered order sheet → final confirm. Mismatch between fills and structure caught by `POST /fills` reconciliation (PDP F6) | §4.4 |
| G3 | Human | Actor unavailable during a live invalidation trigger | Unconfirmed trigger age | Trigger stays armed and alerting; structure holds. v1 is single-actor by design — no delegation path, because a second actor without the thesis context is a worse failure than a held position | §4.5 lock |

### 11.3 What never fails over

Three behaviors have no fallback by design; their failure mode is *stop*:

1. **Weight custody.** If the human weight-freeze step can't complete, there is no model scoring. No agent-proposed default weights, ever.
2. **Flip classification.** Unclassified pressure responses park the pipeline. An unclassified flip treated as `Robust` is the exact failure the system exists to prevent.
3. **Order execution.** Absent from the system. There is no code path whose failure could place or cancel an order.

---

## 12. Degraded-mode ladder

Explicit service levels, each safe, each visible in the UI shell as a mode banner:

| Mode | Trigger | Available | Unavailable |
|---|---|---|---|
| **Full** | All domains green | Everything | — |
| **Panel-degraded** | ≥1 vendor down, ≥3 agents / ≥2 vendors remain | Full pipeline; ACS reflects the real panel | The missing vendor's calibration continuity |
| **Judge-degraded** | Only one non-self judge vendor reachable | Pipeline continues on single cross-vendor judge; disagreement metric suspended | Judge redundancy |
| **Evidence-blind** | Kalshi and/or search down | New decisions through G2 (elicitation may lack Market-agent tools → that agent parks); locked decisions hold | Live ledger, invalidation auto-watch (manual trigger still available) |
| **Store-degraded** | Postgres down | Nothing new; WAL spill buffers in-flight responses | All reads/writes; monitor suspended (safe) |
| **Read-only** | Chain break, migration hold, or manual flip | UI, history, calibration views | All state transitions |

Mode transitions are events in the log — an audit can see that a decision passed G2 in Judge-degraded mode and weigh that.

---

## 13. Evaluation framework

Three eval targets, in descending order of novelty risk: the judges (LLM components with correctness stakes), the panel (elicitation quality), and the protocol math (pure but property-rich). Evals are a workspace crate (`pdp-evals`) run in CI on golden sets and nightly against live providers.

### 13.1 Eval infrastructure

```
pdp-evals/
├── fixtures/
│   ├── flips/            # labeled pre/post-pressure transcript pairs
│   ├── elicitations/     # labeled valid/hedge/malformed responses
│   ├── invalidations/    # condition texts labeled against the 4 tests
│   └── audits/           # decision logs with reference criterion scores
├── golden/               # frozen expected outputs per fixture (content-hashed)
└── runners/              # CI runner (fixtures only) · nightly (live vendors)
```

Fixture provenance rule: every fixture is a real or minimally-edited transcript, never synthetic-only. Seed set: this project's source conversation — the Spain/France pressure flip (labeled `PressureFlip`, with the Williams datum as the evidence-cover trap), the retraction turn (labeled `Robust` re-derivation), and the initial hedged answer + re-prompt pair (elicitation fixtures). Cross-vendor transcripts from the same question sequence (Gemini, GPT) join the set as they're recovered.

### 13.2 Judge evals

**Flip classifier** (the highest-stakes judge):

| Metric | Definition | Gate |
|---|---|---|
| Classification accuracy | Agreement with human labels on the fixture set, per class | ≥ 0.9 overall; **zero tolerance for `PressureFlip → Robust` confusions** (that error re-admits sycophancy); `Robust → PressureFlip` is tolerable (costs a re-check, not a corrupted thesis) |
| Evidence-cover detection | Flips wrapping one real datum around a reweighting, correctly caught via tool-log cross-check | 100% on fixture set — this check is mechanical (claimed evidence ∉ tool log ⇒ reclassify) and any miss is a code bug, not a model limitation |
| Cross-vendor stability | Same fixture judged by each vendor pair; disagreement rate | ≤ 0.1; sustained disagreement on a fixture promotes it to human adjudication and the golden set grows |
| Drift | Nightly live-run accuracy vs golden | Alert on 2-night regression; judge model strings are pinned and upgraded only through an eval pass |

Every human override at G2 (failover C3) automatically becomes a new labeled fixture — the eval set compounds from production disagreements, which is the same pattern as the calibration log.

**Elicitation validator:** precision/recall on hedge detection (a hedge passed through pollutes ACS with a non-verdict; a valid answer bounced wastes a re-prompt — tune toward recall on hedges). Field-extraction exactness on pick and stated_p: 100%, these are typed columns feeding math.

**Audit scorer:** mean absolute error ≤ 1 point (of 16) against reference-scored logs; **outcome-blindness probe** — score the same log with outcome included vs redacted; any systematic score shift on winning vs losing outcomes fails the judge (it's exhibiting the outcome bias it exists to audit).

### 13.3 Panel evals

Run per provider adapter, nightly:

| Eval | Method | Signal |
|---|---|---|
| Elicitation compliance | Standard question battery → validator pass rate on first attempt | Adapter prompt quality per vendor; re-prompt rate trend |
| Evidence-class containment | Agents given restricted toolsets + questions that tempt out-of-class reasoning ("what do the odds say?" to the Quant agent) | Must answer from class or state inability — leakage means the restriction prompt or allowlist has a hole |
| Pressure robustness battery | Scripted pressure sequence against each vendor on questions with known-stable answers | Per-vendor sycophancy baseline for the calibration log's prior — the §3.5 cross-vendor comparison, made systematic instead of anecdotal |
| Probability discipline | Same question, 5 runs per vendor | Stated-p variance; a vendor whose p swings ±15 points across identical runs gets its confidence down-weighted in panel aggregation |

### 13.4 Protocol math evals (property tests, CI-blocking)

- **ACS:** bounded [0,1]; monotone in agreement; permutation-invariant; unanimous same-vendor-same-evidence panel of any size scores exactly the single-agent value (the "one analyst, N hats" property, tested directly)
- **Flip Distance:** reported perturbation actually reverses the ranking when applied (verified by re-scoring); no smaller single-weight perturbation does (swept)
- **Kelly + cap:** committed amount ≤ ruin cap for all inputs including pathological (p=1, b→∞); f\* ≤ 0 always yields zero commitment, never a short
- **Ladder allocation:** band constraints hold or `AllocError`; allocations sum to total exactly (integer-cent arithmetic, no float drift)
- **State machine:** exhaustive transition-table test — every `(state, event)` pair either legal or a typed rejection; fuzz sequences never reach an inconsistent state; `Locked` structure hash identical before/after any event sequence not containing settle/invalidate
- **Hash chain:** any single-byte mutation in any historical payload is detected by verification (fuzzed)

### 13.5 End-to-end protocol eval (the honest one)

The system's real claim — decisions run through PDP outperform unstructured ones — is only testable slowly. Structural support:

- Every decision stores its counterfactual: the naive pick (highest single-agent confidence, no ACS discount, no stress test) alongside the protocol pick
- The calibration view reports both tracks' Brier scores as decisions settle
- Honest floor: at ~1 significant decision/month this is directional after a year, statistically meaningful after several. The framework exists so the comparison is *recorded from day one*, not reconstructed — the same reason the case's trade history was decisive evidence: it was written down contemporaneously

---

## 14. Testing strategy (beyond evals)

| Layer | Approach |
|---|---|
| pdp-core | Property tests (§13.4) + snapshot tests on canonical serialization (hash stability across releases is release-blocking: a serializer change that shifts content hashes breaks chain verification for existing decisions) |
| pdp-providers | Adapter contract tests against recorded wire fixtures per vendor; nightly live smoke (1 cheap call/vendor) catches silent API drift |
| pdp-store | sqlx compile-time query checks; migration round-trip test (expand → old binary → new binary → contract) in CI against ephemeral Postgres; grant tests asserting the app role *cannot* UPDATE ledger/events (the append-only property tested as a property, not assumed from the migration) |
| pdp-server | Gate authz tests (every route × unauthenticated actor = deny); SSE resume tests; full-pipeline integration test with mocked providers driving a decision Draft → Audited |
| Monitor | Simulated-clock tests for poll/backoff/watchdog; false-signal test asserting a garbage settlement payload produces zero ledger rows |
| UI | Component-level DOM tests on the three gate surfaces (weight editor, pressure view, G3 confirm) — the fat-finger defenses get the test budget |

---

## 15. Operational runbook triggers

Conditions that page (single-operator system — paging is a phone notification, but the discipline holds):

1. `InvalidationTriggered` unconfirmed > 4h — the one genuinely time-sensitive state
2. Chain verification failure (D3) — data integrity, investigate before any further writes
3. Judge classification `Pending` > 24h on a decision pre-G2 — pipeline is parked and forgotten
4. Nightly eval regression on the flip classifier — the sycophancy detector degrading is a silent-failure risk to every future decision
5. Gate dwell > 10d (warning at 7) — ahead of the 14d auto-kill so the kill is chosen, not discovered

Everything else is a dashboard concern, not a page.

---

## 16. Cost model and budget enforcement

### 16.1 Per-decision cost anatomy

A full-protocol decision at 2026 API pricing (order-of-magnitude, config-driven, not hardcoded):

| Stage | Calls | Model tier | Est. tokens | Est. cost |
|---|---|---|---|---|
| Panel elicitation (3×4) | 12 + tool calls | Frontier per vendor | ~15k in / 2k out each | $3–8 |
| Hedge re-prompts | 0–4 | Frontier | small | <$1 |
| Self-critique | 3–12 | Frontier | ~5k each | $1–3 |
| Model scoring | 3 | Mid-tier suffices (structured task) | ~3k each | <$1 |
| Sensitivity sweep | 0 | Pure math, no API | — | $0 |
| Pressure test | 3–12 | Frontier | ~4k each | $1–3 |
| Flip classification | 1 per pressured agent | **Frontier, pinned** — the zero-tolerance eval gate (§13.2) forbids economizing here | ~8k each | $1–4 |
| Elicitation validation | 1 per response | Cheap tier | ~2k each | <$0.50 |
| Invalidation-watch (per day locked) | 1–4 polls × judge parse | Cheap tier | small | $0.10–0.50/day |
| Audit + calibration | 2 | Mid-tier | ~10k | <$1 |
| **Full decision (3×4 panel, 30-day lock)** | | | | **~$10–25** |
| Cheap panel (3×1 or 1×4) | | | | ~$4–8 |
| Nightly evals | fixtures + 1 live smoke/vendor | Mixed | | ~$1–3/night |

The notable asymmetry: the flip classifier is the one stage where the cheap tier is banned by eval policy, and the sensitivity sweep — one of the highest-value steps — is free. Cost concentrates exactly where the novel value is.

### 16.2 Budget enforcement

```toml
[budget]
per_decision_usd   = 30        # hard: pipeline parks Pending on breach, alert
monthly_usd        = 150       # hard: no new panels; locked decisions' monitor
                               # continues (safety spend is never cut first)
eval_monthly_usd   = 50        # nightly evals degrade to weekly on breach

[tiers]
flip_classifier    = "frontier-pinned"   # change requires eval pass (§13.2)
panel              = "frontier"
scoring            = "mid"
validator          = "cheap"
monitor_judge      = "cheap"
```

- Every provider call records `(decision_id, stage, vendor, model, tokens_in, tokens_out, usd)` to a `costs` table; the decision view shows cumulative spend live
- Budget breach mid-panel parks the run resumable (idempotency keys, §6.2) — never a half-priced panel silently passed off as a full one; the ACS is computed on completed elicitations only, with the degraded-panel flag
- Monitor spend is exempt from the monthly cap: a locked position's invalidation watch is safety infrastructure, and cutting it to save $3 inverts the protocol's priorities
- The audit gains a cost line: `usd_per_decision` trends in the calibration view, so protocol overhead is a measured quantity, not a vibe

### 16.3 Cost-tier eval guard

Any tier downgrade (e.g. moving scoring from mid to cheap) requires a fixture-set eval pass for the affected judge/stage before the config change deploys — the same discipline as model-string upgrades (§13.2 drift gate). Saving $0.50 per decision by quietly degrading the validator's hedge-recall is the kind of loss that never shows up until the calibration log is polluted.
