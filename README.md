# pawn-crypto

> Cryptography plugin for SA-MP and Open Multiplayer, written in Rust — by [NullSablex](https://github.com/NullSablex)

English | [Português (Brasil)](README.pt-BR.md)

[![Rust CI](https://github.com/NullSablex/pawn-crypto/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/rust.yml)
[![Pawn sources](https://github.com/NullSablex/pawn-crypto/actions/workflows/encoding.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/encoding.yml)
[![Dependency audit](https://github.com/NullSablex/pawn-crypto/actions/workflows/audit.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/audit.yml)
[![Docs](https://github.com/NullSablex/pawn-crypto/actions/workflows/docs.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/docs.yml)
[![CodeQL](https://github.com/NullSablex/pawn-crypto/actions/workflows/codeql.yml/badge.svg?branch=master)](https://github.com/NullSablex/pawn-crypto/actions/workflows/codeql.yml)
[![DCO](https://github.com/NullSablex/pawn-crypto/actions/workflows/contributions.yml/badge.svg?event=pull_request)](DCO)
[![CLA](https://img.shields.io/badge/CLA-required-blue)](CLA.md)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/NullSablex/pawn-crypto/badge)](https://scorecard.dev/viewer/?uri=github.com/NullSablex/pawn-crypto)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)
![SA-MP](https://img.shields.io/badge/SA--MP-plugin-orange)
![open.mp](https://img.shields.io/badge/open.mp-native%20%26%20legacy-orange)
![Rust](https://img.shields.io/badge/Rust-2024%20edition-orange)
![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows-green)
![Architecture](https://img.shields.io/badge/arch-x86%20%2832--bit%29-lightgrey)
[![Release](https://img.shields.io/github/v/release/NullSablex/pawn-crypto?label=release)](https://github.com/NullSablex/pawn-crypto/releases/latest)
[![Latest including pre-releases](https://img.shields.io/github/v/release/NullSablex/pawn-crypto?include_prereleases&label=latest%20%28incl.%20pre%29)](https://github.com/NullSablex/pawn-crypto/releases)
[![Downloads](https://img.shields.io/github/downloads/NullSablex/pawn-crypto/total)](https://github.com/NullSablex/pawn-crypto/releases)
[![Stars](https://img.shields.io/github/stars/NullSablex/pawn-crypto)](https://github.com/NullSablex/pawn-crypto/stargazers)
[![Issues](https://img.shields.io/github/issues/NullSablex/pawn-crypto)](https://github.com/NullSablex/pawn-crypto/issues)
[![Last commit](https://img.shields.io/github/last-commit/NullSablex/pawn-crypto/master)](https://github.com/NullSablex/pawn-crypto/commits/master)

## Overview

**pawn-crypto** provides SHA-2 hashing, HMAC authentication and asynchronous
Argon2id password hashing for SA-MP and Open Multiplayer. Built with
[rust-samp](https://github.com/NullSablex/rust-samp) and RustCrypto.

The Pawn include is **`crypto.inc`**. The same default binary exposes the SA-MP
plugin interface and the native open.mp component interface.

### Features

| Operation | API | Execution |
| --- | --- | --- |
| SHA-224, SHA-256, SHA-384, SHA-512 | `crypto_sha224` / `256` / `384` / `512` | Synchronous, lowercase hexadecimal output. |
| HMAC-SHA-256, HMAC-SHA-512 | `crypto_hmac_sha256` / `512` and `_verify` | Synchronous, full tags and constant-time verification. |
| Argon2id | `crypto_argon2id_hash`, `crypto_argon2id_verify` | Bounded workers, random salts, PHC strings and Pawn callbacks. |

The API accepts packed or unpacked Pawn strings. Message, key and password bytes
are preserved up to the first null byte without text decoding or normalization.
HMAC and Argon2id are distinct operations: use the
[password API](docs/passwords.md) for password storage.

## Installation

1. Download `pawn_crypto.so` (Linux) or `pawn_crypto.dll` (Windows) and
   `crypto.inc` from [Releases](https://github.com/NullSablex/pawn-crypto/releases),
   or [build from source](docs/building.md).
2. Copy `crypto.inc` into your Pawn include directory, such as `pawno/include/`
   or `qawno/include/`.
3. Load the binary using one mode:
   - **SA-MP:** place it in `plugins/` and add `pawn_crypto.so` on Linux or
     `pawn_crypto` on Windows to the `plugins` line in `server.cfg`.
   - **open.mp native:** place it in `components/`; the server loads it automatically.
   - **open.mp legacy:** place it in `plugins/` and add `pawn_crypto` to
     `pawn.legacy_plugins` in `config.json`.

See [installation](docs/installation.md) for the configuration examples.

## Quick start

```pawn
#include <a_samp>
#include <crypto>

public OnFilterScriptInit()
{
    new digest[CRYPTO_SHA256_LENGTH + 1];
    if (crypto_sha256("abc", digest))
    {
        printf("SHA-256: %s", digest);
    }
    return 1;
}
```

For PascalCase, install and include `crypto_omp.inc` instead and call
`Crypto_Sha256`. It provides Pawn aliases to the same natives. Choose one naming
include per script; either style works on SA-MP and open.mp.

## Documentation

Read the [online documentation](https://nullsablex.github.io/pawn-crypto/),
or browse the [Markdown overview](docs/index.md) on GitHub.

| Guide | Contents |
| --- | --- |
| [Installation](docs/installation.md) | Binary placement and server configuration. |
| [SHA-2](docs/hashing.md) | Data digests and output buffers. |
| [HMAC](docs/hmac.md) | Message authentication and tag verification. |
| [Argon2id](docs/passwords.md) | Password storage, callbacks and resource limits. |
| [API reference](docs/api-reference.md) | Native signatures, aliases, constants and return values. |
| [Errors](docs/errors.md) | Submission failures, callback results and troubleshooting. |
| [Security](docs/security.md) | Data handling, secrets and application responsibilities. |
| [Building](docs/building.md) | Toolchain, build scripts and validation commands. |
| [Dependency licenses](docs/licenses.md) | cargo-about inventories and third-party notices. |

## Examples

The [examples](examples/GUIDE.md) cover SHA-2, HMAC and asynchronous Argon2id in
both naming styles. Each `.pwn` file is a runnable filterscript.

## Building from source

Use the Rust toolchain pinned in [rust-toolchain.toml](rust-toolchain.toml).
The build scripts produce `dist/pawn_crypto.so` and `dist/pawn_crypto.dll`:

```sh
./scripts/build-linux.sh
./scripts/build-windows.sh
```

See [the build guide](docs/building.md) for platform requirements and
cross-compilation setup.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development and review requirements.
Each contributor's `Signed-off-by` certifies the [DCO](DCO) and accepts the
[CLA](CLA.md) under the contribution policy. Participation follows the
[Code of Conduct](CODE_OF_CONDUCT.md) and [AI usage policy](AI-POLICY.md).

Report vulnerabilities privately as described in [SECURITY.md](SECURITY.md).
Changes are recorded in [CHANGELOG.md](CHANGELOG.md).

## License

Copyright (c) 2026 NullSablex

Licensed under the [Apache License, Version 2.0](LICENSE). Third-party
dependencies retain their own licenses and notices; see the
[dependency license guide](docs/licenses.md).
