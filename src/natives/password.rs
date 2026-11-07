use crate::{
    password::{MAX_HASH_LEN, MAX_PASSWORD_LEN},
    plugin::CryptoPlugin,
};
use samp::{native, prelude::*};
use zeroize::Zeroizing;

impl CryptoPlugin {
    #[native(name = "crypto_argon2id_hash")]
    pub fn crypto_argon2id_hash(
        &mut self,
        amx: &Amx,
        password: &AmxString,
        callback: &AmxString,
        context: i32,
    ) -> bool {
        let password = Zeroizing::new(password.to_bytes());
        if password.len() > MAX_PASSWORD_LEN {
            return false;
        }
        let Ok(name) = String::from_utf8(callback.to_bytes()) else {
            return false;
        };
        let Some(callback) = self.password_callback(amx, name, context) else {
            return false;
        };
        self.passwords.submit(password, None, callback)
    }

    #[native(name = "crypto_argon2id_verify")]
    pub fn crypto_argon2id_verify(
        &mut self,
        amx: &Amx,
        password: &AmxString,
        hash: &AmxString,
        callback: &AmxString,
        context: i32,
    ) -> bool {
        let password = Zeroizing::new(password.to_bytes());
        let stored = hash.to_bytes();
        if password.len() > MAX_PASSWORD_LEN || stored.len() > MAX_HASH_LEN {
            return false;
        }
        let Ok(stored) = String::from_utf8(stored) else {
            return false;
        };
        let Ok(name) = String::from_utf8(callback.to_bytes()) else {
            return false;
        };
        let Some(callback) = self.password_callback(amx, name, context) else {
            return false;
        };
        self.passwords.submit(password, Some(stored), callback)
    }
}
