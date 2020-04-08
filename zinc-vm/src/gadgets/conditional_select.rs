use crate::gadgets::{Scalar, ScalarType, ScalarTypeExpectation, ScalarVariant};
use crate::{Engine, Result};
use ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::num::AllocatedNum;

pub fn conditional_select<E, CS>(
    mut cs: CS,
    condition: &Scalar<E>,
    if_true: &Scalar<E>,
    if_false: &Scalar<E>,
) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    condition.get_type().assert_type(ScalarType::Boolean)?;
    let scalar_type = ScalarType::expect_same(if_true.get_type(), if_false.get_type())?;

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
                |lc| lc + &if_true.lc::<CS>() - &if_false.lc::<CS>(),
                |lc| lc + &condition.lc::<CS>(),
                |lc| lc + num.get_variable() - &if_false.lc::<CS>(),
            );

            Ok(Scalar::new_unchecked_variable(
                num.get_value(),
                num.get_variable(),
                scalar_type,
            ))
        }
    }
}
