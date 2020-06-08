use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::Assignment;

use zinc_bytecode::ScalarType;
use zinc_utils::math;

use crate::auto_const;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::fr_bigint;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn div_rem_conditional<E, CS>(
    mut cs: CS,
    condition: &Scalar<E>,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<(Scalar<E>, Scalar<E>), RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let denom = gadgets::select::conditional(
        cs.namespace(|| "select denominator"),
        condition,
        right,
        &Scalar::new_constant_int(1, right.get_type()),
    )?;

    auto_const!(div_rem_enforce, cs, left, &denom)
}

/// This is enforcing that `right` is not zero.
pub fn div_rem_enforce<E, CS>(
    mut cs: CS,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<(Scalar<E>, Scalar<E>), RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let nominator = left;
    let denominator = right;

    let mut quotient_value: Option<E::Fr> = None;
    let mut remainder_value: Option<E::Fr> = None;

    if let (Some(nom), Some(denom)) = (nominator.get_value(), denominator.get_value()) {
        let nom_bi = fr_bigint::fr_to_bigint(&nom, nominator.is_signed());
        let denom_bi = fr_bigint::fr_to_bigint(&denom, denominator.is_signed());

        let (q, r) =
            math::euclidean::div_rem(&nom_bi, &denom_bi).ok_or(RuntimeError::DivisionByZero)?;

        quotient_value = fr_bigint::bigint_to_fr::<E>(&q);
        remainder_value = fr_bigint::bigint_to_fr::<E>(&r);
    }

    let (quotient, remainder) = {
        let quotient_var = cs.alloc(|| "quotient", || quotient_value.grab())?;

        let remainder_var = cs.alloc(|| "remainder", || remainder_value.grab())?;

        cs.enforce(
            || "equality",
            |lc| lc + quotient_var,
            |lc| lc + &denominator.lc::<CS>(),
            |lc| lc + &nominator.lc::<CS>() - remainder_var,
        );

        let quotient =
            Scalar::new_unchecked_variable(quotient_value, quotient_var, ScalarType::Field);
        let remainder =
            Scalar::new_unchecked_variable(remainder_value, remainder_var, ScalarType::Field);

        (quotient, remainder)
    };

    let abs_denominator = gadgets::arithmetic::abs::abs(cs.namespace(|| "abs"), denominator)?;
    let lt = gadgets::comparison::lesser_than(
        cs.namespace(|| "lt"),
        &remainder.as_field(),
        &abs_denominator.as_field(),
    )?;
    let zero = Scalar::new_constant_int(0, remainder.get_type());
    let ge = gadgets::comparison::greater_or_equals(cs.namespace(|| "ge"), &remainder, &zero)?;
    cs.enforce(
        || "0 <= rem < |denominator|",
        |lc| lc + CS::one() - &lt.lc::<CS>(),
        |lc| lc + CS::one() - &ge.lc::<CS>(),
        |lc| lc,
    );

    Ok((quotient, remainder))
}
