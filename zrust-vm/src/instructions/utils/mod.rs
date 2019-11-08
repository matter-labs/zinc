use num_bigint::{BigInt};
use bellman::pairing::Engine;
use ff::{PrimeField, PrimeFieldRepr};
use num_traits::Zero;
use crate::{Primitive, RuntimeError, Stack};
use crate::instructions::utils;
use franklin_crypto::bellman::{SynthesisError, ConstraintSystem};
use num_integer::Integer;

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

pub fn div_rem<E, CS>(cs: &mut CS, nominator: Primitive<E>, denominator: Primitive<E>)
    -> Result<(Primitive<E>, Primitive<E>), RuntimeError>
where
    E: Engine,
    CS: ConstraintSystem<E>
{
    let mut quotient: Option<E::Fr> = None;
    let mut remainder: Option<E::Fr> = None;

    if let (Some(nom), Some(denom)) = (nominator.value, denominator.value) {
        let nom_bi = utils::fr_to_bigint::<E>(&nom);
        let denom_bi = utils::fr_to_bigint::<E>(&denom);

        let (q, r) = nom_bi.div_rem(&denom_bi);

        quotient = utils::bigint_to_fr::<E>(&q);
        remainder = utils::bigint_to_fr::<E>(&r);
    }

    let qutioent_var = cs.alloc(
        || "qutioent",
        || quotient.ok_or(SynthesisError::AssignmentMissing))
        .map_err(|e| RuntimeError::SynthesisError)?;

    let remainder_var = cs.alloc(
        || "remainder",
        || remainder.ok_or(SynthesisError::AssignmentMissing))
        .map_err(|e| RuntimeError::SynthesisError)?;

    cs.enforce(
        || "equality",
        |lc| lc + qutioent_var,
        |lc| lc + denominator.variable,
        |lc| lc + nominator.variable - remainder_var
    );

    Ok((
        Primitive { value: quotient, variable: qutioent_var },
        Primitive { value: remainder, variable: remainder_var }
    ))
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
