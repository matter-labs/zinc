use num_bigint::BigInt;
use num_traits::ToPrimitive;
use num_integer::Integer;

pub fn encode(mut number: BigInt) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    while number >= BigInt::from(0x80) {
        let (div, rem) = number.div_rem(&BigInt::from(0x80));
        bytes.push(0x80 | rem.to_u8().unwrap());
        number = div;
    }
    bytes.push(number.to_u8().unwrap());

    bytes
}

pub fn decode(bytes: &[u8]) -> (BigInt, usize) {
    let mut number = BigInt::from(0);
    let mut len = 0;
    for (i, b) in bytes.iter().enumerate() {
        number += BigInt::from(b & 0x7F) * 1 << (7 * i);
        len += 1;
        if b & 0x80 == 0 {
            break;
        }
    }

    (number, len)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode(BigInt::from(1)), vec![0x01]);
        assert_eq!(encode(BigInt::from(127)), vec![0x7F]);
        assert_eq!(encode(BigInt::from(128)), vec![0x80, 0x01]);
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode(&[0x01]), (BigInt::from(1), 1));
        assert_eq!(decode(&[0x7F]), (BigInt::from(127), 1));
        assert_eq!(decode(&[0x80, 0x01]), (BigInt::from(128), 2));
    }
}
