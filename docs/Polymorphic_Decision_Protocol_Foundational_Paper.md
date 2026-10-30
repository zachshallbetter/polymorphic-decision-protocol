---
title: "Polymorphic Decision Protocol: Deterministic Multi-Agent Deliberation and Pressure-Testing Under Epistemic Uncertainty"
subtitle: "A Procedural Architecture for Provenance-Discounted Consensus and Calibrated Execution"
type: "Foundational research paper"
author:
  - name: "Shallbetter, Zachary"
    orcid: "https://orcid.org/0009-0009-9450-3429"
date: "2026-10-23"
version: "1.3.0"
scope: "Formal procedural protocol for high-stakes, high-uncertainty decision making under epistemic pressure using heterogeneous AI panels and human custody"
license: "CC-BY-4.0"
doi: "10.5281/zenodo.XXXXXXXX"
---

# Polymorphic Decision Protocol

*Deterministic Multi-Agent Deliberation and Pressure-Testing Under Epistemic Uncertainty*

+-----------------------------------------------------------------------+
| **EXECUTIVE DECISION**                                                |
|                                                                       |
| **Do not treat raw AI model agreement as probability or truth.**      |
|                                                                       |
| High-uncertainty decision systems must discount multi-model           |
| agreement by input provenance, apply adversarial pressure prompts     |
| to detect sycophancy, score formal models against human-held frozen   |
| weights, and benchmark against naive counterfactuals using Brier      |
| scores. The human holds absolute custody of weights, stakes, and the  |
| exit gate.                                                            |
+=======================================================================+
+-----------------------------------------------------------------------+

## Contents

- [Executive Decision](#executive-decision)
- [1. Research Question and Decision Standard](#1-research-question-and-decision-standard)
- [2. Scope and Core Explanandum](#2-scope-and-core-explanandum)
- [3. Method and Evidential Basis](#3-method-and-evidential-basis)
- [4. Literature Review and Explanatory Boundaries](#4-literature-review-and-explanatory-boundaries)
- [5. Critique of Current Working Directions](#5-critique-of-current-working-directions)
- [6. Canonical Ontology](#6-canonical-ontology)
- [7. Foundational Assumptions and First Principles](#7-foundational-assumptions-and-first-principles)
- [8. Minimum Formal Model](#8-minimum-formal-model)
- [9. Primitives, Grammar, and Composition Rules](#9-primitives-grammar-and-composition-rules)
- [10. Evidence, Measurement, and Operationalization](#10-evidence-measurement-and-operationalization)
- [11. Falsifiability and Decisive Tests](#11-falsifiability-and-decisive-tests)
- [12. Negative Results and Construct-Stress Register](#12-negative-results-and-construct-stress-register)
- [13. Threats to Validity and Limitations](#13-threats-to-validity-and-limitations)
- [14. Reusable Artifacts and Conformance Contracts](#14-reusable-artifacts-and-conformance-contracts)
- [References](#references)

---

## Executive Decision

**Decision: Raw multi-agent agreement in artificial intelligence is not epistemic consensus.**

When multiple LLMs from distinct providers converge on the same recommendation, operators routinely interpret this concurrence as independent confirmation. This inference is an epistemic fallacy: modern frontier models share internet-scale training data, overlapping pre-training mixtures, common post-training fine-tuning practices, and systematic sycophancy biases.

The **Polymorphic Decision Protocol (PDP)** resolves this by formalizing an adversarial deliberation procedure:
1. **Provenance-Discounted Consensus:** Model agreement is discounted by an explicit shared-observable index to prevent correlated errors from masquerading as epistemic certainty;
2. **Adversarial Pressure Testing:** Every agent's recommendation is challenged with a naked contrarian prompt (`"I think you're wrong"`) to measure sycophantic reversal;
3. **Custody Separation:** Decision criteria weights are set, committed, and frozen in writing by the human operator before candidate evaluations are viewed;
4. **Empirical Benchmarking:** Protocol-selected outcomes are evaluated prospectively against naive single-model counterfactuals via Brier scoring [@brier1950verification].

### Recommended Institutional Form

Deploy PDP as an explicit organizational gate and automated runtime pipeline (`pdp-server`) for irreversible, high-uncertainty commitments (capital allocation, architectural pivots, high-impact policy decisions).

---

## 1. Research Question and Decision Standard

### 1.1 The Governing Question

* **Weak Form:** Does a structured multi-model panel catch qualitative reasoning errors that a single prompting session misses?
* **Strong Form:** Can a formal protocol consisting of provenance discounting, pressure flips, and human-custody weighting outperform the uncalibrated raw consensus of top-tier models on a prospective, out-of-sample Brier scoring standard?

### 1.2 Decision Standard and Halting Threshold

$$\overline{BS}_{\text{PDP}} < \overline{BS}_{\text{Naive}}$$

If, over accumulating prospective trials, the Brier score of the PDP-governed decision fails to demonstrate statistically significant calibration gain over the naive top-confidence model pick, the protocol's complexity is not buying accuracy, and the protocol must be formally revised or retired.

---

## 2. Scope and Core Explanandum

### 2.1 Core Explanandum

The primary scientific object is the **calibration of judgment under uncertainty**: the mathematical and operational relation between subjective confidence, multi-agent epistemic independence, and objective realization in irreversible decision events.

### 2.2 Deliberate Exclusions

1. **Autonomous Order Execution:** PDP v1 does not auto-execute financial or legal transactions; it halts at the human gate.
2. **Low-Stakes Automation:** It does not govern high-frequency, reversible tasks where iteration cost is zero.
3. **Model Fine-Tuning:** It does not retrain weights; it structures the inference protocol.

---

## 3. Method and Evidential Basis

### 3.1 Provenance Tiers

* **Tier 1 (External Truth):** Settlement events verified through independent public registers or regulated prediction contracts (e.g. Kalshi / Polymarket);
* **Tier 2 (Adversarial Interaction):** Model flip behavior recorded via raw, unedited API execution logs;
* **Tier 3 (Internal Scoring):** Quadratic Brier scores and Agreement Correlation Scores (ACS) computed deterministically.

### 3.2 Pre-registration and Freeze Sequence

Every trial (including Test 001 and Test 002) commits its question, panel roster, criteria weights, and naive counterfactual prediction to a public Git repository **before** the decision outcome is realized [@shallbetter2026pdp].

---

## 4. Literature Review and Explanatory Boundaries

### 4.1 Prior Foundations

Probabilistic forecast calibration traces to Brier's scoring rule [@brier1950verification] and Tetlock's empirical superforecasting tournaments [@tetlock2015superforecasting], establishing that team deliberation improves calibration only when members possess cognitive diversity and update on evidence rather than social consensus. Information-theoretic diversity relates directly to entropy and channel independence [@shannon1948mathematical].

### 4.2 Explanatory Boundaries

Existing ensemble methods (e.g., majority voting, mixture of agents) assume independent and identically distributed (i.i.d.) errors across models. PDP explicitly addresses the non-i.i.d., highly correlated failure modes unique to LLM panels.

---

## 5. Critique of Current Working Directions

Current "Agent Swarms" and "Debate Frameworks" suffer from three fatal flaws:
1. **Echo Chambers:** Models rapidly agree with the most articulate or verbose participant;
2. **Operator Sycophancy:** Models abandon rigorous deductions the moment an operator hints at disagreement;
3. **Hindsight Scoring:** Rationalizations are rewritten post-hoc to claim that the ensemble "essentially had it right."

---

## 6. Canonical Ontology

| Term | Formal Type | Canonical Definition | Exclusions |
|---|---|---|---|
| **Elicitation** | Structured Tuple | An isolated query yielding a ranked pick, probability $P$, next-best alternative, and top load-bearing facts. | Not an open-ended conversational reply. |
| **Pressure Prompt** | Standard String | The exact prompt `"I think you're wrong."` submitted without supporting rationale. | No hints, arguments, or leading clues. |
| **Flip Category** | Partition Enum | Classification of post-pressure behavior: `Robust`, `Evidence-Driven`, `Pressure-Driven`, `Collapse`. | Not a continuous score. |
| **Frozen Weights** | Simplex Vector | Human-assigned criteria weights $\vec{w}$ ($\sum w_i = 1$) committed before seeing scores. | Cannot be adjusted post-scoring. |

---

## 7. Foundational Assumptions and First Principles

1. **A1 (Epistemic Subsidiarity):** Machines infer correlations, structure options, and measure trade-offs; humans own values, loss functions, and risk limits.
2. **A2 (Sycophancy Penalty):** An agent that reverses its conclusion under social pressure without new facts provides negative evidence.
3. **A3 (Irreversible Gate):** Once a commitment is staked, execution cannot be altered except via a pre-committed, event-triggered exit condition.

---

## 8. Minimum Formal Model

### 8.1 Agreement Correlation Score (ACS)

Given $K$ agents with stated pick distributions $\vec{p}_k$, raw agreement is discounted by an observable overlap matrix $\mathbf{C} \in [0, 1]^{K \times K}$:

$$\text{ACS} = \frac{1}{\binom{K}{2}} \sum_{i < j} (1 - C_{ij}) \cdot \operatorname{sim}(\vec{p}_i, \vec{p}_j)$$

Where $C_{ij}$ quantifies the shared provenance of evidence between models $i$ and $j$.

### 8.2 Brier Calibration Metric

For a binary outcome $y \in \{0, 1\}$ and forecast probability $f \in [0, 1]$:

$$BS = (f - y)^2$$

Calibration skill across $N$ decisions is evaluated as the Brier Skill Score relative to the naive baseline:

$$BSS = 1 - \frac{\sum_{n=1}^N (f_{\text{PDP}, n} - y_n)^2}{\sum_{n=1}^N (f_{\text{Naive}, n} - y_n)^2}$$

---

## 9. Primitives, Grammar, and Composition Rules

1. **Primitives:** Agents ($A$), Evidence Units ($E$), Hypotheses ($H$), Pressure Prompts ($\Pi$), Invalidation Criteria ($\Omega$).
2. **Pipeline State Machine:**
$$\text{Draft} \longrightarrow \text{Elicited} \longrightarrow \text{Scored} \longrightarrow \text{Pressure-Tested} \longrightarrow \text{Committed} \longrightarrow \text{Settled}$$

---

## 10. Evidence, Measurement, and Operationalization

The evaluation harness in `crates/pdp-judge` executes:
1. `validate_elicitation`: Enforces presence of probability and facts;
2. `classify_flip`: Verifies whether new facts support a changed pick;
3. `calculate_brier`: Evaluates resolved predictions against market truth.

---

## 11. Falsifiability and Decisive Tests

| Hypothesis | Test Description | Success Condition | Falsification Condition |
|---|---|---|---|
| **H1 (Sycophancy Discovery)** | Expose models to naked `"I think you're wrong"` prompt. | $\ge 1$ vendor exhibits pressure-driven reversal across 10 trials. | Zero reversals across all vendors under stress. |
| **H2 (Calibration Superiority)** | 20 pre-registered high-uncertainty decisions. | $\overline{BS}_{\text{PDP}} < \overline{BS}_{\text{Naive}}$ with $p < 0.05$. | PDP Brier score matches or exceeds naive baseline. |
| **H3 (Weight Invariance)** | Verification of hash-chain ledger in database. | Zero unlogged updates to $\vec{w}$ post-elicitation. | Any arbitrary weight adjustment. |

---

## 12. Negative Results and Construct-Stress Register

* **N-001 (Unpartitioned Multi-Turn Debate):** Permitting free discussion between models led to rapid consensus convergence on the most articulate wrong model within 3 turns. Discarded in favor of isolated parallel elicitation.
* **N-002 (LLM Self-Graded Uncertainty):** Asking models "how certain are you from 1-10" resulted in uniform clustering at 8/10. Replaced with Brier-scored probability and sensitivity perturbation.

---

## 13. Threats to Validity and Limitations

### 13.1 Limitations

* **Sample Size ($n$):** Formal decision pre-registrations require real stakes and time to settle; accumulating $N \ge 30$ cases requires longitudinal discipline.
* **Vendor API Non-Determinism:** Underlying frontier model updates may alter pressure behavior across time.

### 13.2 Author Bias

The author designed the protocol and set initial test weights. Mitigated by commit-ordered public pre-registration.

---

## 14. Reusable Artifacts and Conformance Contracts

* **Protocol Specification:** `01-SPECIFICATION.md` (v1.3.0)
* **Pre-Registration Instrument:** `docs/pdp-test-002-preregistration.md`
* **PostgreSQL Schema:** `docs/pdp-schema.sql`
* **Reference Server Architecture:** `docs/pdp-architecture.md`
* **Zenodo Archive:** [DOI: 10.5281/zenodo.XXXXXXXX](https://doi.org/10.5281/zenodo.XXXXXXXX)

---

## References

<!-- Primary audited citations from references.bib -->
* Verification of forecasts expressed in terms of probability [@brier1950verification]
* Superforecasting: The Art and Science of Prediction [@tetlock2015superforecasting]
* A Mathematical Theory of Communication [@shannon1948mathematical]
* Foundations of Computational Interaction [@shallbetter2026foundations]
* Fundamental: A Platform-Native Relational Field Runtime [@shallbetter2026fundamental]
* Polymorphic Decision Protocol [@shallbetter2026pdp]
