use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::Assignment;

use crate::auto_const;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn div_rem_conditional<E, CS>(
    mut cs: CS,
    condition: &Scalar<E>,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<(Scalar<E>, Scalar<E>), Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let denom = gadgets::select::conditional(
        cs.namespace(|| "select denominator"),
        condition,
        right,
        &Scalar::new_constant_usize(1, right.get_type()),
    )?;

    auto_const!(div_rem_enforce, cs, left, &denom)
}

/// This is enforcing that `right` is not zero.
pub fn div_rem_enforce<E, CS>(
    mut cs: CS,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<(Scalar<E>, Scalar<E>), Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let nominator = left;
    let denominator = right;

    let mut quotient_value: Option<E::Fr> = None;
    let mut remainder_value: Option<E::Fr> = None;

    if let (Some(nom), Some(denom)) = (nominator.get_value(), denominator.get_value()) {
        let nom_bi = gadgets::scalar::fr_bigint::fr_to_bigint::<E>(&nom, nominator.is_signed());
        let denom_bi =
            gadgets::scalar::fr_bigint::fr_to_bigint::<E>(&denom, denominator.is_signed());

        let (q, r) =
            zinc_math::euclidean_div_rem(&nom_bi, &denom_bi).ok_or(Error::DivisionByZero)?;

        quotient_value = gadgets::scalar::fr_bigint::bigint_to_fr::<E>(&q);
        remainder_value = gadgets::scalar::fr_bigint::bigint_to_fr::<E>(&r);
    }

    let (quotient, remainder) = {
        let quotient_var = cs.alloc(|| "quotient", || quotient_value.grab())?;

        let remainder_var = cs.alloc(|| "remainder", || remainder_value.grab())?;

        cs.enforce(
            || "equality",
            |lc| lc + quotient_var,
            |lc| lc + &denominator.to_linear_combination::<CS>(),
            |lc| lc + &nominator.to_linear_combination::<CS>() - remainder_var,
        );

        let quotient = Scalar::new_unchecked_variable(
            quotient_value,
            quotient_var,
            zinc_types::ScalarType::Field,
        );
        let remainder = Scalar::new_unchecked_variable(
            remainder_value,
            remainder_var,
            zinc_types::ScalarType::Field,
        );

        (quotient, remainder)
    };

    let abs_denominator = gadgets::arithmetic::abs::abs(cs.namespace(|| "abs"), denominator)?;
    let lt = gadgets::comparison::lesser_than(
        cs.namespace(|| "lt"),
        &remainder.to_field(),
        &abs_denominator.to_field(),
    )?;
    let zero = Scalar::new_constant_usize(0, remainder.get_type());
    let ge = gadgets::comparison::greater_or_equals(cs.namespace(|| "ge"), &remainder, &zero)?;
    cs.enforce(
        || "0 <= rem < |denominator|",
        |lc| lc + CS::one() - &lt.to_linear_combination::<CS>(),
        |lc| lc + CS::one() - &ge.to_linear_combination::<CS>(),
        |lc| lc,
    );

    Ok((quotient, remainder))
}
