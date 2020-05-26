use crate::auto_const;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::Scalar;
use crate::{Engine, Result};
use ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::ScalarType;

pub fn mul<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    pub fn mul_inner<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
    {
        let mut value = None;

        let variable = cs.alloc(
            || "variable",
            || {
                let mut sum = left.grab_value()?;
                sum.mul_assign(&right.grab_value()?);
                value = Some(sum);
                Ok(sum)
            },
        )?;

        cs.enforce(
            || "constraint",
            |lc| lc + &left.lc::<CS>(),
            |lc| lc + &right.lc::<CS>(),
            |lc| lc + variable,
        );

        Ok(Scalar::new_unchecked_variable(
            value,
            variable,
            ScalarType::Field,
        ))
    }

    auto_const!(mul_inner, cs, left, right)
}
