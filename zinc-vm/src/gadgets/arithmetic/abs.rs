use franklin_crypto::bellman::ConstraintSystem;

use crate::auto_const;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn abs<E, CS>(cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        match scalar.get_type() {
            zinc_types::ScalarType::Field | zinc_types::ScalarType::Boolean => Ok(scalar.clone()),

            zinc_types::ScalarType::Integer(int_type) => {
                if !int_type.is_signed {
                    return Ok(scalar.clone());
                }

                let scalar_type = zinc_types::ScalarType::Integer(zinc_types::IntegerType {
                    is_signed: true,
                    bitlength: int_type.bitlength + 1,
                });

                let scalar = scalar.to_type_unchecked(scalar_type.clone());
                let zero = Scalar::new_constant_usize(0, scalar_type);
                let neg = gadgets::arithmetic::neg::neg(cs.namespace(|| "neg"), &scalar)?;
                let lt0 = gadgets::comparison::lesser_than(cs.namespace(|| "lt"), &scalar, &zero)?;

                gadgets::select::conditional(
                    cs.namespace(|| "select"),
                    &lt0,
                    &neg.to_field(),
                    &scalar.to_field(),
                )
            }
        }
    }

    auto_const!(inner, cs, scalar)
}
