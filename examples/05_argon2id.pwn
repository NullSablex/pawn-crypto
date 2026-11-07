#include <a_samp>
#include <crypto>

forward OnPasswordHashed(status, const hash[], context);
forward OnPasswordVerified(status, bool:matches, context);

public OnFilterScriptInit()
{
    // Demonstration input only. Never print passwords or stored password hashes.
    if (!crypto_argon2id_hash("example password", "OnPasswordHashed", 42))
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
    if (!crypto_argon2id_verify("example password", hash, "OnPasswordVerified", context))
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
