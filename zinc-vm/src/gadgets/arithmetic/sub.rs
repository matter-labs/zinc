use ff::Field;
use crate::gadgets::{Scalar, ScalarType};
use franklin_crypto::bellman::ConstraintSystem;
use crate::{Result, Engine};

pub fn sub<E, CS>(
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
            let mut tmp = left.grab_value()?;
            tmp.sub_assign(&right.grab_value()?);
            value = Some(tmp);
            Ok(tmp)
        }
    )?;

    cs.enforce(
        || "constraint",
        |lc| lc + &left.lc::<CS>() - &right.lc::<CS>(),
        |lc| lc + CS::one(),
        |lc| lc + variable,
    );

    Ok(Scalar::new_unchecked_variable(value, variable, ScalarType::Field))
}
