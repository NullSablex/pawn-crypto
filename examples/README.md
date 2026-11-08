# Pawn examples

All examples use SA-MP's `<a_samp>` standard include and compile with the open.mp
Pawn compiler. Copy the plugin's generated include into your include search path.

| File | Demonstrates |
| --- | --- |
| `01_sha256.pwn` | SHA-224, SHA-256, SHA-384 and SHA-512 through `crypto.inc`. |
| `02_sha256_omp.pwn` | The same digests through the `crypto_omp.inc` naming aliases. |
| `03_hmac.pwn` | HMAC-SHA-256/512 generation and verification. |
| `04_hmac_omp.pwn` | HMAC through PascalCase aliases. |
| `05_argon2id.pwn` | Asynchronous hashing, PHC output and callback-driven verification. |
| `06_argon2id_omp.pwn` | The same asynchronous API through PascalCase aliases. |

The strings and keys are demonstration data. Never print actual passwords or
secret keys. Choose one plugin include per script; the naming style does not
depend on whether the server is SA-MP or open.mp.

Compile them with `cargo xtask pawn`; see [CONTRIBUTING.md](../CONTRIBUTING.md).
