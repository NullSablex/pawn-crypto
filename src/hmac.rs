use crate::encoding::{decode_hex, hex};
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Sha256, Sha512};

#[derive(Clone, Copy)]
pub enum Algorithm {
    Sha256,
    Sha512,
}

impl Algorithm {
    pub const fn hex_len(self) -> usize {
        match self {
            Self::Sha256 => 64,
            Self::Sha512 => 128,
        }
    }
}

pub fn sign(algorithm: Algorithm, input: &[u8], key: &[u8]) -> Option<String> {
    match algorithm {
        Algorithm::Sha256 => {
            let mut mac = Hmac::<Sha256>::new_from_slice(key).ok()?;
            mac.update(input);
            Some(hex(&mac.finalize().into_bytes()))
        }
        Algorithm::Sha512 => {
            let mut mac = Hmac::<Sha512>::new_from_slice(key).ok()?;
            mac.update(input);
            Some(hex(&mac.finalize().into_bytes()))
        }
    }
}

/// Compares full decoded tags using the MAC library's constant-time verifier.
/// Hex syntax and length are public; malformed or truncated tags are refused.
pub fn verify(algorithm: Algorithm, input: &[u8], key: &[u8], expected: &[u8]) -> bool {
    match algorithm {
        Algorithm::Sha256 => {
            let Some(tag) = decode_hex::<32>(expected) else {
                return false;
            };
            let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(key) else {
                return false;
            };
            mac.update(input);
            mac.verify_slice(&tag).is_ok()
        }
        Algorithm::Sha512 => {
            let Some(tag) = decode_hex::<64>(expected) else {
                return false;
            };
            let Ok(mut mac) = Hmac::<Sha512>::new_from_slice(key) else {
                return false;
            };
            mac.update(input);
            mac.verify_slice(&tag).is_ok()
        }
    }
}

#[cfg(test)]
mod tests;
