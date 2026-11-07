/*
 * crypto - open.mp naming style (standalone).
 *
 * GENERATED from crypto.inc by build.rs - do not edit by hand.
 *
 * Include THIS instead of <crypto> to write the API in
 * open.mp's Prefix_PascalCase style (Crypto_Sha256).
 * Each native aliases the real one, so there is no
 * runtime cost and nothing changes on the plugin side. This file is
 * self-contained - do not include it together with <crypto>.
 */

#if defined _crypto_omp_included
    #endinput
#endif
#define _crypto_omp_included


#pragma library pawn_crypto

#define CRYPTO_VERSION "0.1.0"

// ---------------------------------------------------------------------------
// Hashing
// ---------------------------------------------------------------------------

#define CRYPTO_SHA224_LENGTH (56)
#define CRYPTO_SHA256_LENGTH (64)
#define CRYPTO_SHA384_LENGTH (96)
#define CRYPTO_SHA512_LENGTH (128)

/**
 * Hashes a Pawn string with SHA-224; uses original bytes up to the first null.
 * Packed and unpacked strings are supported. This call is synchronous.
 *
 * @param input Null-terminated string to hash.
 * @param output Destination for the 56-character lowercase hexadecimal digest.
 * @param size Output capacity in cells; at least CRYPTO_SHA224_LENGTH + 1.
 * @return true on success; false if size is too small, leaving output unchanged.
 */
native bool:Crypto_Sha224(const input[], output[], size = sizeof(output)) = crypto_sha224;
/**
 * Hashes a Pawn string with SHA-256 and writes lowercase hexadecimal output.
 * Input bytes are used directly, without text encoding conversion. Packed and
 * unpacked strings are supported; input ends at the first null byte.
 * This is a synchronous hash for data, not a password hashing function.
 *
 * @param input Null-terminated string to hash.
 * @param output Destination for the 64-character lowercase hexadecimal digest.
 * @param size Output capacity in cells; at least CRYPTO_SHA256_LENGTH + 1.
 * @return true on success; false if size is too small, leaving output unchanged.
 */
native bool:Crypto_Sha256(const input[], output[], size = sizeof(output)) = crypto_sha256;
/**
 * Hashes a Pawn string with SHA-384; uses original bytes up to the first null.
 * Packed and unpacked strings are supported. This call is synchronous.
 *
 * @param input Null-terminated string to hash.
 * @param output Destination for the 96-character lowercase hexadecimal digest.
 * @param size Output capacity in cells; at least CRYPTO_SHA384_LENGTH + 1.
 * @return true on success; false if size is too small, leaving output unchanged.
 */
native bool:Crypto_Sha384(const input[], output[], size = sizeof(output)) = crypto_sha384;
/**
 * Hashes a Pawn string with SHA-512; uses original bytes up to the first null.
 * Packed and unpacked strings are supported. This call is synchronous.
 *
 * @param input Null-terminated string to hash.
 * @param output Destination for the 128-character lowercase hexadecimal digest.
 * @param size Output capacity in cells; at least CRYPTO_SHA512_LENGTH + 1.
 * @return true on success; false if size is too small, leaving output unchanged.
 */
native bool:Crypto_Sha512(const input[], output[], size = sizeof(output)) = crypto_sha512;

// ---------------------------------------------------------------------------
// Message authentication
// ---------------------------------------------------------------------------

#define CRYPTO_HMAC_SHA256_LENGTH (CRYPTO_SHA256_LENGTH)
#define CRYPTO_HMAC_SHA512_LENGTH (CRYPTO_SHA512_LENGTH)

// Input and key are packed or unpacked Pawn strings, used as original bytes
// up to the first null. Calls are synchronous. HMAC authenticates data; it
// does not encrypt it or provide password hashing. Use the verify natives
// instead of comparing tags with strcmp.

/**
 * Generates a full HMAC-SHA-256 tag in lowercase hexadecimal.
 *
 * @param input Null-terminated message to authenticate.
 * @param key Null-terminated secret key; its bytes are used without decoding.
 * @param output Destination for the 64-character hexadecimal tag.
 * @param size Output capacity in cells; at least CRYPTO_HMAC_SHA256_LENGTH + 1.
 * @return true on success; false on failure, leaving output unchanged.
 */
native bool:Crypto_HmacSha256(const input[], const key[], output[], size = sizeof(output)) = crypto_hmac_sha256;

/**
 * Verifies a full HMAC-SHA-256 tag using a constant-time byte comparison.
 * Hexadecimal input may use either case. Truncated tags are not accepted.
 *
 * @param input Null-terminated message to authenticate.
 * @param key Null-terminated secret key, in its original byte encoding.
 * @param expected Full 64-character hexadecimal tag to verify.
 * @return true for a valid tag; false for a mismatch or malformed tag.
 */
native bool:Crypto_HmacSha256Verify(const input[], const key[], const expected[]) = crypto_hmac_sha256_verify;

/**
 * Generates a full HMAC-SHA-512 tag in lowercase hexadecimal.
 *
 * @param input Null-terminated message to authenticate.
 * @param key Null-terminated secret key; its bytes are used without decoding.
 * @param output Destination for the 128-character hexadecimal tag.
 * @param size Output capacity in cells; at least CRYPTO_HMAC_SHA512_LENGTH + 1.
 * @return true on success; false on failure, leaving output unchanged.
 */
native bool:Crypto_HmacSha512(const input[], const key[], output[], size = sizeof(output)) = crypto_hmac_sha512;

/**
 * Verifies a full HMAC-SHA-512 tag using a constant-time byte comparison.
 * Hexadecimal input may use either case. Truncated tags are not accepted.
 *
 * @param input Null-terminated message to authenticate.
 * @param key Null-terminated secret key, in its original byte encoding.
 * @param expected Full 128-character hexadecimal tag to verify.
 * @return true for a valid tag; false for a mismatch or malformed tag.
 */
native bool:Crypto_HmacSha512Verify(const input[], const key[], const expected[]) = crypto_hmac_sha512_verify;

// ---------------------------------------------------------------------------
// Password hashing (asynchronous)
// ---------------------------------------------------------------------------

#define CRYPTO_ARGON2ID_HASH_SIZE (128)

enum
{
    CRYPTO_PASSWORD_OK = 0,
    CRYPTO_PASSWORD_INVALID_HASH,
    CRYPTO_PASSWORD_FAILED
};

// Passwords are original bytes up to the first null; packed strings work too.
// A true native return means queued, not completed. A false return never
// produces a callback. Results return to the calling AMX only. Work for an
// unloaded script is cancelled; queued results cannot target a reloaded AMX.
// Limits: 1024 password bytes, 256 PHC bytes, 128 total pending operations,
// and up to 2 workers. Completion order is not guaranteed.

/**
 * Generates an Argon2id v19 PHC string with a fresh OS-random salt.
 * Defaults: m=19456 KiB, t=2, p=1; 16-byte salt and 32-byte output.
 * Callback signature: public Callback(status, const hash[], context).
 * On failure, status is CRYPTO_PASSWORD_FAILED and hash is empty.
 *
 * @param password Password string, never decoded or logged by this plugin.
 * @param callback Name of an existing public in the calling script.
 * @param context Integer passed unchanged to the callback.
 * @return true if queued; false for invalid input, missing callback or full queue.
 */
native bool:Crypto_Argon2idHash(const password[], const callback[], context = 0) = crypto_argon2id_hash;

/**
 * Verifies a password against an Argon2id v19 PHC string on a worker.
 * Callback signature: public Callback(status, bool:matches, context).
 * A wrong password returns CRYPTO_PASSWORD_OK with matches=false.
 * Malformed, unsupported or excessive-cost hashes return
 * CRYPTO_PASSWORD_INVALID_HASH with matches=false. Accepted maximums:
 * m=65536 KiB, t=10, p=4. Other worker failures use CRYPTO_PASSWORD_FAILED.
 *
 * @param password Password string in its original byte encoding.
 * @param hash Complete stored PHC string, including its salt and parameters.
 * @param callback Name of an existing public in the calling script.
 * @param context Integer passed unchanged to the callback.
 * @return true if queued; false for invalid input, missing callback or full queue.
 */
native bool:Crypto_Argon2idVerify(const password[], const hash[], const callback[], context = 0) = crypto_argon2id_verify;
