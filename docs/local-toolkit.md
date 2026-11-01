# PDP Local Toolkit 1.4.0

**Status:** Software release 1.4.0. The protocol specification is v1.3.1. The app implements a human-gated local workflow through process audit; it has not been independently security reviewed or empirically evaluated.

## Start

On macOS, run `npm run start:local` from the repository root, then open `http://127.0.0.1:43173`. The service binds only to IPv4 loopback. It does not support remote deployment, accounts, telemetry, or cross-origin requests.

`PDP_LOCAL_PREVIEW_ONLY=1 npm run start:local` uses an in-memory database seeded with a fictional check-in decision. It does not read Keychain or make hosted provider/search calls; local Ollama role calls remain available. Records disappear when the process stops. Use this mode for demonstration, not durable research records.

The durable records directory is `~/Library/Application Support/ProductAI/PDP`, protected for the current user. The SQLCipher database key and provider credentials are held in macOS Keychain. Provider credentials are loaded only after an explicit call and cached in the service process. They are never returned to the browser or written to decision records. A Keychain authorization prompt can still occur on first access after a service restart or signing-identity change.

## Implemented workflow

- Create decisions, assign all four protocol evidence roles, add source-linked records, and require evidence coverage before starting the panel.
- Pin three distinct model-vendor IDs, model IDs, serving adapters, and a run budget. Vendor identity and serving adapter are separate fields. OpenAI Responses, Anthropic Messages, and local Ollama adapters run calls in-app. Any other vendor can be represented and used through manual prompt and transcript import (including BonzAI Web); this does not automate that vendor's interface. Google Gemini is paused after the recorded HTTP 429 and is omitted from new panel choices. Historical Google adapter records remain readable. No silent fallback occurs.
- Preview each exact role prompt and explicitly approve each hosted or local model call. Calls run one role at a time with one evidence lane; provider-native web search is disabled. Responses are schema-checked before they become elicitation records. The same calls can use the pinned Ollama model. Provider output is not verified evidence.
- Search using Brave only after per-query preview and approval. Result cards exist only in the current response/UI, are not recorded, and are not sent to providers. Users must open and inspect a result and create a curated evidence record manually. A compatible Brave plan and its API key are required; standard terms do not grant result retention or reuse. Brave subscription charges are separate from the model-call estimate and are not capped by the app.
- Compute ACS after all twelve vendor-role records, record the human G1 review, and keep approval/rejection as a human action.
- At G2, collect twelve separate self-critiques, freeze one human-owned formal model with justified weights and option scores, then collect twelve responses to the exact “I think you're wrong.” pressure prompt. Each phase is appended separately and the service blocks phase skipping. G2 displays ACS, weighted scores, Flip Distance, the sensitive dimension, and the protocol sizing cell. The human records all four invalidation tests and approves or rejects the gate.
- At G3, validate quarter-Kelly sizing against the matrix multiplier and ruin-tolerance cap, record the terminal/checkpoint/single-event allocation bounds, and require an explicit human lock. The toolkit records a proposal; it does not execute a purchase, deployment, publication, or other commitment.
- Record manual world-state monitoring checkpoints, invalidation triggers, settlement, and an eight-criterion process audit. The event-history API withholds settlement details until the audit is saved. The blind flag is an auditor attestation and cannot prove that a person had no outside access to the outcome.

Every durable workflow record is versioned and stored as an append-only event in the encrypted database. Event history is checked against its hash chain when read. It detects altered, missing, or reordered event rows but does not provide an independent signature or protect against replacement of the entire local database and Keychain item.

## Cost and provider limits

- Each model request is limited to a 20 KB prompt and at most 1,000 output tokens, uses a fixed HTTPS or loopback endpoint, disables redirects, has a timeout and bounded response, and is not automatically retried. No prompts, keys, or provider response bodies are written to logs.
- Before a model call, the app reserves an estimate against the pinned run budget. Current reservation values are local estimates (zero for local Ollama, $0.001 for OpenAI, and $0.01 for Anthropic/Google); they are not provider-enforced spend caps and may differ from actual billing. Search subscription charges are separate and are not accounted in the model run budget. The UI discloses this before approval.
- Brave results are transient only. Plan-specific storage rights are not inspected by the app, so persistent search-result storage and cross-provider reuse remain off.
- Search results are not source validation. Provider statements and generated classifications require human review and do not establish empirical truth.
- Model calls are sequential and manually approved. There is no batch orchestration, retry policy, cancellation after dispatch, or automated provider failover.

## Not implemented

- Encrypted full-record export, passphrase-protected bundles, redacted report preview, key rotation/recovery, and storage/export/import hash equivalence.
- Independent reviewer blinding, external evidence verification, automatic monitoring, exchange/broker integration, transactions, and external decision authority.
- A complete calibration ledger or aggregate model-performance report; the audit contains a manually entered calibration note only.
- Evidence that the protocol is effective or empirically validated. A completed toolkit workflow is an implementation demonstration, not validation of PDP claims.
