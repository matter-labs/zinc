use crate::gadgets::{Scalar, ScalarType};
use crate::{Engine, Result};
use ff::Field;
use franklin_crypto::bellman::ConstraintSystem;

pub fn sub<E, CS>(mut cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>>
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
