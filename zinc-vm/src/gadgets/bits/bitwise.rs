use crate::auto_const;
use crate::error::Result;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::{Scalar, ScalarTypeExpectation};
use crate::Engine;
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;
use zinc_bytecode::ScalarType;

pub fn bit_and<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
    {
        let scalar_type = ScalarType::expect_same(left.get_type(), right.get_type())?;
        scalar_type.assert_signed(false)?;

        let len = scalar_type.bit_length::<E>();

        let left_bits = left
            .to_expression::<CS>()
            .into_bits_le_fixed(cs.namespace(|| "left bits"), len)?;

        let right_bits = right
            .to_expression::<CS>()
            .into_bits_le_fixed(cs.namespace(|| "left bits"), len)?;

        let result_bits = left_bits
            .into_iter()
            .zip(right_bits)
            .enumerate()
            .map(|(i, (l_bit, r_bit))| {
                Boolean::and(cs.namespace(|| format!("bit {}", i)), &l_bit, &r_bit)
            })
            .collect::<std::result::Result<Vec<Boolean>, SynthesisError>>()?;

        let result = AllocatedNum::pack_bits_to_element(cs.namespace(|| "result"), &result_bits)?;

        Ok(Scalar::new_unchecked_variable(
            result.get_value(),
            result.get_variable(),
            scalar_type,
        ))
    }

    auto_const!(inner, cs, left, right)
}

pub fn bit_or<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
    {
        let scalar_type = ScalarType::expect_same(left.get_type(), right.get_type())?;
        scalar_type.assert_signed(false)?;

        let len = scalar_type.bit_length::<E>();

        let left_bits = left
            .to_expression::<CS>()
            .into_bits_le_fixed(cs.namespace(|| "left bits"), len)?;

        let right_bits = right
            .to_expression::<CS>()
            .into_bits_le_fixed(cs.namespace(|| "left bits"), len)?;

        let result_bits = left_bits
            .into_iter()
            .zip(right_bits)
            .enumerate()
            .map(|(i, (l_bit, r_bit))| {
                Ok(Boolean::not(&Boolean::and(
                    cs.namespace(|| format!("bit {}", i)),
                    &Boolean::not(&l_bit),
                    &Boolean::not(&r_bit),
                )?))
            })
            .collect::<std::result::Result<Vec<Boolean>, SynthesisError>>()?;

        let result = AllocatedNum::pack_bits_to_element(cs.namespace(|| "result"), &result_bits)?;

        Ok(Scalar::new_unchecked_variable(
            result.get_value(),
            result.get_variable(),
            scalar_type,
        ))
    }

    auto_const!(inner, cs, left, right)
}

pub fn bit_xor<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
    {
        let scalar_type = ScalarType::expect_same(left.get_type(), right.get_type())?;
        scalar_type.assert_signed(false)?;

        let len = scalar_type.bit_length::<E>();

        let left_bits = left
            .to_expression::<CS>()
            .into_bits_le_fixed(cs.namespace(|| "left bits"), len)?;

        let right_bits = right
            .to_expression::<CS>()
            .into_bits_le_fixed(cs.namespace(|| "left bits"), len)?;

        let result_bits = left_bits
            .into_iter()
            .zip(right_bits)
            .enumerate()
            .map(|(i, (l_bit, r_bit))| {
                Boolean::xor(cs.namespace(|| format!("bit {}", i)), &l_bit, &r_bit)
            })
            .collect::<std::result::Result<Vec<Boolean>, SynthesisError>>()?;

        let result = AllocatedNum::pack_bits_to_element(cs.namespace(|| "result"), &result_bits)?;

        Ok(Scalar::new_unchecked_variable(
            result.get_value(),
            result.get_variable(),
            scalar_type,
        ))
    }

    auto_const!(inner, cs, left, right)
}

pub fn bit_not<E, CS>(cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
    {
        let scalar_type = scalar.get_type();
        scalar_type.assert_signed(false)?;

        let len = scalar_type.bit_length::<E>();

        let bits = scalar
            .to_expression::<CS>()
            .into_bits_le_fixed(cs.namespace(|| "left bits"), len)?;

        let result_bits = bits.iter().map(Boolean::not).collect::<Vec<_>>();

        let result = AllocatedNum::pack_bits_to_element(cs.namespace(|| "result"), &result_bits)?;

        Ok(Scalar::new_unchecked_variable(
            result.get_value(),
            result.get_variable(),
            scalar_type,
        ))
    }

    auto_const!(inner, cs, scalar)
}
