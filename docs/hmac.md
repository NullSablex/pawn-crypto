# HMAC authentication

HMAC authenticates a message with a shared secret key. Both parties need the
same message bytes and key bytes. It does not encrypt the message and is not
password storage; use [Argon2id](passwords.md) for passwords.

| Algorithm | Generate | Verify | Tag size |
| --- | --- | --- | --- |
| HMAC-SHA-256 | `crypto_hmac_sha256` | `crypto_hmac_sha256_verify` | 64 hexadecimal characters. |
| HMAC-SHA-512 | `crypto_hmac_sha512` | `crypto_hmac_sha512_verify` | 128 hexadecimal characters. |

## Generate and verify

```pawn
#include <a_samp>
#include <crypto>

public OnFilterScriptInit()
{
    // Demonstration key only; obtain real secret keys from server configuration.
    new tag[CRYPTO_HMAC_SHA256_LENGTH + 1];
    if (!crypto_hmac_sha256("message", "example-key", tag))
    {
        print("HMAC generation failed.");
        return 1;
    }
    if (crypto_hmac_sha256_verify("message", "example-key", tag))
    {
        print("Message authentication succeeded.");
    }
    return 1;
}
```

Generation writes lowercase hexadecimal and returns `false` when the output
capacity is insufficient. The array needs the corresponding
`CRYPTO_HMAC_SHA256_LENGTH + 1` or `CRYPTO_HMAC_SHA512_LENGTH + 1` cells.

Verification accepts a full tag in either hexadecimal case. It decodes the tag
and uses the cryptographic library's constant-time byte comparison. Use the
verification native to compare tags. Invalid hexadecimal, a truncated tag,
a different message or a different key all return `false`.

## Key and message handling

Both strings are passed as original bytes up to the first null. Packed strings
are supported. Keys with embedded null bytes cannot be represented by this API;
explicit-length binary buffers are planned.

Empty keys are accepted by the underlying HMAC construction. Applications
should provision suitable secret keys rather than use demonstration or empty
keys. There is no exposed key-generation native yet.

The plugin clears its temporary key copy and enables zeroization features in
its SHA-2 and HMAC dependencies. The caller still owns the original Pawn array.
These calls are synchronous. See [security and data handling](security.md) and
the [native reference](api-reference.md#crypto_hmac_sha256).
