//!
//! The conditional select gadget.
//!

use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::num::AllocatedNum;

use crate::error::Error;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::variant::Variant as ScalarVariant;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

///
/// Select single value from array based on index bits.
///
/// **Note**: index bits are in **big-endian**.
///
pub fn recursive<E, CS>(
    mut cs: CS,
    index_bits_be: &[Scalar<E>],
    array: &[Scalar<E>],
) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    assert!(!array.is_empty(), "internal error in recursive_select 1");

    if array.len() == 1 {
        return Ok(array[0].clone());
    }

    assert!(
        !index_bits_be.is_empty(),
        "internal error in recursive_select 3"
    );

    // Skip unneeded upper bits, so we can always use the first bit for conditional select.
    let extra_bits = index_bits_be.len() - zinc_math::log2ceil(array.len());
    let index_bits_be = &index_bits_be[extra_bits..];

    let half = zinc_math::floor_to_power_of_two(array.len() - 1);
    let left = recursive(
        cs.namespace(|| "left recursion"),
        &index_bits_be[1..],
        &array[..half],
    )?;
    let right = recursive(
        cs.namespace(|| "right recursion"),
        &index_bits_be[1..],
        &array[half..],
    )?;

    conditional(cs.namespace(|| "select"), &index_bits_be[0], &right, &left)
}

pub fn conditional<E, CS>(
    mut cs: CS,
    condition: &Scalar<E>,
    if_true: &Scalar<E>,
    if_false: &Scalar<E>,
) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    condition
        .get_type()
        .assert_type(zinc_types::ScalarType::Boolean)?;
    let scalar_type = zinc_types::ScalarType::expect_same(if_true.get_type(), if_false.get_type())?;

    match condition.get_variant() {
        ScalarVariant::Constant(constant) => {
            if constant.value.is_zero() {
                Ok(if_false.clone())
            } else {
                Ok(if_true.clone())
            }
        }
        ScalarVariant::Variable(_) => {
            let num = AllocatedNum::alloc(cs.namespace(|| "selected"), || {
                if !condition.grab_value()?.is_zero() {
                    if_true.grab_value()
                } else {
                    if_false.grab_value()
                }
            })?;

            // Selected, Right, Left, Condition
            // s = r + c * (l - r)
            // (l - r) * (c) = (s - r)
            cs.enforce(
                || "constraint",
                |lc| {
                    lc + &if_true.to_linear_combination::<CS>()
                        - &if_false.to_linear_combination::<CS>()
                },
                |lc| lc + &condition.to_linear_combination::<CS>(),
                |lc| lc + num.get_variable() - &if_false.to_linear_combination::<CS>(),
            );

            Ok(Scalar::new_unchecked_variable(
                num.get_value(),
                num.get_variable(),
                scalar_type,
            ))
        }
    }
}
