use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;

use crate::auto_const;
use crate::error::Error;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn bit_or<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, Error>
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
        scalar_type.assert_signed(false)?;

        let len = scalar_type.bitlength::<E>();

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
            .collect::<Result<Vec<Boolean>, SynthesisError>>()?;

        let result = AllocatedNum::pack_bits_to_element(cs.namespace(|| "result"), &result_bits)?;

        Ok(Scalar::new_unchecked_variable(
            result.get_value(),
            result.get_variable(),
            scalar_type,
        ))
    }

    auto_const!(inner, cs, left, right)
}
