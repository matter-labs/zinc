use num_bigint::{BigInt};
use bellman::pairing::Engine;
use ff::{PrimeField, PrimeFieldRepr};
use num_traits::Zero;

pub fn fr_to_bigint<E: Engine>(fr: &E::Fr) -> BigInt {
    let mut buf: Vec<u8> = Vec::new();
    fr.into_repr().write_be(&mut buf);

    let mut bigint = BigInt::zero();

    for byte in buf.iter() {
        bigint = (bigint << 8) + *byte;
    }

    bigint
}

pub fn bigint_to_fr<E: Engine>(bigint: &BigInt) -> Option<E::Fr> {
    E::Fr::from_str(&bigint.to_str_radix(10))
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

        for v in values.iter() {
            let bigint = BigInt::from(*v);
            let fr = bigint_to_fr::<Bn256>(&bigint);
            assert_eq!(fr, Fr::from_str(&v.to_string()));
        }
    }
}
