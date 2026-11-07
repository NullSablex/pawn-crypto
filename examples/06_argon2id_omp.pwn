#include <a_samp>
#include <crypto_omp>

forward OnPasswordHashed(status, const hash[], context);
forward OnPasswordVerified(status, bool:matches, context);

public OnFilterScriptInit()
{
    // Demonstration input only. Never print passwords or stored password hashes.
    if (!Crypto_Argon2idHash("example password", "OnPasswordHashed", 42))
    {
        print("[pawn-crypto] Password hash was not queued.");
    }
    return 1;
}

public OnPasswordHashed(status, const hash[], context)
{
    if (status != CRYPTO_PASSWORD_OK)
    {
        printf("[pawn-crypto] Password hashing failed (status %d).", status);
        return 1;
    }
    // Save the complete PHC string to your database, then verify on login.
    // This example immediately verifies to demonstrate callback re-entry.
    if (!Crypto_Argon2idVerify("example password", hash, "OnPasswordVerified", context))
    {
        print("[pawn-crypto] Password verification was not queued.");
    }
    return 1;
}

public OnPasswordVerified(status, bool:matches, context)
{
    printf("[pawn-crypto] Verification: status=%d, matches=%d, context=%d", status, matches, context);
    return 1;
}
