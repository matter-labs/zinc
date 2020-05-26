use crate::auto_const;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::Scalar;
use crate::{gadgets, Engine, Result};
use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::IntegerType;
use zinc_bytecode::ScalarType;

pub fn abs<E, CS>(cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
    {
        match scalar.get_type() {
            ScalarType::Field | ScalarType::Boolean => Ok(scalar.clone()),

            ScalarType::Integer(int_type) => {
                if !int_type.is_signed {
                    return Ok(scalar.clone());
                }

                let scalar_type = ScalarType::Integer(IntegerType {
                    is_signed: true,
                    bitlength: int_type.bitlength + 1,
                });

                let scalar = scalar.with_type_unchecked(scalar_type.clone());
                let zero = Scalar::new_constant_int(0, scalar_type);
                let neg = gadgets::neg(cs.namespace(|| "neg"), &scalar)?;
                let lt0 = gadgets::lt(cs.namespace(|| "lt"), &scalar, &zero)?;
                gadgets::conditional_select(
                    cs.namespace(|| "select"),
                    &lt0,
                    &neg.as_field(),
                    &scalar.as_field(),
                )
            }
        }
    }

    auto_const!(inner, cs, scalar)
}
