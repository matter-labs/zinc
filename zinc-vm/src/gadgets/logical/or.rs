use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::num::AllocatedNum;

use zinc_build::ScalarType;

use crate::auto_const;
use crate::error::RuntimeError;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn or<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(
        mut cs: CS,
        left: &Scalar<E>,
        right: &Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        left.get_type().assert_type(ScalarType::Boolean)?;
        right.get_type().assert_type(ScalarType::Boolean)?;

        let num = AllocatedNum::alloc(cs.namespace(|| "value"), || {
            let l = left.grab_value()?;
            let r = right.grab_value()?;
            if l.is_zero() && r.is_zero() {
                Ok(E::Fr::zero())
            } else {
                Ok(E::Fr::one())
            }
        })?;

        cs.enforce(
            || "equality",
            |lc| lc + CS::one() - &left.to_linear_combination::<CS>(),
            |lc| lc + CS::one() - &right.to_linear_combination::<CS>(),
            |lc| lc + CS::one() - num.get_variable(),
        );

        Ok(Scalar::new_unchecked_variable(
            num.get_value(),
            num.get_variable(),
            ScalarType::Boolean,
        ))
    }

    auto_const!(inner, cs, left, right)
}
