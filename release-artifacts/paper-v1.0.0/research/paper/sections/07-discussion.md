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
