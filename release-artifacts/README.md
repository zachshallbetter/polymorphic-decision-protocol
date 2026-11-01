# Release artifacts

The PDP software v1.4.0 GitHub release is the prior published toolkit release. Version 1.4.1 is a maintenance release with unchanged runtime behavior. Its GitHub release contains separate manifest-controlled MIT software and CC BY 4.0 preprint/research archives, plus a standalone paper PDF. Paper version 1.0.0 is a public pre-review preprint; it is not independently reviewed or peer reviewed, author verification and declarations remain incomplete, and no DOI or Zenodo deposit is claimed.

## GitHub software release automation

Publishing a stable `vMAJOR.MINOR.PATCH` GitHub release triggers `.github/workflows/software-release.yml`. The workflow exports the tagged source, requires the matching software manifest and the curated paper package manifest, builds both ZIPs and the standalone PDF with checksums, verifies archive integrity and package metadata, then attaches all six assets without replacing existing release assets. Re-running the workflow accepts already-attached assets only when their bytes match the deterministic build.

Each new software version requires a reviewed, version-matched manifest and synchronized release metadata in that tag. A missing manifest, mismatched `.zenodo.json` or `CITATION.cff` version, invalid archive, partial asset set, or differing pre-existing asset fails closed. The paper ZIP is a separate license and version boundary; it includes its research files under its own exact manifest.

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
