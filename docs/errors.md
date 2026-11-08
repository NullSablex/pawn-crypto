# Errors and troubleshooting

## Native return values

| Call | `true` | `false` |
| --- | --- | --- |
| SHA-2 generation | Digest was written. | Output capacity is too small, or an AMX operation failed. |
| HMAC generation | Tag was written. | Output capacity is too small, initialization failed, or an AMX operation failed. |
| HMAC verification | The full tag matches. | Mismatch, malformed hexadecimal, truncated tag or initialization failure. |
| Argon2id submission | Job was accepted. | Input was refused, callback was unavailable, queue was full or workers were unavailable. |

For SHA-2 and HMAC generation, use the length constant plus one cell for the
terminator. The explicit small-size refusal leaves the array unchanged. An AMX
error is separate from that refusal; inspect server diagnostics as well.
The plugin has no `crypto_errno` or detailed per-call error native.

An Argon2id submission that returns `false` never schedules a callback. Do not
wait for one after refusal. A stored PHC exceeding the input limit or containing
invalid UTF-8 is refused immediately; an accepted but malformed PHC is reported
through the callback.

## Argon2id callback statuses

| Constant | Value | Meaning |
| --- | --- | --- |
| `CRYPTO_PASSWORD_OK` | 0 | Operation completed. A verification can still have `matches = false`. |
| `CRYPTO_PASSWORD_INVALID_HASH` | 1 | Malformed/unsupported PHC or verification costs outside the limits. |
| `CRYPTO_PASSWORD_FAILED` | 2 | Hashing, RNG or worker failure. |

Generation failures supply an empty `hash`. Verification failures supply
`matches = false`. A wrong password has status zero and `matches = false`;
it is not an invalid stored hash.

## Missing native at script load

Confirm that the binary loaded successfully, matches the script's include and
uses the correct architecture. Check the server's component/plugin startup
messages. On native open.mp, use the default build in `components/`; a
`samp-only` build needs legacy loading. See [installation](installation.md).

## A password callback does not arrive

Check the submission return value first. The named public must exist in the
calling script, and its signature must match the operation. The plugin does not
search other AMX instances for the public.

Accepted work requires the server to keep ticking. Unloading the originating
script cancels delivery; a reloaded script will not receive an old job's result.
Callback execution failures are logged and are not retried. Keep callback work
short, and account for the [queue limits](passwords.md#parameters-and-limits).

## A stored password hash is rejected

Supply the full PHC string, not just its final digest. Only Argon2id v19 is
accepted. Check for truncation in database storage and for memory, iteration or
lane costs above the supported verification limits. Changing the string's
parameters changes what is verified; store it intact.

## Digests differ between programs

Compare bytes, not visible text. Encoding, packing, line endings, trailing
spaces and the first null byte affect the input. This API accepts strings, not
arbitrary binary buffers. HMAC also requires the same key bytes on both sides.

## Diagnostics

The SDK logger writes `logs/crypto.log` and prefixes console messages with
`[Crypto]`. Report the plugin commit/version, server version, platform, loading
mode and a minimal reproduction using synthetic data. Never attach real
passwords, secret keys or production password hashes. Vulnerability reporting
is covered by the [repository security policy](https://github.com/NullSablex/pawn-crypto/blob/master/SECURITY.md).
