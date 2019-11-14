use bellman::pairing::Engine;
use num_bigint::BigInt;
use num_traits::{Zero, Signed};
use ff::{Field, PrimeField, PrimeFieldRepr};
use std::ops::Neg;

pub fn fr_to_bigint<E: Engine>(fr: &E::Fr) -> BigInt {
    let mut buf: Vec<u8> = Vec::new();
    match fr.into_repr().write_be(&mut buf) {
        Ok(_) => {},
        Err(_) => {
            log::error!("failed to convert Fr to BigInt: {:?}", fr);
        }
    }

    let mut bigint = BigInt::zero();

    for byte in buf.iter() {
        bigint = (bigint << 8) + *byte;
    }

    if bigint.bits() < E::Fr::CAPACITY as usize {
        bigint
    } else {
        let mut fr_neg = fr.clone();
        fr_neg.negate();
        - fr_to_bigint::<E>(&fr_neg)
    }
}

pub fn bigint_to_fr<E: Engine>(bigint: &BigInt) -> Option<E::Fr> {
    if bigint.is_positive() {
        E::Fr::from_str(&bigint.to_str_radix(10))
    } else {
        let mut fr = E::Fr::zero();
        match E::Fr::from_str(&bigint.neg().to_str_radix(10)) {
            Some(abs) => {
                fr.sub_assign(&abs);
                Some(fr)
            }
            None => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bellman::pairing::bn256::{Bn256, Fr};
    use num_traits::ToPrimitive;

    #[test]
    fn test_fr_to_bigint() {
        let values = [0, 1, 2, 42, 1234567890];

        for v in values.iter() {
            let fr = Fr::from_str(&v.to_string()).unwrap();
            let bigint = fr_to_bigint::<Bn256>(&fr);
            assert_eq!(bigint.to_i32(), Some(*v));
        }
    }

    #[test]
    fn test_bigint_to_fr() {
        let values = [0, 1, 2, 42, 1234567890];

        for &v in values.iter() {
            let bigint = BigInt::from(v);
            let fr = bigint_to_fr::<Bn256>(&bigint);
            assert_eq!(fr, Fr::from_str(&v.to_string()));
        }
    }

    #[test]
    fn test_negatives() {
        let values = [-1 as isize, -42, -123456789098761];

        for &v in values.iter() {
            let expected = BigInt::from(v);
            let fr = bigint_to_fr::<Bn256>(&expected).expect("bigint_to_fr");
            let actual = fr_to_bigint::<Bn256>(&fr);
            assert_eq!(actual, expected);
        }
    }
}
