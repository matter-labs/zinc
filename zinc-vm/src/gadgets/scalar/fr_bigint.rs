use std::ops::Div;
use std::ops::Neg;

use num_bigint::BigInt;
use num_bigint::Sign;
use num_traits::Signed;

use ff::Field;
use ff::PrimeField;
use ff::PrimeFieldRepr;

use zinc_bytecode::ScalarType;

use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn fr_to_bigint<Fr: PrimeField>(fr: &Fr, signed: bool) -> BigInt {
    if signed {
        fr_to_bigint_signed(fr)
    } else {
        fr_to_bigint_unsigned(fr)
    }
}

pub fn bigint_to_fr<E: IEngine>(bigint: &BigInt) -> Option<E::Fr> {
    if bigint.is_positive() {
        E::Fr::from_str(&bigint.to_str_radix(10))
    } else {
        let abs = E::Fr::from_str(&bigint.neg().to_str_radix(10))?;
        let mut fr = E::Fr::zero();
        fr.sub_assign(&abs);
        Some(fr)
    }
}

pub fn bigint_to_fr_scalar<E: IEngine>(
    bigint: &BigInt,
    scalar_type: ScalarType,
) -> Result<Scalar<E>, RuntimeError> {
    let value = bigint_to_fr::<E>(bigint).ok_or_else(|| RuntimeError::ValueOverflow {
        value: bigint.clone(),
        scalar_type: scalar_type.clone(),
    })?;

    Ok(Scalar::new_constant_fr(value, scalar_type))
}

fn fr_to_bigint_signed<Fr: PrimeField>(fr: &Fr) -> BigInt {
    let mut buffer = Vec::<u8>::new();
    Fr::char()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");
    let modulus = BigInt::from_bytes_be(Sign::Plus, &buffer);
    buffer.clear();

    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");
    let value = BigInt::from_bytes_be(Sign::Plus, &buffer);

    if value < (modulus.clone().div(2)) {
        value
    } else {
        value - modulus
    }
}

fn fr_to_bigint_unsigned<Fr: PrimeField>(fr: &Fr) -> BigInt {
    let mut buffer = Vec::<u8>::new();
    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");
    BigInt::from_bytes_be(Sign::Plus, &buffer)
}

#[cfg(test)]
mod test {
    use num_bigint::BigInt;
    use num_traits::ToPrimitive;

    use ff::PrimeField;
    use franklin_crypto::bellman::pairing::bn256::Bn256;
    use franklin_crypto::bellman::pairing::bn256::Fr;

    use crate::gadgets::scalar::fr_bigint;

    #[test]
    fn test_fr_to_bigint() {
        let values = [0, 1, 2, 42, 1_234_567_890];

        for v in values.iter() {
            let fr = Fr::from_str(&v.to_string()).unwrap();
            let bigint = fr_bigint::fr_to_bigint(&fr, true);
            assert_eq!(bigint.to_i32(), Some(*v));
        }
    }

    #[test]
    fn test_bigint_to_fr() {
        let values = [0, 1, 2, 42, 1_234_567_890];

        for &v in values.iter() {
            let bigint = BigInt::from(v);
            let fr = fr_bigint::bigint_to_fr::<Bn256>(&bigint);
            assert_eq!(fr, Fr::from_str(&v.to_string()));
        }
    }

    #[test]
    fn test_negatives() {
        let values = [-1 as isize, -42, -123_456_789_098_761];

        for &v in values.iter() {
            let expected = BigInt::from(v);
            let fr = fr_bigint::bigint_to_fr::<Bn256>(&expected).expect("bigint_to_fr");
            let actual = fr_bigint::fr_to_bigint(&fr, true);
            assert_eq!(actual, expected);
        }
    }
}
