# Argon2id passwords

Argon2id hashes passwords into self-contained PHC strings. The PHC value contains
the algorithm, version, costs, salt and derived hash. Store the complete string
unchanged and pass it back to the verification native on login.

Both operations run on a worker pool. A native returning `true` means the job
was accepted, not that hashing or verification has completed. A `false` return
produces no callback.

## Hash, then verify

```pawn
#include <a_samp>
#include <crypto>

forward OnPasswordHashed(status, const hash[], context);
forward OnPasswordVerified(status, bool:matches, context);

public OnFilterScriptInit()
{
    if (!crypto_argon2id_hash("example password", "OnPasswordHashed", 42))
    {
        print("Password hashing was not queued.");
    }
    return 1;
}

public OnPasswordHashed(status, const hash[], context)
{
    if (status != CRYPTO_PASSWORD_OK)
    {
        printf("Password hashing failed (status %d).", status);
        return 1;
    }
    // Save the complete PHC string to your database. This demo verifies it now.
    if (!crypto_argon2id_verify("example password", hash, "OnPasswordVerified", context))
    {
        print("Password verification was not queued.");
    }
    return 1;
}

public OnPasswordVerified(status, bool:matches, context)
{
    printf("Verification: status=%d, matches=%d, context=%d", status, matches, context);
    return 1;
}
```

The integer `context` is passed unchanged, with a default of zero. Use it to
identify an outstanding request. In a login system, validate the player's
current session before applying a callback result: a reused player ID does not
identify the original connection. Script unload protection is separate from
player-session tracking.

## Results

| Status | Hash callback | Verify callback |
| --- | --- | --- |
| `CRYPTO_PASSWORD_OK` | `hash` contains the PHC string. | `matches` says whether the password is correct. |
| `CRYPTO_PASSWORD_INVALID_HASH` | Not used for generation. | Invalid, unsupported or excessive-cost stored PHC; `matches` is false. |
| `CRYPTO_PASSWORD_FAILED` | Hashing/worker failure; `hash` is empty. | Worker failure; `matches` is false. |

A wrong password is a completed verification: status is
`CRYPTO_PASSWORD_OK`, with `matches = false`. Check both fields.

Use an array of `CRYPTO_ARGON2ID_HASH_SIZE` cells if copying a generated hash
from the callback; this capacity includes its null terminator. Copy callback
strings before returning when you need them later.

## Parameters and limits

Generation currently uses fixed defaults:

| Setting | Value |
| --- | --- |
| Algorithm and version | Argon2id v19. |
| Memory cost | 19456 KiB. |
| Iterations | 2. |
| Lanes | 1. |
| Salt | 16 OS-random bytes, freshly generated per hash. |
| Derived output | 32 bytes, encoded inside the PHC string. |

The API does not expose tuning parameters. Verification reads parameters from
the stored PHC string, so it can verify other Argon2id v19 settings within its
limits. Argon2i, Argon2d, other versions, malformed PHC strings and excessive
costs produce `CRYPTO_PASSWORD_INVALID_HASH`.

| Resource | Limit |
| --- | --- |
| Password input | 1024 bytes. |
| Stored PHC input | 256 bytes. |
| Total pending operations | 128, including queued, running and undelivered results. |
| Workers | Up to 2. |
| Verification memory cost | Up to 65536 KiB. |
| Verification iterations | Up to 10. |
| Verification lanes | Up to 4. |

Overlong input is refused immediately. A bounded queue can also refuse work;
keep application retry behavior bounded. Password bytes are preserved without
text conversion, packed strings work, and an empty password is accepted by the
primitive. Any password policy belongs to the application.

## Callbacks and script lifetime

The named public must exist in the calling AMX when submitting. Results go to
that script only, even if another script defines a public with the same name.
Delivery uses rust-samp's main-thread queue and `exec_public!`, without retaining
a mutable plugin borrow across the callback. Callbacks may submit new work.

Completion order is not guaranteed. Unloading a script invalidates its queued
callbacks; reloading an AMX at the same address does not revive them. Queued work
for that script is skipped; already-running calculations can finish, with their
results discarded. Plugin unload cancels queued jobs, joins active workers and
drops queued deliveries before the library is released.

The SDK drains deliveries with a 2 ms per-tick budget. A single callback is not
interrupted by that budget, so keep callback work short. See
[errors](errors.md) and [the API reference](api-reference.md#crypto_argon2id_hash).
