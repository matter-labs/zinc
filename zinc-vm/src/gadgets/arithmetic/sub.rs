use crate::auto_const;
use crate::error::Result;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::Scalar;
use crate::Engine;
use ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::ScalarType;

pub fn sub<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    pub fn sub_inner<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
    {
        let mut value = None;

        let variable = cs.alloc(
            || "variable",
            || {
                let mut tmp = left.grab_value()?;
                tmp.sub_assign(&right.grab_value()?);
                value = Some(tmp);
                Ok(tmp)
            },
        )?;

        cs.enforce(
            || "constraint",
            |lc| lc + &left.lc::<CS>() - &right.lc::<CS>(),
            |lc| lc + CS::one(),
            |lc| lc + variable,
        );

        Ok(Scalar::new_unchecked_variable(
            value,
            variable,
            ScalarType::Field,
        ))
    }

    auto_const!(sub_inner, cs, left, right)
}
