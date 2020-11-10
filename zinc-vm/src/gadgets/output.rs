//!
//! The output allocating gadget.
//!

use franklin_crypto::bellman::ConstraintSystem;

use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn output<E, CS>(mut cs: CS, element: Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let variable = cs
        .alloc_input(|| "output value", || element.grab_value())
        .map_err(Error::SynthesisError)?;

    cs.enforce(
        || "enforce output equality",
        |lc| lc + variable,
        |lc| lc + CS::one(),
        |lc| lc + &element.to_linear_combination::<CS>(),
    );

    Ok(Scalar::new_unchecked_variable(
        element.get_value(),
        variable,
        element.get_type(),
    ))
}
