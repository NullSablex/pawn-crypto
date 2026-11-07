# Changelog

## [Unreleased]

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
- GitHub Actions CI, dependency audits, CodeQL configuration, Dependabot,
  issue and pull request templates, and contribution and security policies.

### Project status

- The project license remains undecided. The dependency inventory is separate
  from the license of pawn-crypto.
- Linux runtime coverage uses open.mp in native and legacy modes. Windows
  execution and a SA-MP server runtime remain to be validated.
