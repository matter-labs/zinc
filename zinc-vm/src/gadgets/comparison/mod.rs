use crate::{Result, Engine, RuntimeError};
use crate::gadgets::{Scalar, ScalarTypeExpectation, utils};
use ff::PrimeField;
use zinc_bytecode::scalar::ScalarType;
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use num_bigint::BigInt;
use franklin_crypto::circuit::expression::Expression;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::circuit::boolean::Boolean;

pub fn less_than<E, CS>(mut cs: CS, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    let scalar_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

    match scalar_type {
        ScalarType::Field => less_than_field(cs, left, right),
        ScalarType::Integer(int_type) => {
            let boolean = less_than_integer(cs.namespace(|| "less_than_integer"), int_type.length, left, right)?;
            Scalar::from_boolean(cs.namespace(|| "from_boolean"), boolean)
        },
        ScalarType::Boolean => Err(RuntimeError::TypeError {
            expected: "field or integer type".into(),
            actual: "boolean".to_string()
        }),
    }
}

fn less_than_field<E, CS>(mut cs: CS, left: Scalar<E>, right: Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    let expr_a = left.to_expression::<CS>();
    let expr_b = right.to_expression::<CS>();

    let bits_a = expr_a.into_bits_le_strict(cs.namespace(|| "a representation"))?;
    let bits_b = expr_b.into_bits_le_strict(cs.namespace(|| "b representation"))?;

    let lower_bits_len: usize = E::Fr::NUM_BITS as usize / 2;
    let upper_bits_len: usize = E::Fr::NUM_BITS as usize - lower_bits_len;

    let a_lower = AllocatedNum::pack_bits_to_element(
        cs.namespace(|| "a_lower"),
        &bits_a[..lower_bits_len])?;
    let b_lower = AllocatedNum::pack_bits_to_element(
        cs.namespace(|| "b_lower"),
        &bits_b[..lower_bits_len])?;

    let a_upper = AllocatedNum::pack_bits_to_element(
        cs.namespace(|| "a_upper"),
        &bits_a[lower_bits_len..])?;
    let b_upper = AllocatedNum::pack_bits_to_element(
        cs.namespace(|| "b_upper"),
        &bits_b[lower_bits_len..])?;

    let upper_a_lt_b = less_than_integer(
        cs.namespace(|| "upper_a_lt_b"),
        upper_bits_len,
        a_upper.clone().into(),
        b_upper.clone().into())?;

    let lower_a_lt_b = less_than_integer(
        cs.namespace(|| "lower_a_lt_b"),
        lower_bits_len,
        a_lower.into(),
        b_lower.into())?;

    let upper_a_eq_b = AllocatedNum::equals(
        cs.namespace(|| "upper_a_eq_b"),
        &a_upper,
        &b_upper)?;

    let lower_lt_and_upper_eq = Boolean::and(
        cs.namespace(|| ""),
        &lower_a_lt_b,
        &upper_a_eq_b.into())?;

    let res = boolean_or(cs.namespace(|| "lt"), &upper_a_lt_b, &lower_lt_and_upper_eq)?;
    Scalar::from_boolean(cs.namespace(|| "from_boolean"), res)
}

fn less_than_integer<E, CS>(mut cs: CS, length: usize, left: Scalar<E>, right: Scalar<E>) -> Result<Boolean>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    assert!(length < E::Fr::CAPACITY as usize);
    let base_bigint = (BigInt::from(1) << length) - BigInt::from(1);
    let base = utils::bigint_to_fr::<E>(&base_bigint).unwrap();

    let expr = Expression::constant::<CS>(base) - left.to_expression::<CS>() + right.to_expression::<CS>();
    let bits = expr.into_bits_le_fixed(cs.namespace(|| "into_bits_le_fixed"), length + 1)?;

    Ok(bits.last().unwrap().clone())
}

fn boolean_or<E: Engine, CS: ConstraintSystem<E>>(
    mut cs: CS,
    left: &Boolean,
    right: &Boolean,
) -> std::result::Result<Boolean, SynthesisError> {
    Ok(Boolean::and(
        cs.namespace(|| "and"),
        &left.not(),
        &right.not()
    )?.not())
}
