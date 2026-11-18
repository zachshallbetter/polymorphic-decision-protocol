# Contributing to the Polymorphic Decision Protocol

This repository holds the protocol specification, its reference architecture and schema, a Rust and browser local toolkit, and a research plan. Read the [README](README.md) first: it separates what is documented from what is implemented and what is validated, and contributions are held to the same distinction.

Protocol doctrine, editorial rules and quality gates are in [`AGENTS.md`](AGENTS.md) and [`.agents/AGENTS.md`](.agents/AGENTS.md). Read them before editing a specification, the database schema, or the code. They apply to people and agents alike.

## Setup

You need a Rust toolchain and Node.js with support for `--experimental-strip-types`. The `Dockerfile` builds with `rust:1.80`.

```sh
git clone https://github.com/zachshallbetter/polymorphic-decision-protocol.git
cd polymorphic-decision-protocol
npm install
```

To try the local toolkit without Keychain or a database, run the in-memory preview:

```sh
PDP_LOCAL_PREVIEW_ONLY=1 npm run start:local
```

## Checks

```sh
npm test            # test:rust, test:ts and test:ui together
npm run test:rust   # cargo test
npm run test:ts     # node --experimental-strip-types ui/pdp-core.test.ts
npm run test:ui     # node --check ui/local-app.js
python3 scripts/gen-llms.py   # after changing README.md, AGENTS.md, .agents/AGENTS.md, docs/ or the local toolkit guide
```

`scripts/gen-llms.py` regenerates `llms.txt` and `llms-full.txt` (and their versioned copies) from the documents; commit its output with the change.

## Conventions

- **Authority order.** Protocol specification, then architecture design, then database schema, then Rust and UI code, then generated artifacts. Code and generated files never silently change a specification.
- **Claim tiers.** Documented, implemented, tested and empirically validated are different claims. When in doubt, state the weaker one.
- **Database discipline.** Weight custody rows are frozen on creation. Event, pressure-test and fill records stay append-only.
- **Preregistration.** A new decision-evaluation trial is preregistered before it runs. Negative results stay on record.
- **Versions.** The protocol version, the repository release and the package version are independent. A release needs a version-matched manifest in `release-artifacts/manifests/`, and `.zenodo.json`, `CITATION.cff` and the package README must agree on the version.
- **Commits.** Short, imperative subjects, with a conventional prefix where it fits (`ci:`, `docs:`, `feat:`, `release:`).
- **Changelog.** Record protocol and software changes in `CHANGELOG.md`.

## Pull requests

Describe what changed and why, list the checks you ran, and keep the diff to one concern. Use the pull request template.

Report vulnerabilities through [SECURITY.md](SECURITY.md), not a public issue.
