use ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::expression::Expression;
use num_bigint::BigInt;

use crate::{Engine, Result, RuntimeError};
use crate::gadgets::{Scalar, ScalarType, utils, IntegerType, ScalarTypeExpectation};

pub fn conditional_type_check<E, CS>(cs: CS, condition: &Scalar<E>, scalar: &Scalar<E>, scalar_type: ScalarType) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    condition.get_type().assert_type(ScalarType::Boolean)?;

    match scalar_type {
        ScalarType::Field => {
            // Always safe to cast into field
            Ok(scalar.as_field())
        }
        ScalarType::Boolean => {
            // Check as u1 integer, then changet type to Boolean
            let checked = conditional_type_check(cs, condition, scalar, IntegerType::BIT.into())?;
            Ok(checked.with_type_unchecked(scalar_type))
        }
        ScalarType::Integer(int_type) => {
            conditional_int_type_check(cs, condition, scalar, int_type)
        }
    }
}

fn conditional_int_type_check<E, CS>(
    mut cs: CS,
    condition: &Scalar<E>,
    scalar: &Scalar<E>,
    int_type: IntegerType
) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    // Throw runtime error if value is known.
    if let (Some(value_fr), Some(condition_fr)) = (scalar.get_value(), condition.get_value()) {
        let value = utils::fr_to_bigint(&value_fr, int_type.signed);
        if !condition_fr.is_zero() && (value < int_type.min() || value > int_type.max()) {
            return Err(RuntimeError::ValueOverflow {
                value,
                scalar_type: int_type.into()
            })
        }
    }

    // If scalar is constant and have passed the check, no need to create constraints.
    if scalar.is_constant() {
        return Ok(scalar.with_type_unchecked(int_type.into()));
    }

    let scalar_expr = scalar.to_expression::<CS>();
    let offset_expr = if !int_type.signed {
        Expression::u64::<CS>(0)
    } else {
        let offset = BigInt::from(1) << (int_type.length - 1);
        let offset_fr = utils::bigint_to_fr::<E>(&offset)
            .expect("invalid integer type length");
        Expression::constant::<CS>(offset_fr)
    };
    let zero = Expression::u64::<CS>(0);

    // If checking inside the false branch, use zero instead to avoid throwing an error.
    let condition_bool = condition.to_boolean(cs.namespace(|| "to_boolean"))?;
    let value_to_check = Expression::conditionally_select(
        cs.namespace(|| "select value to check"),
        scalar_expr + offset_expr,
        zero,
        &condition_bool
    )?;

    // If value is overflowing, `into_bits_le_fixed` will be unsatisfiable.
    let _bits = value_to_check.into_bits_le_fixed(cs.namespace(|| "into_bits"), int_type.length)?;

    Ok(scalar.with_type_unchecked(int_type.into()))
}
