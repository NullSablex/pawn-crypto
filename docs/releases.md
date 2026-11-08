# Releases and pre-releases

## Version and channel

`Cargo.toml` is the source of the plugin version. A release tag must be exactly
`v` followed by that version, and `CHANGELOG.md` must contain exactly one matching
entry. Generated Pawn includes must match the package version.

Mark a pre-release entry with `## [VERSION] - Pre-release`. Stable entries use
`## [VERSION]` or `## [VERSION] - YYYY/MM/DD`. Replace `VERSION` with the Cargo
version. A SemVer prerelease suffix such as `-rc.1` also requires the pre-release
heading. The initial pre-release uses the existing package version without
adding a suffix; GitHub's pre-release flag identifies its channel.

Published versions and tags are never reused or rewritten. If a pre-release
already used a version without a suffix, choose a new package version for the
stable release. A final changelog describes the result since the previous stable
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
release. Existing published releases are refused; an unpublished draft may be
completed with the validated assets and curated changelog notes.

Downloads contain the two binaries, both include naming styles, the Apache
license, dependency inventories and `SHA256SUMS`. Platform archives also contain
documentation, examples and contribution policies. SHA-256 checksums detect
content differences; they are not cryptographic signatures.

## Release Drafter

Release Drafter updates an unpublished PR-based draft after pushes to `master`.
Labels group features, fixes, documentation and maintenance and suggest a next
version. Suggestions do not update Cargo or authorize publication. Review them
when writing the changelog; the release workflow uses the curated entry rather
than publishing draft text automatically.

Configure branch protection for `master` in the repository settings. The
`DCO and CLA` check verifies sign-off records separately from release
validation. Scorecard runs for public repositories; CodeQL requires code scanning
availability and private repositories opt in through `ENABLE_CODEQL`.
