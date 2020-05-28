use crate::auto_const;
use crate::error::Result;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::{Scalar, ScalarTypeExpectation};
use crate::Engine;
use ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::num::AllocatedNum;
use zinc_bytecode::ScalarType;

pub fn and<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
    {
        left.get_type().assert_type(ScalarType::Boolean)?;
        right.get_type().assert_type(ScalarType::Boolean)?;

        let num = AllocatedNum::alloc(cs.namespace(|| "value"), || {
            let mut conj = left.grab_value()?;
            conj.mul_assign(&right.grab_value()?);
            Ok(conj)
        })?;

        cs.enforce(
            || "equality",
            |lc| lc + &left.lc::<CS>(),
            |lc| lc + &right.lc::<CS>(),
            |lc| lc + num.get_variable(),
        );

        Ok(Scalar::new_unchecked_variable(
            num.get_value(),
            num.get_variable(),
            ScalarType::Boolean,
        ))
    }

    auto_const!(inner, cs, left, right)
}
