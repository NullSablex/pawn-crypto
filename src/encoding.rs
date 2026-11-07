/// Encodes bytes as lowercase hexadecimal without interpreting their encoding.
pub fn hex(input: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(input.len() * 2);
    for &byte in input {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

/// Decodes an exact-length public tag, accepting upper- or lowercase hex.
pub fn decode_hex<const N: usize>(input: &[u8]) -> Option<[u8; N]> {
    if input.len() != N * 2 {
        return None;
    }
    let mut output = [0; N];
    for (pair, byte) in input.as_chunks::<2>().0.iter().zip(&mut output) {
        *byte = (nibble(pair[0])? << 4) | nibble(pair[1])?;
    }
    Some(output)
}

fn nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_hex, hex};

    #[test]
    fn binary_hex_roundtrip() {
        let bytes: Vec<_> = (u8::MIN..=u8::MAX).collect();
        let encoded = hex(&bytes);
        assert_eq!(
            decode_hex::<256>(encoded.as_bytes()).unwrap().as_slice(),
            bytes
        );
        assert_eq!(
            decode_hex::<256>(encoded.to_ascii_uppercase().as_bytes())
                .unwrap()
                .as_slice(),
            bytes
        );
    }

    #[test]
    fn refuses_malformed_hex() {
        for invalid in [b"0".as_slice(), b"000", b"gg", b"-1", b" 0", &[0xff, 0xff]] {
            assert!(decode_hex::<1>(invalid).is_none());
        }
    }
}
