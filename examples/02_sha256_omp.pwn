// Naming style does not depend on the server's standard library.
#include <a_samp>
#include <crypto_omp>

public OnFilterScriptInit()
{
    new digest[CRYPTO_SHA512_LENGTH + 1];

    if (!Crypto_Sha256("abc", digest))
    {
        print("[pawn-crypto] SHA-256 failed.");
        return 1;
    }

    printf("[pawn-crypto] SHA-256: %s", digest);
    if (Crypto_Sha224("abc", digest))
    {
        printf("[pawn-crypto] SHA-224: %s", digest);
    }
    if (Crypto_Sha384("abc", digest))
    {
        printf("[pawn-crypto] SHA-384: %s", digest);
    }
    if (Crypto_Sha512("abc", digest))
    {
        printf("[pawn-crypto] SHA-512: %s", digest);
    }
    return 1;
}
