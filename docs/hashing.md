# SHA-2 hashing

SHA-2 computes a deterministic digest of the input bytes. Use it for data hashes;
use [Argon2id](passwords.md) for password storage. These natives are synchronous.

| Algorithm | Native | Hex characters | Output array capacity |
| --- | --- | --- | --- |
| SHA-224 | `crypto_sha224` | 56 | `CRYPTO_SHA224_LENGTH + 1` |
| SHA-256 | `crypto_sha256` | 64 | `CRYPTO_SHA256_LENGTH + 1` |
| SHA-384 | `crypto_sha384` | 96 | `CRYPTO_SHA384_LENGTH + 1` |
| SHA-512 | `crypto_sha512` | 128 | `CRYPTO_SHA512_LENGTH + 1` |

## Hash a string

```pawn
#include <a_samp>
#include <crypto>

public OnFilterScriptInit()
{
    new digest[CRYPTO_SHA256_LENGTH + 1];
    if (!crypto_sha256("abc", digest))
    {
        print("SHA-256 failed.");
        return 1;
    }
    printf("SHA-256: %s", digest);
    return 1;
}
```

The result is lowercase hexadecimal. `size` defaults to `sizeof(output)` and
includes the null terminator. A smaller capacity returns `false` without
modifying the output; successful calls return `true`.

## Bytes, packing and buffers

Packed and unpacked Pawn strings produce the same digest when they represent
the same bytes. The plugin preserves the bytes and does not decode them as
UTF-8, Windows-1252 or another encoding. Different byte encodings of the same
visible text therefore produce different digests.

Input ends at the first null byte. There is no explicit-length binary hashing
native yet. Empty strings are accepted. Input and output may share the same
array when its capacity is sufficient for the full digest and null terminator.
The supplied `size` must correspond to the actual output array capacity.

Large inputs are processed synchronously, so account for their cost on the
server thread. The [reference](api-reference.md#crypto_sha256) lists the exact
signatures; [data handling](security.md) covers shared behavior.
