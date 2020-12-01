use num::BigInt;

use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::expression::Expression;
use franklin_crypto::circuit::num::AllocatedNum;

use crate::auto_const;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn greater_than<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    lesser_than(cs, right, left)
}

pub fn greater_or_equals<E, CS>(
    cs: CS,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    lesser_or_equals(cs, right, left)
}

pub fn lesser_or_equals<E, CS>(
    mut cs: CS,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let is_gt = greater_than(cs.namespace(|| "gt"), left, right)?;
    gadgets::logical::not::not(cs.namespace(|| "not"), &is_gt)
}

pub fn lesser_than<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        let scalar_type = zinc_types::ScalarType::expect_same(left.get_type(), right.get_type())?;

        match scalar_type {
            zinc_types::ScalarType::Field => less_than_field(cs, left, right),
            zinc_types::ScalarType::Integer(int_type) => {
                let boolean = less_than_integer(
                    cs.namespace(|| "less_than_integer"),
                    int_type.bitlength,
                    left,
                    right,
                )?;
                Scalar::from_boolean(cs.namespace(|| "from_boolean"), boolean)
            }
            r#type @ zinc_types::ScalarType::Boolean => Err(Error::TypeError {
                expected: "field or integer type".into(),
                found: r#type.to_string(),
            }),
        }
    }

    auto_const!(inner, cs, left, right)
}

fn less_than_field<E, CS>(
    mut cs: CS,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let expr_a = left.to_expression::<CS>();
    let expr_b = right.to_expression::<CS>();

    let bits_a = expr_a.into_bits_le_strict(cs.namespace(|| "a representation"))?;
    let bits_b = expr_b.into_bits_le_strict(cs.namespace(|| "b representation"))?;

    let lower_bits_len: usize = E::Fr::NUM_BITS as usize / 2;
    let upper_bits_len: usize = E::Fr::NUM_BITS as usize - lower_bits_len;

    let a_lower =
        AllocatedNum::pack_bits_to_element(cs.namespace(|| "a_lower"), &bits_a[..lower_bits_len])?;
    let b_lower =
        AllocatedNum::pack_bits_to_element(cs.namespace(|| "b_lower"), &bits_b[..lower_bits_len])?;

    let a_upper =
        AllocatedNum::pack_bits_to_element(cs.namespace(|| "a_upper"), &bits_a[lower_bits_len..])?;
    let b_upper =
        AllocatedNum::pack_bits_to_element(cs.namespace(|| "b_upper"), &bits_b[lower_bits_len..])?;

    let upper_a_lt_b = less_than_integer(
        cs.namespace(|| "upper_a_lt_b"),
        upper_bits_len,
        &a_upper.clone().into(),
        &b_upper.clone().into(),
    )?;

    let lower_a_lt_b = less_than_integer(
        cs.namespace(|| "lower_a_lt_b"),
        lower_bits_len,
        &a_lower.into(),
        &b_lower.into(),
    )?;

    let upper_a_eq_b = AllocatedNum::equals(cs.namespace(|| "upper_a_eq_b"), &a_upper, &b_upper)?;

    let lower_lt_and_upper_eq =
        Boolean::and(cs.namespace(|| ""), &lower_a_lt_b, &upper_a_eq_b.into())?;

    let res = Boolean::and(
        cs.namespace(|| "and"),
        &upper_a_lt_b.not(),
        &lower_lt_and_upper_eq.not(),
    )?
    .not();
    Scalar::from_boolean(cs.namespace(|| "from_boolean"), res)
}

fn less_than_integer<E, CS>(
    mut cs: CS,
    length: usize,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<Boolean, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    assert!(length < E::Fr::CAPACITY as usize);
    let base_bigint = (BigInt::from(1) << length) - BigInt::from(1);
    let base = gadgets::scalar::fr_bigint::bigint_to_fr::<E>(&base_bigint)
        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

    let expr =
        Expression::constant::<CS>(base) - left.to_expression::<CS>() + right.to_expression::<CS>();
    let bits = expr.into_bits_le_fixed(cs.namespace(|| "into_bits_le_fixed"), length + 1)?;

    Ok(bits
        .last()
        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
        .clone())
}

pub fn equals<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        let le = left.to_expression::<CS>();
        let re = right.to_expression::<CS>();

        let eq = Expression::equals(cs.namespace(|| "equals"), le, re)?;

        Scalar::from_boolean(cs.namespace(|| "scalar"), Boolean::from(eq))
    }

    auto_const!(inner, cs, left, right)
}

pub fn not_equals<E, CS>(
    mut cs: CS,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let t = equals(cs.namespace(|| "eq"), left, right)?;
    gadgets::logical::not::not(cs.namespace(|| "not"), &t)
}
