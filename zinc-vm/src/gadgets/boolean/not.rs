use crate::auto_const;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::{Scalar, ScalarTypeExpectation};
use crate::{Engine, Result};
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::expression::Expression;
use zinc_bytecode::ScalarType;

pub fn not<E, CS>(cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
    {
        scalar.get_type().assert_type(ScalarType::Boolean)?;

        let one_expr = Expression::u64::<CS>(1);
        let not_expr = one_expr - scalar.to_expression::<CS>();

        let not_num = not_expr.into_number(cs.namespace(|| "not_num"))?;

        Ok(Scalar::new_unchecked_variable(
            not_num.get_value(),
            not_num.get_variable(),
            ScalarType::Boolean,
        ))
    }

    auto_const!(inner, cs, scalar)
}
