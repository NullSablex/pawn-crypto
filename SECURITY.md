# Security policy

## Reporting a vulnerability

Do not disclose a vulnerability in a public issue. Use a private
[GitHub security advisory](https://github.com/NullSablex/pawn-crypto/security/advisories/new)
when private reporting is available. Otherwise, request a private reporting
channel from [the maintainer](https://github.com/NullSablex) without sharing
exploit details publicly.

Include a minimal reproduction with synthetic data, the affected commit or
release, operating system, server version and loading mode. Never send real
passwords, secret keys, production PHC strings or unrelated server credentials.

## Scope

This policy covers the Rust implementation, native bindings, generated Pawn
includes, build tooling and examples. Relevant findings include incorrect hash
or MAC results, authentication bypasses, secret exposure, unsafe AMX access,
callbacks delivered to the wrong or unloaded script, unbounded resource use and
worker or library lifetime errors.

Report defects in dependencies too when they affect this plugin. CI audits
`Cargo.lock`, and cargo-about inventories the dependency licenses. A successful
audit or test suite does not replace review of cryptographic API changes.

## Supported versions

The initial pre-release is being prepared on `master`; no release has been
published yet. Security fixes currently target `master`. Pre-release APIs and
behavior may change. Once releases are published, support will be stated in
their release notes.

## Operational behavior

SHA-2 and HMAC operations are synchronous and accept null-terminated Pawn strings.
Argon2id runs on a bounded worker pool and returns to the calling AMX through
rust-samp's main-thread queue. Input and verification-cost limits are documented
in the include and [password guide](docs/passwords.md). The plugin clears its
owned temporary password and key copies; the caller remains responsible for
its Pawn arrays and storage.
