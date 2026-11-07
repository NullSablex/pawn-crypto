#include <a_samp>
#include <crypto_omp>

public OnFilterScriptInit()
{
    // Test key only. Obtain real secret keys from your server configuration.
    new tag[CRYPTO_HMAC_SHA512_LENGTH + 1];
    if (!Crypto_HmacSha256("message", "example-key", tag))
    {
        print("[pawn-crypto] HMAC-SHA-256 generation failed.");
        return 1;
    }
    if (!Crypto_HmacSha256Verify("message", "example-key", tag))
    {
        print("[pawn-crypto] HMAC-SHA-256 verification failed.");
        return 1;
    }
    if (!Crypto_HmacSha512("message", "example-key", tag))
    {
        print("[pawn-crypto] HMAC-SHA-512 generation failed.");
        return 1;
    }
    if (!Crypto_HmacSha512Verify("message", "example-key", tag))
    {
        print("[pawn-crypto] HMAC-SHA-512 verification failed.");
        return 1;
    }
    print("[pawn-crypto] HMAC verification succeeded.");
    return 1;
}
