# Changelog

## [0.1.0] - Pre-release

### Added

- A Rust plugin for SA-MP and open.mp, with 32-bit Linux GNU and Windows MSVC builds.
- SHA-224, SHA-256, SHA-384 and SHA-512 hashing over original Pawn string bytes.
- HMAC-SHA-256 and HMAC-SHA-512 generation and constant-time verification.
- Asynchronous Argon2id password hashing and verification, with random salts,
  PHC output, bounded workers, script-specific callbacks and unload cancellation.
- Generated `crypto.inc` and `crypto_omp.inc` includes, with snake_case natives
  and PascalCase aliases derived from one template.
- Runnable Pawn examples, known-answer tests and isolated open.mp runtime fixtures.
- Linux and Windows build scripts, a pinned Rust toolchain and cargo-about inventories.
- Rust maintenance tooling through `cargo xtask` for Pawn source checks,
  API reference generation, documentation checks and isolated runtime tests.
- GitHub Actions CI, dependency audits, CodeQL configuration, Dependabot,
  issue and pull request templates, and contribution, AI usage and security policies.
- Release and pre-release automation with version and changelog validation,
  platform archives, dependency notices and SHA-256 checksums; Release Drafter
  and OpenSSF Scorecard workflows.
- Apache-2.0 project licensing, DCO 1.1 sign-offs, a contributor license
  agreement accepted through sign-offs, and automated pull request checks.
- README and Zensical documentation covering installation, operations,
  asynchronous behavior, errors, security, builds and dependency licensing,
  with the native API reference generated from the documented Pawn includes.
