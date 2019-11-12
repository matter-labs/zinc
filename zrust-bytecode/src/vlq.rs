use num_bigint::{BigInt, ToBigInt};
use num_traits::ToPrimitive;
use std::ops::Rem;

#[allow(dead_code)]
pub fn encode(mut value: BigInt) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    while value >= 128_u8.to_bigint().unwrap()  {
        bytes.push(128 | value.clone().rem(128 as u8).to_u8().unwrap());
        value >>= 7;
        value -= 1;
    }

    bytes.push(value.to_u8().unwrap());

    bytes
}

#[allow(dead_code)]
pub fn decode(bytes: &[u8]) -> Option<(BigInt, usize)> {
    let mut number = BigInt::from(0);

    let mut len = 0;
    for (i, &b) in bytes.iter().enumerate() {
        len += 1;

        if b & 128 == 0 {
            number += b.to_bigint().unwrap() << (7 * i);
            return Some((number, len))
        }

        let v = (127 & b) + 128;
        number += v.to_bigint().unwrap() << (7 * i);
    };

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode(BigInt::from(1)), vec![0x01]);
        assert_eq!(encode(BigInt::from(127)), vec![0x7F]);
        assert_eq!(encode(BigInt::from(128)), vec![0x80, 0x00]);
        assert_eq!(encode(BigInt::from(129)), vec![0x81, 0x00]);
        assert_eq!(encode(BigInt::from(255)), vec![0xff, 0x00]);
        assert_eq!(encode(BigInt::from(256)), vec![0x80, 0x01]);
    }

    #[test]
    fn test_decode() {
        for &value in [0, 1, 2, 127, 128, 255, 3000, 456745356].iter() {
            let bytes = encode(value.to_bigint().unwrap());
            let (decoded, len) = decode(bytes.as_slice()).unwrap();
            assert_eq!(decoded.to_i32(), Some(value));
            assert_eq!(len, bytes.len())
        }
    }
}
