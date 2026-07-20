-- PDP Engine — Postgres schema v0.3
-- System of record for the Polymorphic Decision Protocol (PDP v1.3).
-- Matches pdp-architecture.md §6, §16, and PDP §4A (portfolio governance).
-- Conventions: typed columns for operationally queried fields; jsonb only for
-- snapshots and raw payloads (audit blobs); append-only tables enforced by
-- grants, not convention; every protocol artifact carries a content_hash from
-- the canonical serializer (explicit field order, no map iteration, no
-- timestamps inside hashed content).

BEGIN;

-- ---------------------------------------------------------------------------
-- Enums
-- ---------------------------------------------------------------------------

CREATE TYPE decision_state AS ENUM (
  'draft', 'eliciting', 'g1_pending', 'stress_testing', 'g2_pending',
  'composing_structure', 'g3_pending', 'locked', 'invalidation_triggered',
  'settled', 'audited', 'killed'
);

CREATE TYPE vendor AS ENUM ('anthropic', 'openai', 'google');

CREATE TYPE agent_role AS ENUM ('quant', 'market', 'structural', 'contrarian', 'unrestricted');

CREATE TYPE agent_status AS ENUM ('active', 'dropped', 'completed');

CREATE TYPE pressure_classification AS ENUM (
  'robust', 'evidence_flip', 'pressure_flip', 'collapse', 'pending'
);

CREATE TYPE thesis_impact AS ENUM ('none', 'supports', 'damages', 'invalidates');

CREATE TYPE invalidation_status AS ENUM ('armed', 'triggered', 'expired');

CREATE TYPE sizing_band AS ENUM ('full', 'half', 'quarter', 'kill');

-- ---------------------------------------------------------------------------
-- Core decision pipeline
-- ---------------------------------------------------------------------------

CREATE TABLE decisions (
  id            uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  question      text NOT NULL,
  domain        text NOT NULL,                    -- e.g. 'prediction-market', 'hiring'
  state         decision_state NOT NULL DEFAULT 'draft',
  created_by    text NOT NULL,                    -- human actor id
  created_at    timestamptz NOT NULL DEFAULT now(),
  killed_at_gate text,                            -- set iff state = 'killed'
  kill_reason   text
);

CREATE TABLE agents (
  id            uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id   uuid NOT NULL REFERENCES decisions(id),
  vendor        vendor NOT NULL,
  role          agent_role NOT NULL,
  model_string  text NOT NULL,                    -- pinned; calibration is per model string
  status        agent_status NOT NULL DEFAULT 'active',
  UNIQUE (decision_id, vendor, role)
);

CREATE TABLE elicitations (
  id                 uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  agent_id           uuid NOT NULL REFERENCES agents(id),
  round              smallint NOT NULL DEFAULT 1, -- 1 = initial, 2 = hedge re-prompt
  raw_response       jsonb NOT NULL,              -- full vendor payload, audit blob
  pick               text,                        -- NULL until validator extracts
  stated_p           numeric(5,4) CHECK (stated_p BETWEEN 0 AND 1),
  alternatives       jsonb,                       -- [{option, p}], validator-extracted
  load_bearing_facts jsonb,                       -- [text, text, text]
  tool_calls         jsonb NOT NULL DEFAULT '[]', -- observed, not self-declared;
                                                  -- feeds ACS axis-2 and evidence-cover check
  content_hash       text NOT NULL,
  created_at         timestamptz NOT NULL DEFAULT now(),
  UNIQUE (agent_id, round)                        -- idempotency key
);

-- Weight custody: constructible only via the human freeze endpoint.
-- ONE row per decision, ever (§3.3 of the protocol).
CREATE TABLE frozen_weights (
  id             uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id    uuid NOT NULL REFERENCES decisions(id),
  weights        jsonb NOT NULL,                  -- {dimension: weight}, sums to 1
  justifications jsonb NOT NULL,                  -- {dimension: one-line reason}
  frozen_by      text NOT NULL,                   -- human actor id, never a service id
  frozen_at      timestamptz NOT NULL DEFAULT now(),
  content_hash   text NOT NULL
);
CREATE UNIQUE INDEX one_weights_per_decision ON frozen_weights (decision_id);

CREATE TABLE scored_models (
  id            uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id   uuid NOT NULL REFERENCES decisions(id),
  weights_id    uuid NOT NULL REFERENCES frozen_weights(id),
  scores        jsonb NOT NULL,                   -- {option: {dimension: 0..10}}
  totals        jsonb NOT NULL,                   -- {option: weighted total}
  flip_distance numeric(6,4),                     -- min single-weight perturbation
  flip_dimension text,                            -- which dial it sits on
  acs           numeric(5,4) CHECK (acs BETWEEN 0 AND 1),
  content_hash  text NOT NULL,
  created_at    timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE pressure_tests (
  id                    uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  agent_id              uuid NOT NULL REFERENCES agents(id),
  pre_elicitation_id    uuid NOT NULL REFERENCES elicitations(id),
  pressure_prompt       text NOT NULL,            -- verbatim; canonically "I think you're wrong."
  post_response         jsonb NOT NULL,
  classification        pressure_classification NOT NULL DEFAULT 'pending',
  classified_by_vendor  vendor,                   -- MUST differ from agents.vendor (no self-grading)
  evidence_check        jsonb,                    -- claimed evidence vs tool_calls log;
                                                  -- mismatch reclassifies to pressure_flip
  human_override        jsonb,                    -- {actor, rationale}; original kept
  created_at            timestamptz NOT NULL DEFAULT now(),
  UNIQUE (agent_id, pre_elicitation_id)
);

CREATE TABLE invalidations (
  id                 uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id        uuid NOT NULL REFERENCES decisions(id),
  condition_text     text NOT NULL,               -- IF <observable> THEN <exit>
  observable_inputs  jsonb NOT NULL,              -- [{name, source, query}] for monitor;
                                                  -- shared-observable dedupe across the book (§4A)
  exit_action        text NOT NULL,               -- pre-committed
  status             invalidation_status NOT NULL DEFAULT 'armed',
  triggered_at       timestamptz,
  content_hash       text NOT NULL
);

-- ---------------------------------------------------------------------------
-- Commitment and portfolio (PDP §4, §4A)
-- ---------------------------------------------------------------------------

-- Correlation groups: decisions sharing terminal outcomes or invalidation
-- observables count as one exposure against the book cap.
CREATE TABLE decision_groups (
  id           uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  reason       text NOT NULL,                     -- 'shared_terminal' | 'shared_observable' | 'same_domain'
  rho          numeric(3,2) NOT NULL CHECK (rho BETWEEN 0 AND 1),
  created_at   timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE decision_group_members (
  group_id     uuid NOT NULL REFERENCES decision_groups(id),
  decision_id  uuid NOT NULL REFERENCES decisions(id),
  PRIMARY KEY (group_id, decision_id)
);

CREATE TABLE structures (
  id              uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id     uuid NOT NULL REFERENCES decisions(id),
  legs            jsonb NOT NULL,                 -- [{ref, kind: terminal|checkpoint|single_event,
                                                  --   market_id, planned_qty, planned_price, alloc_pct}]
  total_committed numeric(14,2) NOT NULL,
  matrix_cell     sizing_band NOT NULL,
  kelly_inputs    jsonb NOT NULL,                 -- {p, b, k, m, ruin_cap, bankroll}
  confirmed_by    text NOT NULL,                  -- human actor, G3
  frozen_at       timestamptz NOT NULL,
  superseded_by   uuid REFERENCES structures(id), -- corrections pre-lock only; no UPDATEs
  content_hash    text NOT NULL
);
CREATE UNIQUE INDEX one_active_structure_per_decision
  ON structures (decision_id) WHERE superseded_by IS NULL;

CREATE TABLE fills (
  id            uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  structure_id  uuid NOT NULL REFERENCES structures(id),
  leg_ref       text NOT NULL,
  external_id   text,                             -- exchange fill/order id
  price         numeric(10,4) NOT NULL,
  qty           numeric(14,4) NOT NULL,
  filled_at     timestamptz NOT NULL,
  reconciled    boolean NOT NULL DEFAULT false,   -- F6: fills vs planned structure
  discrepancy   text                              -- e.g. 'unchosen_exposure: misclick;
                                                  --  excluded from thesis accounting'
);

-- ---------------------------------------------------------------------------
-- Monitoring, settlement, audit (PDP §5)
-- ---------------------------------------------------------------------------

-- Append-only: app role has INSERT + SELECT only (see grants).
CREATE TABLE ledger (
  id             uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id    uuid NOT NULL REFERENCES decisions(id),
  event_at       timestamptz NOT NULL,
  checkpoint_ref text,
  expected       text,
  actual         text,
  thesis_impact  thesis_impact NOT NULL,
  poll_gap       boolean NOT NULL DEFAULT false   -- B1: monitor blind windows are visible
);

CREATE TABLE settlements (
  id                 uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id        uuid NOT NULL REFERENCES decisions(id) UNIQUE,
  outcome            text NOT NULL,               -- 'win' | 'loss' | per-leg detail in jsonb
  detail             jsonb NOT NULL,
  realized_amount    numeric(14,2),
  thesis_p           numeric(5,4) NOT NULL,
  luck_note          text NOT NULL,               -- the required verbatim sentence
  checkpoint_hits    smallint,
  checkpoint_total   smallint,
  naive_pick         text NOT NULL,               -- counterfactual, recorded at THESIS time
  naive_p            numeric(5,4) NOT NULL,
  protocol_brier     numeric(6,4),
  naive_brier        numeric(6,4),
  settled_at         timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE audits (
  id                uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id       uuid NOT NULL REFERENCES decisions(id) UNIQUE,
  criterion_scores  jsonb NOT NULL,               -- {criterion: 0|1|2}, 8 criteria
  total             smallint NOT NULL CHECK (total BETWEEN 0 AND 16),
  outcome_redacted  boolean NOT NULL,             -- must be true for a valid audit
  scored_by_vendor  vendor,
  created_at        timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE calibration (
  id                     uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id            uuid NOT NULL REFERENCES decisions(id),
  agent_id               uuid NOT NULL REFERENCES agents(id),
  vendor                 vendor NOT NULL,
  model_string           text NOT NULL,
  evidence_class         agent_role NOT NULL,
  pick                   text NOT NULL,
  stated_p               numeric(5,4) NOT NULL,
  outcome                text NOT NULL,
  pressure_response      pressure_classification NOT NULL,
  surfaced_invalidation  boolean NOT NULL,
  facts_verified         boolean NOT NULL,
  UNIQUE (decision_id, agent_id)
);

-- ---------------------------------------------------------------------------
-- Event chain (tamper evidence, §6.3) and costs (§16)
-- ---------------------------------------------------------------------------

-- Append-only, hash-chained per decision. prev_hash of the first event per
-- decision is the decision id's hash.
CREATE TABLE events (
  seq          bigserial PRIMARY KEY,
  decision_id  uuid NOT NULL REFERENCES decisions(id),
  event_type   text NOT NULL,                     -- state transitions + artifact creations
  payload      jsonb NOT NULL,
  payload_hash text NOT NULL,
  prev_hash    text NOT NULL,
  created_at   timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX events_by_decision ON events (decision_id, seq);

CREATE TABLE costs (
  id           uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  decision_id  uuid REFERENCES decisions(id),     -- NULL for eval/smoke spend
  stage        text NOT NULL,                     -- 'panel' | 'self_critique' | 'scoring' |
                                                  -- 'pressure' | 'flip_classifier' | 'validator' |
                                                  -- 'monitor' | 'audit' | 'eval'
  vendor       vendor NOT NULL,
  model_string text NOT NULL,
  tokens_in    integer NOT NULL,
  tokens_out   integer NOT NULL,
  usd          numeric(10,4) NOT NULL,
  created_at   timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX costs_by_decision ON costs (decision_id);
CREATE INDEX costs_by_month ON costs (date_trunc('month', created_at));

-- ---------------------------------------------------------------------------
-- Grants: append-only enforced as a property, not a convention.
-- The application connects as pdp_app. It cannot UPDATE or DELETE the
-- record-of-fact tables. Migrations run as a separate owner role.
-- ---------------------------------------------------------------------------

-- Example (role creation is deployment-specific):
--   REVOKE ALL ON ledger, events, fills, calibration, costs FROM pdp_app;
--   GRANT SELECT, INSERT ON ledger, events, fills, calibration, costs TO pdp_app;
--   GRANT SELECT, INSERT ON decisions, agents, elicitations, frozen_weights,
--     scored_models, pressure_tests, invalidations, structures, settlements,
--     audits, decision_groups, decision_group_members TO pdp_app;
--   GRANT UPDATE (state, killed_at_gate, kill_reason) ON decisions TO pdp_app;
--   GRANT UPDATE (status, triggered_at) ON invalidations TO pdp_app;
--   GRANT UPDATE (classification, classified_by_vendor, evidence_check,
--     human_override) ON pressure_tests TO pdp_app;
--   GRANT UPDATE (pick, stated_p, alternatives, load_bearing_facts)
--     ON elicitations TO pdp_app;   -- validator extraction only
--   GRANT UPDATE (reconciled, discrepancy) ON fills TO pdp_app;
--   GRANT UPDATE (status) ON agents TO pdp_app;
--   GRANT UPDATE (superseded_by) ON structures TO pdp_app;  -- pre-lock corrections
-- No UPDATE grant on frozen_weights under any circumstances.

COMMIT;
