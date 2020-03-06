use crate::gadgets::{Scalar, ScalarTypeExpectation};
use crate::{Engine, RuntimeError};

use bellman::ConstraintSystem;
use ff::Field;
use franklin_crypto::circuit::Assignment;
use zinc_bytecode::scalar::ScalarType;

pub fn inverse<E, CS>(mut cs: CS, scalar: Scalar<E>) -> Result<Scalar<E>, RuntimeError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    scalar.get_type().assert_type(ScalarType::Field)?;

    let value = match scalar.get_value() {
        None => None,
        Some(value) => match value.inverse() {
            Some(inverse) => Some(inverse),
            None => return Err(RuntimeError::ZeroInversion),
        },
    };

    let variable = cs.alloc(|| "inverse", || value.grab())?;

    cs.enforce(
        || "value * inverse = 1",
        |zero| zero + &scalar.lc::<CS>(),
        |zero| zero + variable,
        |zero| zero + CS::one(),
    );

    Ok(Scalar::new_unchecked_variable(
        value,
        variable,
        ScalarType::Field,
    ))
}
