use crate::{
    hash::{self, Algorithm},
    plugin::CryptoPlugin,
};
use samp::native;
use samp::prelude::*;

impl CryptoPlugin {
    #[native(name = "crypto_sha224")]
    pub fn crypto_sha224(
        _amx: &Amx,
        input: &AmxString,
        output: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        hash(Algorithm::Sha224, input, output, size)
    }

    #[native(name = "crypto_sha256")]
    pub fn crypto_sha256(
        _amx: &Amx,
        input: &AmxString,
        output: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        hash(Algorithm::Sha256, input, output, size)
    }

    #[native(name = "crypto_sha384")]
    pub fn crypto_sha384(
        _amx: &Amx,
        input: &AmxString,
        output: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        hash(Algorithm::Sha384, input, output, size)
    }

    #[native(name = "crypto_sha512")]
    pub fn crypto_sha512(
        _amx: &Amx,
        input: &AmxString,
        output: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        hash(Algorithm::Sha512, input, output, size)
    }
}

fn hash(
    algorithm: Algorithm,
    input: &AmxString,
    output: UnsizedBuffer,
    size: usize,
) -> AmxResult<bool> {
    if size <= algorithm.hex_len() {
        return Ok(false);
    }
    // Dereferencing AmxString would decode text and could change its bytes.
    let digest = hash::digest(algorithm, &input.to_bytes());
    output.write_str(size, &digest)?;
    Ok(true)
}
