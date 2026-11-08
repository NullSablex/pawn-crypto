# Security and data handling

## What each API provides

SHA-2 produces data digests. HMAC authenticates a message using a shared secret.
Argon2id is the password hashing API. Encryption and decryption are not currently
implemented. These interfaces do not define an application's authentication,
message framing, replay protection or password policy.

Use the HMAC verification natives for tag comparison. They accept only full,
valid hexadecimal tags and use the library's constant-time byte comparison.
The public tag's syntax and length are checked before comparison.

## String semantics

Message, key and password inputs support Pawn's packed and unpacked representations.
The plugin reads these original bytes without Unicode decoding, normalization
or code-page conversion. Strings end at the first null byte. This matters when
matching data from programs that use a different text encoding.

There is no explicit-length binary input API yet. A key or message containing
embedded null bytes cannot be passed intact through these string natives.

SHA-2 and HMAC output lowercase hexadecimal. Verification accepts either case.
The output capacity includes the terminator and must describe the actual array.
The plugin cannot infer a Pawn array's declared size from its address alone.

## Secrets and ownership

Passwords and HMAC keys are copied out of AMX memory before work uses them.
Temporary owned password/key copies are zeroized on drop, and the cryptographic
libraries' zeroization features are enabled. The caller remains responsible
for its Pawn arrays, database fields, configuration and backups. This does not
clear copies made by the application or guarantee removal from every system
buffer or crash dump.

The plugin does not log passwords or secret keys. Do not add such logging in
callbacks, error reports or examples. Treat stored PHC strings as authentication
data and keep them out of public diagnostics too.

## Execution and resource use

SHA-2 and HMAC are synchronous. Large input still occupies the server thread;
there is no streaming or asynchronous data-hash API.

Argon2id has a bounded worker pool, an overall pending-operation limit and
limits on password length and verification costs. These bounds do not replace
application login rate limiting. A failed submission produces no callback.
See the exact [password limits](passwords.md#parameters-and-limits).

Callbacks run on the server thread through rust-samp. Results stay bound to the
originating script generation and are cancelled when that AMX unloads. Player
IDs can still be reused within the same AMX: correlate callbacks with the current
application request/session before applying authentication results.

## Dependencies and reports

The cryptographic primitives come from RustCrypto libraries. `Cargo.lock` fixes
the dependency versions; CI runs a RustSec audit, and cargo-about generates the
[license inventory](licenses.md). A clean scan and passing functional tests are
not a formal verification of side-channel behavior or an application protocol.

Report a suspected vulnerability through the
[security policy](https://github.com/NullSablex/pawn-crypto/blob/master/SECURITY.md).
