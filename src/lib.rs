//! Pawn crypto plugin entry points and native registration.

mod encoding;
mod hash;
mod hmac;
mod logger;
mod natives;
mod password;
mod plugin;

use plugin::CryptoPlugin;
use samp::initialize_plugin;

initialize_plugin!(
    natives: [
        // Hashing
        CryptoPlugin::crypto_sha224,
        CryptoPlugin::crypto_sha256,
        CryptoPlugin::crypto_sha384,
        CryptoPlugin::crypto_sha512,
        // Message authentication
        CryptoPlugin::crypto_hmac_sha256,
        CryptoPlugin::crypto_hmac_sha512,
        CryptoPlugin::crypto_hmac_sha256_verify,
        CryptoPlugin::crypto_hmac_sha512_verify,
        // Password hashing
        CryptoPlugin::crypto_argon2id_hash,
        CryptoPlugin::crypto_argon2id_verify,
    ],
    {
        samp::plugin::enable_tick();
        samp::mainthread::set_budget(Some(std::time::Duration::from_millis(2)));
        return CryptoPlugin::new();
    }
);
