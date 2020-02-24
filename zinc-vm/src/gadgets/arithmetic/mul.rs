use ff::Field;
use crate::gadgets::{Scalar, ScalarType};
use franklin_crypto::bellman::ConstraintSystem;
use crate::{Result, Engine};

pub fn mul<E, CS>(
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
            sum.mul_assign(&right.grab_value()?);
            value = Some(sum);
            Ok(sum)
        }
    )?;

    cs.enforce(
        || "constraint",
        |lc| lc + &left.lc::<CS>(),
        |lc| lc + &right.lc::<CS>(),
        |lc| lc + variable,
    );

    Ok(Scalar::new_unchecked_variable(value, variable, ScalarType::Field))
}
