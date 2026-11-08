# Releases and pre-releases

## Version and channel

`Cargo.toml` is the source of the plugin version. A release tag must be exactly
`v` followed by that version, and `CHANGELOG.md` must contain exactly one matching
entry. Generated Pawn includes must match the package version.

Mark a pre-release entry with `## [VERSION] - YYYY/MM/DD - Pre-release`; the date
is optional. Stable entries use
`## [VERSION]` or `## [VERSION] - YYYY/MM/DD`. Replace `VERSION` with the Cargo
version. Pre-release versions include a SemVer suffix such as `-alpha.1`,
`-beta.3` or `-rc.1`. The version, tag, changelog heading and GitHub pre-release
flag must all identify the same channel.

Published versions and tags are never reused or rewritten. A final changelog
describes the result since the previous stable
release; remove intermediate candidate entries and candidate-only history.

## Prepare and approve

Update Cargo, rebuild the includes, update the API reference when needed, and
write the release's changelog entry. Run the normal validation described in
[building](building.md). Preview the exact metadata and notes locally:

```sh
cargo xtask prepare-release --tag vVERSION --channel prerelease --out dist/release
```

Use `--channel release` for a stable version. A channel mismatch, missing or
duplicate changelog entry, empty notes, or tag/version mismatch fails preparation.
The command writes `metadata.json` and `notes.md`; it does not create a tag or
publish a release.

Show the maintainer the version, channel and changelog and obtain approval before
creating or pushing a signed tag. Pushing an approved `v*` tag starts `release.yml`.
For a failed run, retry it from Actions. Manual dispatch accepts an existing
approved tag and an explicit channel, which must agree with its changelog.

## Build and publication

The workflow validates the tagged source and reuses Rust CI to build and test
Linux GNU x86 and Windows MSVC x86, run Clippy and inventory dependency licenses.
It packages only the tagged commit, then publishes with `gh` after those jobs
succeed. A pre-release is marked as such and never selected as the latest stable
release. The remote tag must still point to the validated commit before any
publication changes are made. An unpublished draft is completed with validated
assets and curated changelog notes.

A release already published through GitHub's interface can receive its missing
assets. Its channel must match the changelog. Existing assets must have matching
SHA-256 digests and are never overwritten. Existing notes are preserved; an empty
body is filled from the curated notes. Lookup errors, channel mismatches and
asset conflicts stop publication with explicit diagnostics.

Downloads contain the two binaries, both include naming styles, the Apache
license, dependency inventories and `SHA256SUMS`. Platform archives also contain
documentation, examples and contribution policies. SHA-256 checksums detect
content differences; they are not cryptographic signatures.

Configure branch protection for `master` in the repository settings. The
`DCO and CLA` check verifies sign-off records separately from release
validation. Scorecard runs for public repositories; CodeQL requires code scanning
availability and private repositories opt in through `ENABLE_CODEQL`.
