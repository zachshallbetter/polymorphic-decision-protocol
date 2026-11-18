# Security Policy

## Reporting a vulnerability

Report security issues privately through GitHub private vulnerability reporting:
<https://github.com/zachshallbetter/polymorphic-decision-protocol/security/advisories/new>.
Do not open a public issue for a vulnerability.

Include a description of the issue and its impact, steps to reproduce (a minimal example is ideal), and the affected version or commit.

This is a solo-maintained project. Handling is best-effort with no SLA: there is no guaranteed acknowledgement or fix time, and the maintainer will respond when next active.

## Scope

This repository holds a protocol specification, its reference documentation and a local toolkit. The security-relevant surface is the local toolkit (`crates/pdp-local`, `ui/local-app.*`), a macOS loopback service and browser interface. In scope:

- The loopback service: local-origin checks, request body limits, and anything that lets another local process or a web page reach or alter decision records.
- Storage: the SQLCipher database, the database key held in macOS Keychain, and the append-only hash-chained event records.
- Model and search calls (OpenAI Responses, Anthropic Messages, local Ollama, Brave search): the per-call preview and approval step, and any path that sends data or uses API credentials without that approval.
- The release workflow (`.github/workflows/software-release.yml`) and packaging scripts under `release-artifacts/`.

The application does not execute consequential actions and does not export encrypted records. Flaws in the protocol's reasoning or its mathematics are not security issues; report them as ordinary issues. No secrets or API keys belong in the repository.

## Supported versions

Only the latest tagged release receives fixes.
