use crate::{Primitive, RuntimeError};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use crate::instructions::utils;
use bellman::pairing::Engine;
use num_integer::Integer;

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
        .map_err(|_| RuntimeError::SynthesisError)?;

    let remainder_var = cs.alloc(
        || "remainder",
        || remainder.ok_or(SynthesisError::AssignmentMissing))
        .map_err(|_| RuntimeError::SynthesisError)?;

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
