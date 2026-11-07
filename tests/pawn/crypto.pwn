// Run as a gamemode to exercise the real AMX native boundary.
#include <a_samp>
#include <crypto>

new Failures;
new Checks;

Check(bool:condition, const label[])
{
    Checks++;
    if (!condition)
    {
        printf("CRYPTO_TEST_FAILED: %s", label);
        Failures++;
    }
}

main() {}

public OnGameModeInit()
{
    new digest[CRYPTO_SHA256_LENGTH + 1];
    Check(crypto_sha256("abc", digest), "normal input accepted");
    Check(!strcmp(digest, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"), "abc vector");

    Check(crypto_sha256("", digest), "empty input accepted");
    Check(!strcmp(digest, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"), "empty vector");

    Check(crypto_sha256(!"abc", digest), "packed input accepted");
    Check(!strcmp(digest, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"), "packed vector");

    Check(crypto_sha256("\xE9", digest), "non-UTF8 input accepted");
    Check(!strcmp(digest, "de2e331d891ae267a7009cb45b4e8830f170e0c937288ea2731a1941c7a53b0d"), "non-UTF8 vector");

    new small[64] = "unchanged";
    Check(!crypto_sha256("abc", small), "short buffer refused");
    Check(!strcmp(small, "unchanged"), "short buffer preserved");
    Check(!crypto_sha256("abc", digest, 0), "zero capacity refused");

    new inplace[CRYPTO_SHA256_LENGTH + 1] = "abc";
    Check(crypto_sha256(inplace, inplace), "in-place input accepted");
    Check(!strcmp(inplace, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"), "in-place vector");


    new output[CRYPTO_SHA512_LENGTH + 1];
    Check(crypto_sha224("abc", output, CRYPTO_SHA224_LENGTH + 1), "SHA-224 abc accepted");
    Check(!strcmp(output, "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7"), "SHA-224 abc vector");
    Check(crypto_sha224("", output, CRYPTO_SHA224_LENGTH + 1), "SHA-224 empty accepted");
    Check(!strcmp(output, "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f"), "SHA-224 empty vector");
    Check(crypto_sha224(!"abc", output, CRYPTO_SHA224_LENGTH + 1), "SHA-224 packed accepted");
    Check(!strcmp(output, "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7"), "SHA-224 packed vector");
    Check(crypto_sha224("\xE9", output, CRYPTO_SHA224_LENGTH + 1), "SHA-224 raw byte accepted");
    Check(!strcmp(output, "c04ab00098897882933d4899f4ba593024f14943bb25e98b018cd994"), "SHA-224 raw byte vector");
    new short224[CRYPTO_SHA224_LENGTH] = "unchanged";
    Check(!crypto_sha224("abc", short224), "SHA-224 short buffer refused");
    Check(!strcmp(short224, "unchanged"), "SHA-224 short buffer preserved");
    Check(crypto_sha384("abc", output, CRYPTO_SHA384_LENGTH + 1), "SHA-384 abc accepted");
    Check(!strcmp(output, "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"), "SHA-384 abc vector");
    Check(crypto_sha384("", output, CRYPTO_SHA384_LENGTH + 1), "SHA-384 empty accepted");
    Check(!strcmp(output, "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b"), "SHA-384 empty vector");
    Check(crypto_sha384(!"abc", output, CRYPTO_SHA384_LENGTH + 1), "SHA-384 packed accepted");
    Check(!strcmp(output, "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"), "SHA-384 packed vector");
    Check(crypto_sha384("\xE9", output, CRYPTO_SHA384_LENGTH + 1), "SHA-384 raw byte accepted");
    Check(!strcmp(output, "cf16e9b450d92416b3a86e452a5162e8d6d496a9e126dcead3d60ce8cb9242baeb94b24c73af76c71c00c11a7c0acc03"), "SHA-384 raw byte vector");
    new short384[CRYPTO_SHA384_LENGTH] = "unchanged";
    Check(!crypto_sha384("abc", short384), "SHA-384 short buffer refused");
    Check(!strcmp(short384, "unchanged"), "SHA-384 short buffer preserved");
    Check(crypto_sha512("abc", output, CRYPTO_SHA512_LENGTH + 1), "SHA-512 abc accepted");
    Check(!strcmp(output, "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"), "SHA-512 abc vector");
    Check(crypto_sha512("", output, CRYPTO_SHA512_LENGTH + 1), "SHA-512 empty accepted");
    Check(!strcmp(output, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"), "SHA-512 empty vector");
    Check(crypto_sha512(!"abc", output, CRYPTO_SHA512_LENGTH + 1), "SHA-512 packed accepted");
    Check(!strcmp(output, "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"), "SHA-512 packed vector");
    Check(crypto_sha512("\xE9", output, CRYPTO_SHA512_LENGTH + 1), "SHA-512 raw byte accepted");
    Check(!strcmp(output, "b164716d213ddec0e2dbf8f2c7cc16d673b14c7a5ce46826e10ad36abe97fdeb9a34bb379da49edf29a46f4d2b77883c97786b63d543924dad3873c43a91a1af"), "SHA-512 raw byte vector");
    new short512[CRYPTO_SHA512_LENGTH] = "unchanged";
    Check(!crypto_sha512("abc", short512), "SHA-512 short buffer refused");
    Check(!strcmp(short512, "unchanged"), "SHA-512 short buffer preserved");

    Check(crypto_hmac_sha256("what do ya want for nothing?", "Jefe", output), "HMAC-256 accepted");
    Check(!strcmp(output, "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"), "HMAC-256 RFC4231 vector");
    Check(crypto_hmac_sha256_verify("what do ya want for nothing?", "Jefe", output), "HMAC-256 verified");
    Check(crypto_hmac_sha256_verify("what do ya want for nothing?", "Jefe", "5BDCC146BF60754E6A042426089575C75A003F089D2739839DEC58B964EC3843"), "HMAC-256 uppercase tag");
    Check(!crypto_hmac_sha256_verify("changed", "Jefe", output), "HMAC-256 message mismatch");
    Check(!crypto_hmac_sha256_verify("what do ya want for nothing?", "wrong", output), "HMAC-256 key mismatch");
    Check(!crypto_hmac_sha256_verify("what do ya want for nothing?", "Jefe", "0bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"), "HMAC-256 tag mismatch");
    Check(!crypto_hmac_sha256_verify("what do ya want for nothing?", "Jefe", "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec384"), "HMAC-256 truncated tag");
    Check(!crypto_hmac_sha256_verify("what do ya want for nothing?", "Jefe", "gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg"), "HMAC-256 invalid hex");
    Check(crypto_hmac_sha256(!"what do ya want for nothing?", !"Jefe", output), "HMAC-256 packed accepted");
    Check(!strcmp(output, "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"), "HMAC-256 packed vector");
    Check(crypto_hmac_sha256("", "", output), "HMAC-256 empty accepted");
    Check(!strcmp(output, "b613679a0814d9ec772f95d778c35fc5ff1697c493715653c6c712144292c5ad"), "HMAC-256 empty vector");
    Check(crypto_hmac_sha256_verify("", "", output), "HMAC-256 empty verified");
    Check(crypto_hmac_sha256("\xFF", "\xE9", output), "HMAC-256 raw accepted");
    Check(!strcmp(output, "fc9081ebd54e6733de24fd6e9cf4873b7e38a3d905690a2acb59399e2ed864fd"), "HMAC-256 raw vector");
    Check(crypto_hmac_sha256_verify("\xFF", "\xE9", output), "HMAC-256 raw verified");
    new hmacshort256[CRYPTO_HMAC_SHA256_LENGTH] = "unchanged";
    Check(!crypto_hmac_sha256("message", "key", hmacshort256), "HMAC-256 short buffer refused");
    Check(!strcmp(hmacshort256, "unchanged"), "HMAC-256 short buffer preserved");
    Check(!crypto_hmac_sha256("message", "key", output, 0), "HMAC-256 zero capacity refused");
    new hmaci256[CRYPTO_HMAC_SHA256_LENGTH + 1] = "what do ya want for nothing?";
    Check(crypto_hmac_sha256(hmaci256, "Jefe", hmaci256), "HMAC-256 in-place accepted");
    Check(!strcmp(hmaci256, "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"), "HMAC-256 in-place vector");

    Check(crypto_hmac_sha512("what do ya want for nothing?", "Jefe", output), "HMAC-512 accepted");
    Check(!strcmp(output, "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"), "HMAC-512 RFC4231 vector");
    Check(crypto_hmac_sha512_verify("what do ya want for nothing?", "Jefe", output), "HMAC-512 verified");
    Check(crypto_hmac_sha512_verify("what do ya want for nothing?", "Jefe", "164B7A7BFCF819E2E395FBE73B56E0A387BD64222E831FD610270CD7EA2505549758BF75C05A994A6D034F65F8F0E6FDCAEAB1A34D4A6B4B636E070A38BCE737"), "HMAC-512 uppercase tag");
    Check(!crypto_hmac_sha512_verify("changed", "Jefe", output), "HMAC-512 message mismatch");
    Check(!crypto_hmac_sha512_verify("what do ya want for nothing?", "wrong", output), "HMAC-512 key mismatch");
    Check(!crypto_hmac_sha512_verify("what do ya want for nothing?", "Jefe", "064b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"), "HMAC-512 tag mismatch");
    Check(!crypto_hmac_sha512_verify("what do ya want for nothing?", "Jefe", "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce73"), "HMAC-512 truncated tag");
    Check(!crypto_hmac_sha512_verify("what do ya want for nothing?", "Jefe", "gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg"), "HMAC-512 invalid hex");
    Check(crypto_hmac_sha512(!"what do ya want for nothing?", !"Jefe", output), "HMAC-512 packed accepted");
    Check(!strcmp(output, "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"), "HMAC-512 packed vector");
    Check(crypto_hmac_sha512("", "", output), "HMAC-512 empty accepted");
    Check(!strcmp(output, "b936cee86c9f87aa5d3c6f2e84cb5a4239a5fe50480a6ec66b70ab5b1f4ac6730c6c515421b327ec1d69402e53dfb49ad7381eb067b338fd7b0cb22247225d47"), "HMAC-512 empty vector");
    Check(crypto_hmac_sha512_verify("", "", output), "HMAC-512 empty verified");
    Check(crypto_hmac_sha512("\xFF", "\xE9", output), "HMAC-512 raw accepted");
    Check(!strcmp(output, "382ee1353298ef46af94a089f1267b1db3fd73e9a8758cc738ce28c2ec8425db98333018e25ef4feaf732d190f066360f4c287b6bb4d2dc94ea984a663ffac12"), "HMAC-512 raw vector");
    Check(crypto_hmac_sha512_verify("\xFF", "\xE9", output), "HMAC-512 raw verified");
    new hmacshort512[CRYPTO_HMAC_SHA512_LENGTH] = "unchanged";
    Check(!crypto_hmac_sha512("message", "key", hmacshort512), "HMAC-512 short buffer refused");
    Check(!strcmp(hmacshort512, "unchanged"), "HMAC-512 short buffer preserved");
    Check(!crypto_hmac_sha512("message", "key", output, 0), "HMAC-512 zero capacity refused");
    new hmaci512[CRYPTO_HMAC_SHA512_LENGTH + 1] = "what do ya want for nothing?";
    Check(crypto_hmac_sha512(hmaci512, "Jefe", hmaci512), "HMAC-512 in-place accepted");
    Check(!strcmp(hmaci512, "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"), "HMAC-512 in-place vector");

    printf("CRYPTO_TEST_DONE: %d failures, %d checks", Failures, Checks);
    new command[] = "exit";
    SendRconCommand(command);
    return 1;
}
