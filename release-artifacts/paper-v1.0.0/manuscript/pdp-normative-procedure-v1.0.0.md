---
title: "Polymorphic Decision Protocol: A Normative Procedure for High-Uncertainty Decisions with AI Panels"
author: "Zachary Shallbetter"
date: "2026-11-01"
version: "1.0.0"
scope: "Critical analysis of the normative PDP v1.3.0 procedure and its testable limits; no empirical efficacy claim."
license: "CC-BY-4.0"
doi: "Not assigned"
lang: en
abstract: >-
  This Type II protocol paper analyzes the normative design, assumptions,
  implementation boundary, and falsifiability of PDP v1.3.0. It reports no
  efficacy result. A private author-reported case is included only as
  development context and is not independently verified. Test 002 is unexecuted.
keywords:
  - decision protocols
  - language models
  - forecast combination
  - calibration
  - human oversight
  - research methods
---

# 00. Implementation status matrix

| Capability | Status at frozen repository baseline | What the status establishes |
|---|---|---|
| Normative decision procedure | Documented at protocol v1.3.0 | Requirements exist in prose; no conformance determination is implied |
| Mathematical routines | Implemented and unit-tested in Rust | Only tested code paths are covered; no empirical decision validity follows |
| UI behavior | Implemented and tested in TypeScript | Local interface behavior only |
| Multi-provider orchestration | Design only | Not represented as implemented |
| Durable weight custody and append-only audit | Proposed schema and architecture | No deployed access-control or operational evidence supplied |
| Pressure-response classification | Normative proposal | No validated instrument, reliability estimate, or cross-vendor corpus supplied |
| Prospective performance comparison | Planned, not executed | Test 002 has no results |

The matrix separates **documented**, **implemented**, **tested**, and **empirically validated**. These states are not interchangeable. The current paper reports no empirical validation.

# 1. Abstract and executive decision

## Abstract

The Polymorphic Decision Protocol (PDP) is a normative procedure for organizing high-uncertainty decisions made with AI panels while reserving decision authority for a human. This paper examines the protocol as a designed method, not as an evaluated intervention. We reconstruct its four decision phases and portfolio-governance extension, identify the assumptions behind its consensus and pressure-response rules, distinguish documented requirements from the limited software implementation, and specify empirical conditions under which the method's claims could fail. The current protocol proposes provenance-aware panel formation, ordered adversarial review, frozen human-set criteria, commitment and invalidation rules, and outcome-blind audit. Its adjusted convergence score uses fixed categorical weights based on vendor and evidence-class differences; those weights are not estimates of error dependence. The protocol's effectiveness, measurement validity, safety, and generality remain unestablished. A private author-reported World Cup case is mentioned only as development context and is not independently verified. The planned Test 002 has not been executed. The paper's defensible contribution is therefore a bounded protocol specification and test agenda. It makes no claim that PDP improves forecast accuracy, prevents sycophancy, or outperforms a baseline.

**Keywords:** decision protocols; language models; forecast combination; calibration; human oversight; research methods

## Executive decision

PDP v1.3.0 is a normative candidate that specifies a potentially testable sequence of decision controls. The available record supports describing what the specification requires and what parts of the repository implement. It does not support a conclusion that the procedure improves decisions, calibrates AI systems, identifies sycophancy reliably, or transfers across domains. The immediate research decision is to submit the protocol and its measurement assumptions to independent expert review, then freeze a study-specific protocol before any prospective evaluation.

## Author and status

Zachary Shallbetter, Independent Researcher; ORCID: [0009-0009-9450-3429](https://orcid.org/0009-0009-9450-3429). Manuscript version 1.0.0; protocol baseline 1.3.0. Public pre-review preprint released 2026-11-01 ([release page](https://github.com/zachshallbetter/polymorphic-decision-protocol/releases/tag/v1.4.1)). It has not received independent review or peer review. No DOI is assigned. Author verification of claims, mathematics, implementation-status descriptions, and citations remains outstanding.

## Claim boundary

The paper makes three bounded claims: (1) the frozen specification documents a staged procedure; (2) the repository contains tested components but not an end-to-end implementation; and (3) comparative efficacy is unresolved. These are claims about documents, code, and missing admissible evidence. They are not claims about causal effects or real-world performance.

# 2. Introduction

## 2.1 Research question

The immediate question is descriptive and methodological: what decision procedure does PDP v1.3.0 specify, what assumptions do its controls require, and what evidence would be necessary to evaluate them? The future empirical question is whether, under a defined decision population and comparison design, use of PDP changes decision quality or calibration relative to a prespecified baseline. This paper answers only the first question.

## 2.2 Contribution type

The paper is a Type II protocol specification. It describes a proposed method, maps its normative rules to an implementation boundary, and identifies measurable failure conditions. It does not report a completed study, a validated instrument, a causal model, or a validated intervention. Its contribution claim is limited to presenting and critically bounding the existing v1.3.0 protocol in a scholarly form.

## 2.3 Decision standard

Protocol review should establish whether requirements are internally coherent, measurable, feasible, and ethically acceptable. Empirical evaluation requires a separate preregistration that specifies population, sampling, treatment, comparator, outcomes, estimand, stopping rule, uncertainty procedure, and missing-data handling. Neither successful software tests nor expert agreement can substitute for that evaluation.

## 2.4 Core explanandum

The core explanandum is the design of a decision procedure that constrains how panel outputs are elicited, challenged, and recorded while retaining human authority over value choices and commitment. This paper describes that proposed procedure; it does not explain or measure real-world decision outcomes.

# 2.5 Scope and exclusions

## Scope

PDP concerns decisions with uncertain outcomes and a commitment point. Its normative text organizes model elicitation, evidence review, pressure testing, human custody of weights and stakes, commitment controls, monitoring, and post-outcome audit. The protocol can be analyzed as a governance procedure even when it is not implemented as software.

## Exclusions

This paper does not establish or claim: predictive accuracy; causal improvement over single-model or human baselines; prevalence of sycophancy; validity of the adjusted convergence score; safety for financial, medical, legal, hiring, or other high-impact use; transfer across domains; correctness of private source data; or public validation of the originating case. The current code is not an autonomous trading system. No live execution or recommendation should be inferred from this paper.

## Development illustration

The protocol originated in part from an author-reported 2026 World Cup decision involving a multi-model panel. The underlying conversations and trade records remain private and are not included here. The account is not independently inspectable, was not collected under the proposed protocol, and is vulnerable to recall, selection, and outcome bias. It is cited only as a development illustration. No numerical case result is reproduced, and no claim about the case is treated as verified evidence. Test 002 remains planned and unexecuted.

# 3. Prior art and explanatory boundaries

Probabilistic scoring has a mature foundation. Brier's score evaluates the accuracy of probability forecasts against realized outcomes [@brier1950]. Forecast combination also predates language models; Bates and Granger showed that combinations can reduce mean-square error in a particular forecasting context, with weights informed by historical forecast errors [@batesgranger1969]. PDP therefore cannot claim novelty for probability scoring, forecast combination, or the general insight that dependence matters.

Research on multi-agent debate evaluates how interaction strategies affect model performance, with results varying by task and strategy [@smit2024]. Research on sycophancy studies model behavior that aligns with user views at the expense of truthfulness under specified tasks [@sharma2023]. These literatures make the motivating concerns scientifically plausible research topics. They do not establish that PDP's exact prompt, categorical flip labels, or sequence controls measure the same constructs or improve outcomes.

PDP combines governance rules and operational proposals: evidence-class restrictions, a two-axis categorical agreement adjustment, a challenge prompt, frozen human-set weights, commitment limits, and audit procedures. This paper does not claim that their combination is novel or more effective. A broader systematic prior-art search and independent novelty assessment remain required. The source register identifies the records checked for the cited bibliographic details; those records do not establish PDP's empirical effectiveness.

# 4. Method and evidential basis

## 4.1 Document analysis

This paper treats the v1.3.0 normative specification as its object [@pdpv130]. The baseline is repository commit `35d945a04e622a560c8e1be53ffc0a796be1aa79`, corresponding to the source snapshot associated with repository release v1.3.2. The manuscript itself is a later analysis and does not alter protocol rules. Repository code and tests are inspected as implementation artifacts, not as evidence of empirical effectiveness.

## 4.2 Evidence lanes

Statements about protocol requirements are document-grounded. Statements about software are repository-grounded and bounded to inspected source and tests. Claims about effectiveness, prevalence, user outcomes, or generality are pending proof. The private originating case is not admissible for independent verification in this paper. No new experiment or participant data were collected for this manuscript.

## 4.3 Freeze and registration

The normative baseline is frozen for this analysis at the commit above. The paper's claims and Test 002 prediction are recorded in the RDP registry, but the present working tree is uncommitted. The prediction is therefore explicitly marked pending registration. It must be committed before Test 002 starts; there is no claim of commit-ordered preregistration yet.

## 4.4 Review method

Two independent reviews are planned: one on research design and measurement, and one on AI evaluation or protocol methods. Reviewers have not been contacted and no feedback has been received. The review packet is prepared separately. Reviewer selection, independence, conflicts, and dispositions must be recorded before any reviewed-preprint status is used. The public GitHub version is a pre-review preprint, not peer reviewed; its release does not imply independent review.

# 4.5 Minimum formal model

Let decision instance $d$ have candidate set $A_d$, declared information state $I_d$, human criteria weights $w_d$, panel agents $K_d$, and eventual outcome $y_d$. Agent $k$ returns forecast distribution $p_{kd}$ after observing its permitted evidence $E_{kd} \subseteq I_d$. The protocol records a decision procedure $\pi$ mapping the declared inputs and responses to a recommendation record, commitment gate, and audit record.

For a categorical outcome with $J$ possible values, the multicategory Brier score is

$$BS(p,y)=\sum_{j=1}^{J}(p_j-\mathbb{1}[y=j])^2.$$

For a binary event this can be reported in the convention $BS=(p-y)^2$. Lower scores are better for the observed forecast set, but a sample estimate requires a defined sampling process and uncertainty analysis. A skill score against baseline forecasts $q_d$ may be written

$$BSS=1-\frac{\sum_d BS(p_d,y_d)}{\sum_d BS(q_d,y_d)}.$$

It is undefined when the baseline denominator is zero and can be unstable in small or dependent samples. This paper reports no Brier scores and no Brier skill score.

The v1.3.0 adjusted convergence score is a categorical rule over agreement indicators and declared weights. Its weights are not identified from a covariance model, and the current paper does not reinterpret them as such. No theorem in this paper establishes that the score estimates consensus reliability or decision quality.

# 5. Proposed evaluation and falsifiability

## 5.1 Decisive tests

| Claim under test | Design needed | Failure condition |
|---|---|---|
| Pressure-response labels are reproducible | Blinded raters, controlled repeated prompts, frozen context, prespecified coding manual | Reliability is inadequate or labels change materially under irrelevant wording changes |
| Categorical agreement adjustment tracks dependence or improves decisions | Repeated forecasts, outcome data, comparison with explicit dependence estimates and simple baselines | No incremental predictive or decision value, or weights worsen calibration under the prespecified analysis |
| Procedure improves forecast quality | Prospective sample, registered estimand, comparator, complete outcomes, uncertainty intervals | PDP is not better than the baseline under the declared decision criterion, or causes material harms |
| Human custody controls are operationally followed | Independent audit of frozen criteria, overrides, and commitment records | Unlogged changes occur or the procedure cannot preserve the stated authority boundary |

These are proposed tests, not findings. Sample size, population, outcomes, and statistical decision rules must be specified before execution. A single case cannot establish broad performance.

## 5.2 Limitations

The protocol is a normative candidate. The literature search is preliminary. The private originating case cannot be audited here. No prospective study, independent software review, or protocol conformance assessment is reported. The categorical consensus weights lack empirical calibration. The challenge-based classifier may confound multiple causes of response change. Existing tests establish only behavior of tested code paths. No operational safety or regulatory suitability claim is made.

## 5.3 Author and funding disclosures

The sole author originated PDP and is not independent of its design. Author contributions under the CRediT taxonomy, funding status, and any additional competing-interest declarations remain unconfirmed; no unverified declaration is made here. The author is identified as an Independent Researcher. This public pre-review preprint was released before those confirmations; the release does not imply that the declarations are complete.

## 5.4 Data, code, and AI assistance

No new study data were collected. Private originating-case records and transcripts are not included and are unavailable for independent verification. PDP source code is available in the associated repository under its software license; this paper is licensed separately under CC BY 4.0. OpenAI Codex assisted with repository inspection, literature discovery, critical analysis, manuscript editing, and preparation of research records and release artifacts. It is not an author. Author verification of claims, mathematics, code, and citations remains outstanding in this pre-review version.

## 5.5 Review and publication status

Two independent expert reviews are planned and have not yet been obtained. This is a public pre-review preprint; it is not independently reviewed or peer reviewed. Reviewer comments will not be represented as empirical validation.

# 6. Results and negative evidence

This analysis generated no empirical results. No participants, model outputs, forecasts, or outcomes were collected or reanalyzed. The author-reported development case is not independently inspectable and is not used as a result.

The repository inspection identified a difference between the normative scope and the implementation: mathematical routines and UI behavior have code and tests, while multi-provider orchestration, durable governance controls, and end-to-end audit remain design proposals. This is a status finding about repository contents, not a performance finding.

No admissible positive or negative efficacy result is available. The research record therefore treats the central performance question as unresolved and defines conditions that could falsify the proposed measurement and outcome claims.

# 7. Discussion: protocol analysis, ontology, and assumptions

## Stages

The v1.3.0 specification describes thesis formation, adversarial stress testing, structured commitment, monitoring and settlement, and portfolio governance across concurrent decisions. Stage gates are intended to prevent commitment when a required step fails. The paper reports these as normative design features, not observed operating properties.

## Human authority

The specification reserves value weights, stakes, and exit decisions to a human. This is a governance allocation. It does not establish that human judgments are accurate, unbiased, or legally sufficient. A frozen weight vector can preserve an invalid value model just as faithfully as a defensible one; the specification must therefore be evaluated separately for procedural integrity and decision quality.

## Pressure response

The protocol proposes an ordered challenge without new evidence and labels response patterns such as robust, evidence-driven flip, pressure-driven flip, and collapse. Such labels require an operational coding rule. The presence of a challenge alone cannot identify the cause of an answer change: stochastic variation, context effects, new facts, or ambiguity may also matter. The classifier needs blinded human or validated automated scoring, reliability estimates, and controls before it can be treated as a measurement instrument.

## Consensus adjustment

The current specification assigns categorical weights according to vendor and evidence-class differences. The values are design choices, not empirically estimated dependence coefficients. Distinct vendors do not guarantee independent errors, and shared evidence does not fully determine error correlation. Until calibrated against outcome data, the score should be interpreted as a declared governance heuristic, not as a probability, effective sample size, or validated measure of epistemic independence.

## Commitment and audit

Precommitted conditions and outcome-blind audit can make decision records more inspectable. They also introduce operational burdens and risks: ambiguous triggers, missing records, changing environments, and pressure to treat a mechanical threshold as objective authority. The protocol should report compliance separately from accuracy and utility.

## Canonical ontology

| Term | Type | Definition in this analysis | Exclusion |
|---|---|---|---|
| Candidate decision | Decision object | A bounded choice under uncertainty before commitment | Not an AI recommendation or realized outcome |
| Claim | Proposition | A statement whose support and failure condition can be assessed | Not a preference or confidence expression alone |
| Evidence class | Input constraint | A declared category restricting what an agent may use | Not proof that two agents' errors are independent |
| Agreement adjustment | Governance heuristic | The v1.3.0 categorical weighting of agreement based on vendor and evidence-class differences | Not a calibrated probability or dependence estimate |
| Pressure response | Observed response category | A coded change or stability after a declared challenge | Not proof of sycophancy without validated design |
| Frozen weights | Human-set vector | Criteria weights recorded before evaluation and not silently changed | Not evidence that the weights are correct |
| Invalidation condition | Prospective rule | An observable condition paired with a predeclared response | Not an agent utterance or retrospective explanation |
| Protocol compliance | Process measure | Whether declared procedure requirements were followed | Not decision accuracy, safety, or utility |

## Assumptions and first principles

PDP assumes that input provenance can be described well enough to inform panel construction; criteria and weights can be set before model scores are observed; a challenge can be applied without adding evidence; response categories can be distinguished reliably; commitment and invalidation can be operationalized; and outcomes can be scored on a declared horizon. Each can fail in practice and requires its own evidence.

The human-authority principle is normative, not an empirical axiom. It assigns authority but does not establish that the decision owner has expertise, independence, or adequate information. A frozen weight vector can preserve an invalid value model as faithfully as a defensible one; procedural integrity and decision quality must therefore be evaluated separately.

The model-independence assumption is particularly uncertain. Provider and evidence labels are observable proxies, while residual dependence must be measured from repeated forecasts and outcomes. The current protocol supplies no validated dependence model.

# 8. Conclusion

PDP v1.3.0 specifies a staged governance procedure for uncertain decisions involving AI panels. Its strongest present contribution is a set of explicit procedural requirements that can be inspected, challenged, and converted into future tests. The present record does not show that the procedure improves accuracy, calibration, safety, or utility. The categorical agreement adjustment and pressure-response labels require measurement validation, while code tests must remain distinct from system-level and empirical evidence.

The next defensible step is critical review of the protocol's research design and measurement assumptions. Any empirical study must be frozen before execution and must report outcomes irrespective of direction. Until that work is complete, PDP should be described as a normative candidate and the originating case as an unverified author report.

# 9. Illustrative example

## 9.1 Author-reported illustration

An author-reported World Cup decision motivated several protocol rules. The records are private and unavailable for independent inspection. The decision was not prospectively conducted under the final protocol, and the author both designed the rules and interprets the case. It is therefore a development illustration only. No outcome, score, or quoted transcript is presented as verified evidence. The case cannot establish that any rule caused a decision, detected a model behavior, or generalizes.

## 9.2 Implementation-status matrix

| Capability | Repository status | Evidence boundary |
|---|---|---|
| Core mathematical routines | Implemented in Rust | Unit tests exercise the implemented functions; no empirical validity follows |
| UI behavior | Implemented in TypeScript | UI tests cover local behavior only |
| Multi-provider orchestration | Design document only | No end-to-end orchestration is claimed |
| Durable custody and append-only audit controls | Proposed schema/design | Schema presence is not proof of deployed permissions or operational control |
| Pressure-response classification across vendors | Proposed method | No validated corpus, inter-rater study, or cross-vendor evaluation supplied |
| Prospective performance comparison | Planned, not run | Test 002 remains unexecuted |

## 9.3 Negative evidence

No admissible negative or positive empirical result is reported. The absence of admissible evidence is not evidence that the protocol succeeds or fails. The negative-results register records the open prediction and the conditions that would expose measurement failure.

# 10. Concept and implementation audit, limitations, and disclosures

| Protocol concept | Normative location | Software evidence | Unresolved issue |
|---|---|---|---|
| Adjusted convergence score | PDP v1.3.0, panel formation | Mathematical subset in `pdp-core`; unit tests | Fixed category weights are not estimates of error dependence |
| Pressure-response classes | PDP v1.3.0, stress test | No validated cross-vendor classifier | Coding reliability, stochastic response, and alternative causes |
| Frozen human weights | PDP v1.3.0, formal model and commitment | Repository code does not constitute a complete custody service | No deployed system or independent control audit |
| Invalidation and commitment gates | PDP v1.3.0, commitment and monitoring | Architecture/schema proposals | Ambiguity, authority, and real-world enforceability |
| Calibration audit | PDP v1.3.0, outcome audit | Brier-related math is not a longitudinal study | Baseline definition, sample dependence, decision utility |
| Portfolio governance | PDP v1.3.0, concurrent decisions | No complete service implementation | Exposure correlation and operational estimation |

The code and architecture are not interchangeable. Where the architecture describes components absent from code, this paper calls them design proposals. The paper does not treat a schema as an enforced permission boundary unless deployed database grants and operational tests demonstrate that behavior.

## Disclosures and publication conditions

The sole author originated PDP and is not independent of its design. The author is identified as Zachary Shallbetter, Independent Researcher (ORCID 0009-0009-9450-3429). **CRediT:** role assignments have not been confirmed by the author; no roles are assigned. **Funding:** status requires author confirmation; no funding declaration is made. **Competing interests:** the author developed PDP and has an intellectual interest in its evaluation; any additional financial or nonfinancial interests remain unconfirmed.

No new study data were collected. Private originating-case records and transcripts are not included and are unavailable for independent verification. PDP source code is available at [the public repository](https://github.com/zachshallbetter/polymorphic-decision-protocol) under its MIT license. This paper is licensed under CC BY 4.0. OpenAI Codex assisted with repository inspection, literature discovery, critical analysis, manuscript editing, and preparation of research records and release artifacts. It is not an author. Author verification of claims, mathematics, code, and citations remains outstanding in this public pre-review version.

Two independent expert reviews are planned and have not yet been obtained. This is a public pre-review preprint; it is not independently reviewed or peer reviewed. Reviewer comments will not be represented as empirical validation.

## References

The bibliography is provided in the accompanying `references.bib` file and is rendered from citation keys in the Markdown and PDF sources.
