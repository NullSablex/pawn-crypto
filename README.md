# pawn-crypto

Cryptography plugin for SA-MP and Open Multiplayer, built with
[rust-samp](https://github.com/NullSablex/rust-samp).

SHA-2 hashing, HMAC message authentication and asynchronous Argon2id password
hashing, implemented with RustCrypto libraries and rust-samp.

## Supported features

| Feature | Status | Details |
| --- | --- | --- |
| SHA-2 | Available | SHA-224, SHA-256, SHA-384 and SHA-512, with lowercase hexadecimal output. |
| HMAC | Available | HMAC-SHA-256 and HMAC-SHA-512 generation and constant-time tag verification. |
| Argon2id | Available | Asynchronous password hashing and verification with OS-random salts and PHC strings. |
| Pawn strings | Available | Packed and unpacked strings; original bytes up to the first null byte. |
| Output buffers | Available | Capacity checks, null termination and in-place hashing. |
| Pawn naming styles | Available | `crypto.inc` uses snake_case; `crypto_omp.inc` provides PascalCase aliases. |
| Linux x86 | Build verified | 32-bit `.so`; runtime tests passed in open.mp native and legacy modes. |
| Windows x86 MSVC | Build verified | 32-bit `.dll`; runtime testing on Windows is pending. |
| SA-MP plugin interface | Available | Legacy entry points are exported; testing on a SA-MP server is pending. |
| Dependency license inventory | Available | `cargo-about` reports for Linux and Windows. |

All four SHA-2 variants have known-answer tests for empty input, standard
vectors and a million-byte input. HMAC has RFC 4231 vectors, including keys
longer than a hash block. Tests cover packed strings, non-UTF-8 bytes, insufficient
buffers, zero capacity, in-place output, wrong messages and keys, tampered tags,
truncated tags and invalid hexadecimal. Verification accepts either hex case.

| Algorithm | Generation native | Hex characters | Verification native |
| --- | --- | --- | --- |
| SHA-224 | `crypto_sha224` | 56 | - |
| SHA-256 | `crypto_sha256` | 64 | - |
| SHA-384 | `crypto_sha384` | 96 | - |
| SHA-512 | `crypto_sha512` | 128 | - |
| HMAC-SHA-256 | `crypto_hmac_sha256` | 64 | `crypto_hmac_sha256_verify` |
| HMAC-SHA-512 | `crypto_hmac_sha512` | 128 | `crypto_hmac_sha512_verify` |

Output arrays need one extra cell for the null terminator. The includes expose
`CRYPTO_SHA224_LENGTH`, `CRYPTO_SHA256_LENGTH`, `CRYPTO_SHA384_LENGTH`,
`CRYPTO_SHA512_LENGTH`, `CRYPTO_HMAC_SHA256_LENGTH` and `CRYPTO_HMAC_SHA512_LENGTH`.

## Planned features

These are the initial roadmap candidates. They are not implemented yet;
their APIs, dependencies and licenses will be reviewed before addition.

| Feature | Intended use |
| --- | --- |
| Cryptographically secure random bytes | Keys, salts, nonces and tokens. |
| Authenticated encryption | Encryption and decryption with integrity verification; algorithm selection pending. |
| Binary buffers and encoding helpers | Explicit-length data, including null bytes, with hexadecimal and Base64 conversion. |

## Build

Rust 1.99.1 is pinned in `rust-toolchain.toml` and required by `Cargo.toml`.

```sh
./scripts/build-linux.sh                # Linux + Windows, from Linux
./scripts/build-windows.sh              # Windows + Linux, from Git Bash
PROFILE=dev ./scripts/build-linux.sh    # Development build
```

The scripts produce `dist/pawn_crypto.so` and `dist/pawn_crypto.dll`. Both are
32-bit binaries with SA-MP and native open.mp entry points. Linux builds need
32-bit development libraries (`gcc-multilib` on Debian/Ubuntu), Rust, `jq`, `rg`
and `cargo-about`.
The Linux script uses `cargo-xwin` for Windows; the Windows script uses MSVC
and either WSL or Docker with `cross` for Linux. Missing Rust targets and Cargo
cross-compilation tools are installed by the scripts.

## Install and use

Copy `include/crypto.inc` to the Pawn compiler's include directory. For SA-MP,
put the binary in `plugins/` and append `pawn_crypto.so` (Linux) or `pawn_crypto`
(Windows) to the `plugins` line in `server.cfg`. On open.mp, the default build
can instead be loaded directly from `components/`.

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

`crypto_sha256(const input[], output[], size = sizeof(output))` writes 64
lowercase hexadecimal characters. The output needs 65 cells, including the
null terminator. A smaller size returns `false` without modifying the output.
Packed and unpacked Pawn strings are hashed as their original bytes up to the
first null byte, without encoding conversion. This operation is synchronous;
SHA-256 is for data hashing, not password storage.

HMAC generation takes `(input, key, output, size = sizeof(output))`; verification
takes `(input, key, expected)` and returns `false` for a mismatch, malformed
hexadecimal or a truncated tag. Messages and keys use the original bytes up to
the first null byte. Empty keys are accepted by HMAC, though applications should
use secret keys generated securely. The plugin wipes its temporary key copy
and enables the SHA-2 and HMAC libraries' zeroization features; the caller owns
the Pawn key array.
Use the verification natives to compare tags. Runnable examples for both naming
styles are in `examples/03_hmac.pwn` and `examples/04_hmac_omp.pwn`.

Alternatively, `crypto_omp.inc` exposes `Crypto_Sha256` as a Pawn alias to
that native. Choose one naming style per script. Both includes work with
SA-MP and open.mp; neither requires the open.mp standard library. The open.mp
Pawn compiler can compile the examples using SA-MP's standard includes.

## Argon2id passwords

`crypto_argon2id_hash(password, callback, context = 0)` queues password hashing.
Its callback is `public Callback(status, const hash[], context)`. Store the full
PHC string unchanged; `CRYPTO_ARGON2ID_HASH_SIZE` is a suitable buffer capacity,
including the null terminator. Every hash uses a fresh 16-byte OS-random salt,
Argon2id v19, 19456 KiB of memory, two iterations, one lane and a 32-byte output.

`crypto_argon2id_verify(password, hash, callback, context = 0)` queues verification.
Its callback is `public Callback(status, bool:matches, context)`:

| Status | Meaning |
| --- | --- |
| `CRYPTO_PASSWORD_OK` | Hash generated, or verification completed; inspect `matches` for verification. |
| `CRYPTO_PASSWORD_INVALID_HASH` | Malformed PHC, unsupported algorithm/version, or excessive verification costs. |
| `CRYPTO_PASSWORD_FAILED` | Worker or hashing failure. |

A wrong password returns `CRYPTO_PASSWORD_OK` with `matches = false`. Failed
hash operations supply an empty hash string. A native returning `true` means
accepted into the queue. A `false` return produces no callback; check it before
waiting for a result. Callbacks must already exist in the calling AMX and may
submit new operations. Packed and unpacked passwords retain their original
bytes, including non-UTF-8 bytes, up to the first null.

The pool uses at most two workers and accepts at most 128 total queued, running
and undelivered operations. Passwords are limited to 1024 bytes and stored PHC
strings to 256 bytes. Verification reads parameters from the PHC string, with
maximums of 65536 KiB, ten iterations and four lanes. Completion order can vary.
The plugin clears its temporary password copies and enables Argon2 zeroization.

Results use rust-samp's main-thread queue and `exec_public!`, with a 2 ms queue
drain budget per tick. They go only to the calling script; unloading it cancels
its callbacks, including when a new AMX reuses its address. Unloading the plugin
cancels queued work and joins active workers before the library is released.
Examples: `examples/05_argon2id.pwn` and `examples/06_argon2id_omp.pwn`.

## Layout

- `src/lib.rs`: module declarations and native registration.
- `src/plugin.rs`: plugin state and lifecycle.
- `src/hash.rs`: hashing operations and known-answer tests.
- `src/hmac.rs`: keyed authentication and constant-time verification.
- `src/encoding.rs`: internal hexadecimal conversion for digests and tags.
- `src/password.rs`: Argon2id workers, bounded submission and SDK callback delivery.
- `src/logger.rs`: SDK logging and startup banner.
- `src/natives/`: native implementations grouped by domain.
- `include/crypto.inc.in`: source template; `build.rs` generates both includes.
- `scripts/`: Linux and Windows build entry points.
- `examples/`: Pawn examples using each naming style.

Edit the include template rather than generated files. `Cargo.toml` is the
single source of the plugin version; the build injects it into the includes.

```sh
cargo fmt --all -- --check
cargo test --locked --target i686-unknown-linux-gnu --lib
cargo clippy --locked --target i686-unknown-linux-gnu --all-targets -- -D warnings
python3 .github/scripts/check_pawn_encoding.py
```

Compile the examples and run the isolated Pawn runtime fixtures with
`scripts/check-pawn.py`; see [CONTRIBUTING.md](CONTRIBUTING.md) for the commands.

## Project information

- [Changelog](CHANGELOG.md): implemented changes and current project status.
- [Contributing](CONTRIBUTING.md): development, validation and commit conventions.
- [Security policy](SECURITY.md): private vulnerability reporting and scope.
- [Code of conduct](CODE_OF_CONDUCT.md): participation expectations.
- [Pawn examples](examples/README.md): usage by operation and naming style.

CI builds and tests Linux GNU and Windows MSVC, checks both plugin configurations,
validates generated includes and produces dependency license artifacts. RustSec
audits run on changes and weekly. CodeQL is configured for Rust and GitHub Actions;
private repositories opt in after code scanning becomes available. This setup
does not publish tags or releases automatically.

## Dependency licenses

The project license is undecided. Generate the dependency inventory with:

```sh
./scripts/licenses.sh
```

This uses `cargo-about` and the lockfile to write `dist/licenses.json` and
`dist/THIRD-PARTY-LICENSES.html` for both supported targets. Build scripts also
generate these reports. Build, development and transitive dependencies are
included; only the unpublished root package is excluded while its license
remains undecided. `about.toml` selects among dependency license alternatives
and fails on unaccepted licenses; it does not license this project.
The offline inventory may contain synthesized standard texts where packages
omit license files. Original copyright notices still need review before
using the report for redistribution.
