use crate::encoding::hex;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

#[derive(Clone, Copy)]
pub enum Algorithm {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

impl Algorithm {
    pub const fn hex_len(self) -> usize {
        match self {
            Self::Sha224 => 56,
            Self::Sha256 => 64,
            Self::Sha384 => 96,
            Self::Sha512 => 128,
        }
    }
}

/// Hashes bytes without decoding or normalizing text.
pub fn digest(algorithm: Algorithm, input: &[u8]) -> String {
    match algorithm {
        Algorithm::Sha224 => hex(&Sha224::digest(input)),
        Algorithm::Sha256 => hex(&Sha256::digest(input)),
        Algorithm::Sha384 => hex(&Sha384::digest(input)),
        Algorithm::Sha512 => hex(&Sha512::digest(input)),
    }
}

#[cfg(test)]
mod tests {
    use super::{Algorithm, digest};

    fn sha256(input: &[u8]) -> String {
        digest(Algorithm::Sha256, input)
    }

    #[test]
    fn standard_vectors() {
        for (input, expected) in [
            (
                "",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                "abc",
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
            (
                "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            ),
        ] {
            assert_eq!(sha256(input.as_bytes()), expected);
        }
    }

    #[test]
    fn million_byte_vector() {
        assert_eq!(
            sha256(&vec![b'a'; 1_000_000]),
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
        );
    }

    #[test]
    fn preserves_non_utf8_bytes() {
        // Independently checked against Python's hashlib.sha256.
        assert_eq!(
            sha256(&[0xe9]),
            "de2e331d891ae267a7009cb45b4e8830f170e0c937288ea2731a1941c7a53b0d"
        );
        assert_ne!(sha256(&[0xe9]), sha256("\u{fffd}".as_bytes()));
        assert_ne!(sha256(&[0xe9]), sha256("\u{e9}".as_bytes()));
    }
}

#[cfg(test)]
mod additional_tests {
    use super::{Algorithm, digest};

    #[test]
    fn additional_sha2_known_answers() {
        assert_eq!(
            digest(Algorithm::Sha224, b""),
            "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f"
        );
        assert_eq!(
            digest(Algorithm::Sha224, b"abc"),
            "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7"
        );
        assert_eq!(
            digest(Algorithm::Sha224, &vec![b'a'; 1_000_000]),
            "20794655980c91d8bbb4c1ea97618a4bf03f42581948b2ee4ee7ad67"
        );
        assert_eq!(
            digest(Algorithm::Sha224, &[0xe9]),
            "c04ab00098897882933d4899f4ba593024f14943bb25e98b018cd994"
        );
        assert_eq!(
            digest(Algorithm::Sha384, b""),
            "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b"
        );
        assert_eq!(
            digest(Algorithm::Sha384, b"abc"),
            "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"
        );
        assert_eq!(
            digest(Algorithm::Sha384, &vec![b'a'; 1_000_000]),
            "9d0e1809716474cb086e834e310a4a1ced149e9c00f248527972cec5704c2a5b07b8b3dc38ecc4ebae97ddd87f3d8985"
        );
        assert_eq!(
            digest(Algorithm::Sha384, &[0xe9]),
            "cf16e9b450d92416b3a86e452a5162e8d6d496a9e126dcead3d60ce8cb9242baeb94b24c73af76c71c00c11a7c0acc03"
        );
        assert_eq!(
            digest(Algorithm::Sha512, b""),
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
        );
        assert_eq!(
            digest(Algorithm::Sha512, b"abc"),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );
        assert_eq!(
            digest(Algorithm::Sha512, &vec![b'a'; 1_000_000]),
            "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973ebde0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b"
        );
        assert_eq!(
            digest(Algorithm::Sha512, &[0xe9]),
            "b164716d213ddec0e2dbf8f2c7cc16d673b14c7a5ce46826e10ad36abe97fdeb9a34bb379da49edf29a46f4d2b77883c97786b63d543924dad3873c43a91a1af"
        );
    }
}
