#include <a_samp>
#include <crypto>

public OnFilterScriptInit()
{
    new digest[CRYPTO_SHA512_LENGTH + 1];

    if (!crypto_sha256("abc", digest))
    {
        print("[pawn-crypto] SHA-256 failed.");
        return 1;
    }

    printf("[pawn-crypto] SHA-256: %s", digest);
    if (crypto_sha224("abc", digest))
    {
        printf("[pawn-crypto] SHA-224: %s", digest);
    }
    if (crypto_sha384("abc", digest))
    {
        printf("[pawn-crypto] SHA-384: %s", digest);
    }
    if (crypto_sha512("abc", digest))
    {
        printf("[pawn-crypto] SHA-512: %s", digest);
    }
    return 1;
}
