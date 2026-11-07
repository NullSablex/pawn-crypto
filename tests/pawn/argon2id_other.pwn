#include <a_samp>
#include <crypto>

new bool:Finished;

forward OnOwnerHash(status, const hash[], context);
forward ArgonOtherDone();

public OnFilterScriptInit()
{
    if (!crypto_argon2id_hash("owned by filterscript", "OnOwnerHash", 456))
        print("ARGON_TEST_FAILED: filterscript submission");
    return 1;
}

public OnOwnerHash(status, const hash[], context)
{
    if (status != CRYPTO_PASSWORD_OK || context != 456 || !hash[0])
        print("ARGON_TEST_FAILED: callback crossed AMX ownership");
    else print("ARGON_OWNER_OK");
    Finished = true;
    return 1;
}

public ArgonOtherDone()
{
    return Finished;
}
