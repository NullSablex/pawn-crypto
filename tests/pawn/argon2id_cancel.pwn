#include <a_samp>
#include <crypto>

forward OnCancelledArgon(status, bool:matches, context);
forward ArgonCancellationArmed();

public OnFilterScriptInit()
{
    if (CallRemoteFunction("ArgonCancellationPhase", "") == 1)
    {
        if (!crypto_argon2id_verify("test", "invalid", "OnCancelledArgon", 999))
            print("ARGON_TEST_FAILED: cancellation job was not queued");
    }
    return 1;
}

public OnCancelledArgon(status, bool:matches, context)
{
    #pragma unused status, matches, context
    print("ARGON_TEST_FAILED: stale callback reached a reloaded script");
    return 1;
}

public ArgonCancellationArmed() { return 1; }
