use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::num::AllocatedNum;

use crate::auto_const;
use crate::error::Error;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn and<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        left.get_type()
            .assert_type(zinc_types::ScalarType::Boolean)?;
        right
            .get_type()
            .assert_type(zinc_types::ScalarType::Boolean)?;

        let num = AllocatedNum::alloc(cs.namespace(|| "value"), || {
            let mut conj = left.grab_value()?;
            conj.mul_assign(&right.grab_value()?);
            Ok(conj)
        })?;

        cs.enforce(
            || "equality",
            |lc| lc + &left.to_linear_combination::<CS>(),
            |lc| lc + &right.to_linear_combination::<CS>(),
            |lc| lc + num.get_variable(),
        );

        Ok(Scalar::new_unchecked_variable(
            num.get_value(),
            num.get_variable(),
            zinc_types::ScalarType::Boolean,
        ))
    }

    auto_const!(inner, cs, left, right)
}
