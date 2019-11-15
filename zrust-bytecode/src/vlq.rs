use num_bigint::BigInt;
use num_traits::{ToPrimitive, Signed};
use std::ops::Rem;

#[allow(dead_code)]
pub fn encode(value: &BigInt) -> Vec<u8> {
    let mut chunks: Vec<u8> = Vec::new();
    let mut abs = value.abs();

    let sign: u8 = if value.is_negative() {
        abs -= 1;
        1 << 6
    } else {
        0
    };

    chunks.push(sign | abs.clone().rem(64u8).to_u8().unwrap());
    abs >>= 6;

    while abs.is_positive()  {
        abs -= 1;
        chunks.push(abs.clone().rem(128u8).to_u8().unwrap());
        abs >>= 7;
    }

    let len = chunks.len();

    chunks
        .iter()
        .enumerate()
        .map(|(i, &c)| if i == len - 1 { c } else { 128 | c })
        .collect()
}

#[allow(dead_code)]
pub fn decode(bytes: &[u8]) -> Option<(BigInt, usize)> {
    if bytes.is_empty() {
        return None
    }

    let first = bytes[0];
    let (mut number, sign) = if first & 64 == 0 {
        (BigInt::from(first & 63), 1)
    } else {
        (BigInt::from((first & 63) + 1), -1)
    };

    if first & 128 == 0 {
        return Some((sign * number, 1));
    }

    let mut len = 1;
    for (i, &b) in bytes[1..].iter().enumerate() {
        len += 1;
        let v = (127 & b) + 1;
        number += BigInt::from(v) << (7 * i + 6);

        if b & 128 == 0 {
            return Some((sign * number, len))
        }
    };

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use num_bigint::ToBigInt;

    #[test]
    fn test_encode() {
        assert_eq!(encode(&BigInt::from(0)), vec![0x00]);
        assert_eq!(encode(&BigInt::from(1)), vec![0x01]);
        assert_eq!(encode(&BigInt::from(63)), vec![0x3f]);
        assert_eq!(encode(&BigInt::from(64)), vec![0x80, 0x00]);
        assert_eq!(encode(&BigInt::from(65)), vec![0x81, 0x00]);
        assert_eq!(encode(&BigInt::from(-1)), vec![0x40]);
        assert_eq!(encode(&BigInt::from(-63)), vec![0x7e]);
        assert_eq!(encode(&BigInt::from(-64)), vec![0x7f]);
        assert_eq!(encode(&BigInt::from(-65)), vec![0xc0, 0x00]);
        assert_eq!(encode(&BigInt::from(-66)), vec![0xc1, 0x00]);
    }

    #[test]
    fn test_decode() {
        for &value in [0, 1, 63, 64, 65, -1, -63, -64, -65, -66].iter() {
            let bytes = encode(&value.to_bigint().unwrap());
            let (decoded, len) = decode(bytes.as_slice()).unwrap();
            assert_eq!(decoded.to_i32(), Some(value));
            assert_eq!(len, bytes.len())
        }
    }
}
