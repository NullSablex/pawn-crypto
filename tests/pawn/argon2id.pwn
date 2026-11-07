#include <a_samp>
#include <crypto>

new Failures, Checks, Pending, Phase, Timer;
new bool:Submitting;
new FirstHash[CRYPTO_ARGON2ID_HASH_SIZE];
new CancellationPhase;

forward OnArgonHash(status, const hash[], context);
forward OnArgonVerify(status, bool:matches, context);
forward AdvanceArgonTest();
forward ArgonCancellationPhase();

public ArgonCancellationPhase() { return CancellationPhase; }

Check(bool:condition, const label[])
{
    Checks++;
    if (!condition)
    {
        Failures++;
        printf("ARGON_TEST_FAILED: %s", label);
    }
}

Queued(bool:accepted)
{
    Check(accepted, "operation queued");
    if (accepted) Pending++;
}

main() {}

public OnGameModeInit()
{
    Check(!crypto_argon2id_hash("test", "MissingPublic"), "missing callback refused");
    Check(!crypto_argon2id_hash("test", ""), "empty callback refused");
    Check(!crypto_argon2id_verify("test", "invalid", "MissingPublic"), "missing verify callback refused");
    new longPassword[1026];
    for (new i; i < 1025; i++) longPassword[i] = 'a';
    Check(!crypto_argon2id_hash(longPassword, "OnArgonHash"), "oversized password refused");
    new longHash[258];
    for (new i; i < 257; i++) longHash[i] = 'a';
    Check(!crypto_argon2id_verify("test", longHash, "OnArgonVerify"), "oversized PHC refused");
    Check(!crypto_argon2id_verify("test", "\xE9", "OnArgonVerify"), "non-ASCII PHC refused");

    // Workers may finish now, but their posted results still occupy the limit.
    new accepted;
    Submitting = true;
    for (new i; i < 129; i++)
    {
        if (crypto_argon2id_verify("test", "invalid", "OnArgonVerify", 1000))
        {
            accepted++;
            Pending++;
        }
    }
    Submitting = false;
    Check(accepted == 128, "128 total pending operations enforced");
    new timerCallback[] = "AdvanceArgonTest";
    Timer = SetTimer(timerCallback, 100, true);
    return 1;
}

public AdvanceArgonTest()
{
    if (Pending != 0) return 1;
    if (Phase == 0)
    {
        Phase = 1;
        // Reload the same AMX while an old generation has work outstanding.
        CancellationPhase = 1;
        new loadCancel[] = "loadfs argon2id_cancel";
        new unloadCancel[] = "unloadfs argon2id_cancel";
        SendRconCommand(loadCancel);
        SendRconCommand(unloadCancel);
        CancellationPhase = 2;
        SendRconCommand(loadCancel);
        Check(CallRemoteFunction("ArgonCancellationArmed", "") == 1, "cancellation script reloaded");
        Submitting = true;
        Queued(crypto_argon2id_hash("test password", "OnArgonHash", 1));
        Queued(crypto_argon2id_hash("", "OnArgonHash", 2));
        Queued(crypto_argon2id_hash(!"\xE9", "OnArgonHash", 3));
        Queued(crypto_argon2id_hash("test password", "OnArgonHash", 4));
        Queued(crypto_argon2id_verify("password", "$argon2id$v=19$m=256,t=2,p=1$c29tZXNhbHQ$nf65EOgLrQMR/uIPnA4rEsF5h7TKyQwu9U1bMCHGi/4", "OnArgonVerify", 10));
        Queued(crypto_argon2id_verify("wrong", "$argon2id$v=19$m=256,t=2,p=1$c29tZXNhbHQ$nf65EOgLrQMR/uIPnA4rEsF5h7TKyQwu9U1bMCHGi/4", "OnArgonVerify", 11));
        Queued(crypto_argon2id_verify("password", "$argon2i$v=19$m=256,t=2,p=1$c29tZXNhbHQ$nf65EOgLrQMR/uIPnA4rEsF5h7TKyQwu9U1bMCHGi/4", "OnArgonVerify", 12));
        Queued(crypto_argon2id_verify("password", "$argon2id$v=19$m=65537,t=2,p=1$c29tZXNhbHQ$nf65EOgLrQMR/uIPnA4rEsF5h7TKyQwu9U1bMCHGi/4", "OnArgonVerify", 13));
        Submitting = false;
        new command[] = "loadfs argon2id_other";
        SendRconCommand(command);
        // An operation from this AMX must not be delivered to the filterscript.
        Queued(crypto_argon2id_hash("owned by gamemode", "OnOwnerHash", 123));
        return 1;
    }
    if (CallRemoteFunction("ArgonOtherDone", "") != 1) return 1;
    KillTimer(Timer);
    printf("ARGON_TEST_DONE: %d failures, %d checks", Failures, Checks);
    new command[] = "exit";
    SendRconCommand(command);
    return 1;
}

forward OnOwnerHash(status, const hash[], context);
public OnOwnerHash(status, const hash[], context)
{
    Pending--;
    Check(status == CRYPTO_PASSWORD_OK && context == 123 && strlen(hash) > 0, "gamemode owns callback");
    return 1;
}

public OnArgonHash(status, const hash[], context)
{
    Pending--;
    Check(!Submitting, "hash callback is asynchronous");
    Check(status == CRYPTO_PASSWORD_OK, "hash succeeded");
    Check(strfind(hash, "$argon2id$v=19$m=19456,t=2,p=1$") == 0, "default PHC parameters");
    Check(strlen(hash) < CRYPTO_ARGON2ID_HASH_SIZE, "PHC fits documented buffer");
    if (context == 1 || context == 4)
    {
        if (FirstHash[0]) Check(strcmp(FirstHash, hash) != 0, "random salts differ");
        else format(FirstHash, sizeof(FirstHash), "%s", hash);
        Queued(crypto_argon2id_verify("test password", hash, "OnArgonVerify", 1));
        Queued(crypto_argon2id_verify("wrong", hash, "OnArgonVerify", 11));
    }
    else if (context == 2) Queued(crypto_argon2id_verify("", hash, "OnArgonVerify", 1));
    else if (context == 3)
    {
        Queued(crypto_argon2id_verify(!"\xE9", hash, "OnArgonVerify", 1));
        Queued(crypto_argon2id_verify("\xC3\xA9", hash, "OnArgonVerify", 11));
    }
    else Check(false, "unexpected hash context");
    return 1;
}

public OnArgonVerify(status, bool:matches, context)
{
    Pending--;
    Check(!Submitting, "verify callback is asynchronous");
    if (context == 1000 || context == 12 || context == 13)
        Check(status == CRYPTO_PASSWORD_INVALID_HASH && !matches, "invalid PHC status");
    else if (context == 11)
        Check(status == CRYPTO_PASSWORD_OK && !matches, "wrong password is not an internal error");
    else if (context == 1 || context == 10)
        Check(status == CRYPTO_PASSWORD_OK && matches, "correct password verifies");
    else Check(false, "unexpected verify context");
    return 1;
}
