# Release artifacts

The PDP software v1.4.0 GitHub release is the prior published toolkit release. Version 1.4.1 is a maintenance release with unchanged runtime behavior. Its GitHub release contains separate manifest-controlled MIT software and CC BY 4.0 preprint/research archives, plus a standalone paper PDF. Paper version 1.0.0 is a public pre-review preprint; it is not independently reviewed or peer reviewed, author verification and declarations remain incomplete, and no DOI or Zenodo deposit is claimed.

## Version policy

`VERSION` contains the normative protocol version declared in [the specification](../docs/polymorphic-decision-protocol.md). The repository release is identified by its `vMAJOR.MINOR.PATCH` GitHub tag and recorded in `.zenodo.json` and `CITATION.cff`. The software ZIP is named for that repository release. Rust and npm packages use the versions declared in their own manifests. The protocol register records the normative protocol version. Update each version when its corresponding protocol, repository release, or software package changes.

For the next release, add the new reviewed manifest to the `.gitignore` allowlist and create `manifests/software-v<release>.csv` with the row `VERSION,VERSION,Normative protocol version,metadata,MIT`. Use the canonical repository paths for the version files listed by the builder and map `release-artifacts/software-readme.md` to the archive's `README.md`. In that source README, use `- **Repository release:** PDP Local Toolkit <release>` and retain the separate `Normative protocol` and `Rust and npm package versions` fields. Title the accompanying notes `# Repository release <release>`.

The published v1.4.0 and v1.4.1 software snapshots contain a stale `VERSION` value of `1.4.0`. Their tagged specifications and archive READMEs identify `1.3.1` as the normative version. For a historical software ZIP replay, export the original tag with `git archive` and use the builder's `--legacy-version-metadata` flag. Release automation verifies the original Git tree before enabling this compatibility path. The v1.4.1 replay matches its published checksum. The v1.4.0 replay with the curated builder differs from the published v1.4.0 checksum, as it did before this validation was added.

## GitHub software release automation

Publishing a stable `vMAJOR.MINOR.PATCH` GitHub release triggers `.github/workflows/software-release.yml`. The workflow exports the tagged source, requires the matching software manifest and the curated paper package manifest, builds both ZIPs and the standalone PDF with checksums, verifies archive integrity and package metadata, then attaches all six assets without replacing existing release assets. Re-running the workflow accepts already-attached assets only when their bytes match the deterministic build.

Each new repository release requires a reviewed, version-matched manifest and synchronized metadata in that tag. Pull requests run `python3 release-artifacts/version_metadata.py` on the checkout. The release builder runs those checks on the tagged source and validates the staged archive against the same version values. A missing manifest, mismatched `.zenodo.json` or `CITATION.cff` version, invalid archive, partial asset set, or differing pre-existing asset fails closed. The paper ZIP is a separate license and version boundary; it includes its research files under its own exact manifest.

The workflow publishes GitHub release assets only. It does not deposit files to Zenodo; RPP reserves archival deposit and post-deposit verification to the human research owner. The PDP repository guide requires generated `llms` corpora in the source repository; `.gitattributes` excludes those files from GitHub-generated source archives, and the software package builder rejects them from curated ZIPs.

`prepare-packages.py` prepares the local software and paper package trees. The GitHub workflow validates and archives the committed curated paper tree without regenerating paper content. `github-release-v1.4.1.md` is the release-page text. GitHub preprint release does not perform a Zenodo deposit; RPP reserves archival deposit and post-deposit verification to the human research owner.

`build-software-release.py` and `build-paper-release.py` share the deterministic ZIP writer in `package_archive.py`. The paper builder verifies its manifest, payload checksums, review-status metadata, PDF signature and size, and citation/version metadata before generating release assets.

## Software and paper v1.4.1 release build — 2026-11-01

- The local build from `manifests/software-v1.4.1.csv` contains 30 files and passes exact manifest parity, internal checksums, ZIP CRC integrity, and version parity checks.
- The software archive contains no generated LLM corpora, paper files, or research records. Its SHA-256 is recorded with the release asset after the GitHub workflow completes.
- The paper archive contains the CC BY 4.0 manuscript and RDP/RPP research records declared in its manifest. It excludes private case records. PDF searchability and file-list parity are checked before release; the GitHub workflow verifies package checksums, PDF signature/size, and ZIP integrity.
- Neither release asset set represents independent review, peer review, empirical validation, author verification, DOI assignment, or Zenodo deposit.

## Historical verification record — 2026-10-31

- PDP: Rust tests 10/10; TypeScript tests 10/10.
- RDP: 63 tests pass; project registry and paper checks pass. The registry reports the expected X5 warning because no claim rests on independent evidence. The reconstructed NPS example passes; its `--git` check fails by design because it has no registering commits.
- RPP: 36 tests pass; gates 1–5 pass with no warnings after the data, analysis, and results directories were marked not applicable.
- The PDF is searchable, 10 pages, 92,554 bytes, and under the 5 MB target. Its embedded title and author match the manuscript; paper Markdown/PDF copies match the RDP source byte-for-byte.
- PDP software v1.3.3 ZIP: 28 files, 44,106 bytes; SHA-256 `499061701efbbc079bb70eb51c79fdf4c579406ffa28ae2db22c2c8b8916172e`.
- Paper ZIP: 58 files, 270,761 bytes; SHA-256 `f5a8b72a74bbe72f0333b866326ca5ea4f9a1eec860b7eb5939649d248511bdb`.
- Both ZIPs pass archive integrity and package-boundary scans and reproduced byte-for-byte on a same-day rebuild.

These checks establish packaging and implementation facts only. They do not establish scientific validity, empirical effectiveness, independent review, or publication readiness. Author declarations, author verification, independent reviews, and human release/deposit checks remain open.
