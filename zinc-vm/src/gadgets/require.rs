//!
//! The `require` gadget.
//!

use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::Assignment;

use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn require<E, CS>(mut cs: CS, element: Scalar<E>, message: Option<&str>) -> Result<(), Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    if let Some(value) = element.get_value() {
        if value.is_zero() {
            let s = message.unwrap_or("<no message>");
            return Err(Error::RequireError(s.into()));
        }
    }

    let inverse_value = element
        .get_value()
        .map(|fr| fr.inverse().unwrap_or_else(E::Fr::zero));

    let inverse_variable = cs
        .alloc(|| "inverse", || inverse_value.grab())
        .map_err(Error::SynthesisError)?;

    cs.enforce(
        || "require",
        |lc| lc + &element.to_linear_combination::<CS>(),
        |lc| lc + inverse_variable,
        |lc| lc + CS::one(),
    );

    Ok(())
}
