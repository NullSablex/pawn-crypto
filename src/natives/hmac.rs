use crate::{
    hmac::{self, Algorithm},
    plugin::CryptoPlugin,
};
use samp::native;
use samp::prelude::*;
use zeroize::Zeroizing;

impl CryptoPlugin {
    #[native(name = "crypto_hmac_sha256")]
    pub fn crypto_hmac_sha256(
        _amx: &Amx,
        input: &AmxString,
        key: &AmxString,
        output: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        sign(Algorithm::Sha256, input, key, output, size)
    }

    #[native(name = "crypto_hmac_sha512")]
    pub fn crypto_hmac_sha512(
        _amx: &Amx,
        input: &AmxString,
        key: &AmxString,
        output: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        sign(Algorithm::Sha512, input, key, output, size)
    }

    #[native(name = "crypto_hmac_sha256_verify")]
    pub fn crypto_hmac_sha256_verify(
        _amx: &Amx,
        input: &AmxString,
        key: &AmxString,
        expected: &AmxString,
    ) -> bool {
        verify(Algorithm::Sha256, input, key, expected)
    }

    #[native(name = "crypto_hmac_sha512_verify")]
    pub fn crypto_hmac_sha512_verify(
        _amx: &Amx,
        input: &AmxString,
        key: &AmxString,
        expected: &AmxString,
    ) -> bool {
        verify(Algorithm::Sha512, input, key, expected)
    }
}

fn sign(
    algorithm: Algorithm,
    input: &AmxString,
    key: &AmxString,
    output: UnsizedBuffer,
    size: usize,
) -> AmxResult<bool> {
    if size <= algorithm.hex_len() {
        return Ok(false);
    }
    let key = Zeroizing::new(key.to_bytes());
    let Some(tag) = hmac::sign(algorithm, &input.to_bytes(), &key) else {
        return Ok(false);
    };
    output.write_str(size, &tag)?;
    Ok(true)
}

fn verify(algorithm: Algorithm, input: &AmxString, key: &AmxString, expected: &AmxString) -> bool {
    let key = Zeroizing::new(key.to_bytes());
    hmac::verify(algorithm, &input.to_bytes(), &key, &expected.to_bytes())
}
