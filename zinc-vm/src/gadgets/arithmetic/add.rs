use ff::Field;
use crate::gadgets::{Scalar, ScalarType};
use franklin_crypto::bellman::ConstraintSystem;
use crate::{Result, Engine};

pub fn add<E, CS>(
    mut cs: CS,
    left: &Scalar<E>,
    right: &Scalar<E>,
) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    let mut value = None;

    let variable = cs.alloc(
        || "variable",
        || {
            let mut sum = left.grab_value()?;
            sum.add_assign(&right.grab_value()?);
            value = Some(sum);
            Ok(sum)
        }
    )?;

    cs.enforce(
        || "constraint",
        |lc| lc + &left.lc::<CS>() + &right.lc::<CS>(),
        |lc| lc + CS::one(),
        |lc| lc + variable,
    );

    Ok(Scalar::new_unchecked_variable(value, variable, ScalarType::Field))
}
