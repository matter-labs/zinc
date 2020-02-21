use crate::gadgets::{Primitive, ScalarTypeExpectation};
use crate::{Engine, RuntimeError};

use bellman::ConstraintSystem;
use ff::Field;
use franklin_crypto::circuit::Assignment;
use zinc_bytecode::scalar::ScalarType;

pub fn inverse<E, CS>(mut cs: CS, scalar: Primitive<E>) -> Result<Primitive<E>, RuntimeError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    scalar.scalar_type.assert_type(ScalarType::Field)?;

    let value = match scalar.value {
        None => None,
        Some(value) => match value.inverse() {
            Some(inverse) => Some(inverse),
            None => return Err(RuntimeError::ZeroInversion),
        },
    };

    let variable = cs.alloc(|| "inverse", || value.grab())?;

    cs.enforce(
        || "value * inverse = 1",
        |zero| zero + scalar.variable,
        |zero| zero + variable,
        |zero| zero + CS::one(),
    );

    Ok(Primitive {
        value,
        variable,
        scalar_type: ScalarType::Field,
    })
}
