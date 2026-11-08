# pawn-crypto

pawn-crypto provides cryptographic primitives to Pawn scripts on SA-MP and
open.mp. Its default binary works as a legacy plugin or a native open.mp
component. The API uses `crypto.inc`; `crypto_omp.inc` exposes PascalCase aliases.

The project is in pre-release: the API and behavior may change before a stable
release. Downloads are on the
[releases page](https://github.com/NullSablex/pawn-crypto/releases); the
[changelog](https://github.com/NullSablex/pawn-crypto/blob/master/CHANGELOG.md)
lists what each version contains.

## Choose an operation

| Task | Use | Result |
| --- | --- | --- |
| Compute a data digest | [SHA-2 hashing](hashing.md) | A lowercase hexadecimal string. |
| Authenticate a message with a shared secret | [HMAC](hmac.md) | A tag, or a boolean verification result. |
| Store and verify passwords | [Argon2id](passwords.md) | An asynchronous PHC string or match result. |

Hashing and HMAC execute during the native call. Argon2id runs on a bounded
worker pool; results return through callbacks on the server thread.

## Get started

Download a release and follow the [installation guide](installation.md), or
[build from source](building.md). The guides contain runnable filterscript
examples, while the [API reference](api-reference.md) lists all native signatures,
parameters, aliases and constants from the generated includes.

For shared behavior and limits, read [data handling](security.md) and
[errors](errors.md). The [dependency license guide](licenses.md) explains the
inventory, Apache-2.0 project licensing and contributor agreements.

## Available and planned

Available: SHA-224, SHA-256, SHA-384, SHA-512, HMAC-SHA-256, HMAC-SHA-512 and
asynchronous Argon2id v19 password hashing and verification.

Not implemented: authenticated encryption, an exposed random-byte native,
explicit-length binary buffers, and hexadecimal/Base64 conversion natives.
Argon2id already obtains salts from the operating system internally.

## Validation scope

Linux x86 and Windows x86 MSVC binaries build. Runtime fixtures have been
exercised on Linux open.mp in native and legacy modes. Windows server execution
and a SA-MP server runtime remain to be validated. Both standard include naming
styles are independent of the server's standard library.
