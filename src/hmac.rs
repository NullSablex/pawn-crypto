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
mod tests {
    use super::{Algorithm, sign, verify};

    #[test]
    fn rfc4231_known_answers() {
        assert_eq!(
            sign(Algorithm::Sha256, b"Hi There", &[0x0b; 20]).unwrap(),
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
        assert!(verify(
            Algorithm::Sha256,
            b"Hi There",
            &[0x0b; 20],
            b"b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        ));
        assert_eq!(
            sign(Algorithm::Sha256, b"what do ya want for nothing?", b"Jefe").unwrap(),
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
        assert!(verify(
            Algorithm::Sha256,
            b"what do ya want for nothing?",
            b"Jefe",
            b"5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        ));
        assert_eq!(
            sign(Algorithm::Sha256, &[0xdd; 50], &[0xaa; 20]).unwrap(),
            "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe"
        );
        assert!(verify(
            Algorithm::Sha256,
            &[0xdd; 50],
            &[0xaa; 20],
            b"773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe"
        ));
        assert_eq!(
            sign(
                Algorithm::Sha256,
                b"Test Using Larger Than Block-Size Key - Hash Key First",
                &[0xaa; 131]
            )
            .unwrap(),
            "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"
        );
        assert!(verify(
            Algorithm::Sha256,
            b"Test Using Larger Than Block-Size Key - Hash Key First",
            &[0xaa; 131],
            b"60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"
        ));
        assert_eq!(
            sign(Algorithm::Sha512, b"Hi There", &[0x0b; 20]).unwrap(),
            "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cdedaa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"
        );
        assert!(verify(Algorithm::Sha512, b"Hi There", &[0x0b; 20], b"87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cdedaa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"));
        assert_eq!(
            sign(Algorithm::Sha512, b"what do ya want for nothing?", b"Jefe").unwrap(),
            "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"
        );
        assert!(verify(Algorithm::Sha512, b"what do ya want for nothing?", b"Jefe", b"164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea2505549758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"));
        assert_eq!(
            sign(Algorithm::Sha512, &[0xdd; 50], &[0xaa; 20]).unwrap(),
            "fa73b0089d56a284efb0f0756c890be9b1b5dbdd8ee81a3655f83e33b2279d39bf3e848279a722c806b485a47e67c807b946a337bee8942674278859e13292fb"
        );
        assert!(verify(Algorithm::Sha512, &[0xdd; 50], &[0xaa; 20], b"fa73b0089d56a284efb0f0756c890be9b1b5dbdd8ee81a3655f83e33b2279d39bf3e848279a722c806b485a47e67c807b946a337bee8942674278859e13292fb"));
        assert_eq!(
            sign(
                Algorithm::Sha512,
                b"Test Using Larger Than Block-Size Key - Hash Key First",
                &[0xaa; 131]
            )
            .unwrap(),
            "80b24263c7c1a3ebb71493c1dd7be8b49b46d1f41b4aeec1121b013783f8f3526b56d037e05f2598bd0fd2215d6a1e5295e64f73f63f0aec8b915a985d786598"
        );
        assert!(verify(Algorithm::Sha512, b"Test Using Larger Than Block-Size Key - Hash Key First", &[0xaa; 131], b"80b24263c7c1a3ebb71493c1dd7be8b49b46d1f41b4aeec1121b013783f8f3526b56d037e05f2598bd0fd2215d6a1e5295e64f73f63f0aec8b915a985d786598"));
    }

    #[test]
    fn verification_rejects_tampering_and_malformed_tags() {
        for algorithm in [Algorithm::Sha256, Algorithm::Sha512] {
            let tag = sign(algorithm, b"message", b"secret").unwrap();
            assert!(verify(algorithm, b"message", b"secret", tag.as_bytes()));
            assert!(verify(
                algorithm,
                b"message",
                b"secret",
                tag.to_ascii_uppercase().as_bytes()
            ));
            assert!(!verify(algorithm, b"changed", b"secret", tag.as_bytes()));
            assert!(!verify(algorithm, b"message", b"wrong", tag.as_bytes()));
            for invalid in [
                "",
                " ",
                &tag[..tag.len() - 1],
                &format!("{tag}00"),
                &"g".repeat(tag.len()),
            ] {
                assert!(!verify(
                    algorithm,
                    b"message",
                    b"secret",
                    invalid.as_bytes()
                ));
            }
            for at in 0..tag.len() {
                let mut bad = tag.as_bytes().to_vec();
                bad[at] = if bad[at] == b'0' { b'1' } else { b'0' };
                assert!(!verify(algorithm, b"message", b"secret", &bad));
            }
        }
    }

    #[test]
    fn empty_and_non_utf8_inputs_are_not_normalized() {
        assert_eq!(
            sign(Algorithm::Sha256, b"", b"").unwrap(),
            "b613679a0814d9ec772f95d778c35fc5ff1697c493715653c6c712144292c5ad"
        );
        assert_eq!(
            sign(Algorithm::Sha256, &[0xff], &[0xe9]).unwrap(),
            "fc9081ebd54e6733de24fd6e9cf4873b7e38a3d905690a2acb59399e2ed864fd"
        );
        assert_eq!(
            sign(Algorithm::Sha512, b"", b"").unwrap(),
            "b936cee86c9f87aa5d3c6f2e84cb5a4239a5fe50480a6ec66b70ab5b1f4ac6730c6c515421b327ec1d69402e53dfb49ad7381eb067b338fd7b0cb22247225d47"
        );
        assert_eq!(
            sign(Algorithm::Sha512, &[0xff], &[0xe9]).unwrap(),
            "382ee1353298ef46af94a089f1267b1db3fd73e9a8758cc738ce28c2ec8425db98333018e25ef4feaf732d190f066360f4c287b6bb4d2dc94ea984a663ffac12"
        );
    }
}
