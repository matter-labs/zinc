use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;

use crate::auto_const;
use crate::error::Error;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn bit_not<E, CS>(cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        let scalar_type = scalar.get_type();
        scalar_type.assert_signed(false)?;

        let len = scalar_type.bitlength::<E>();

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
