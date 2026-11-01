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
