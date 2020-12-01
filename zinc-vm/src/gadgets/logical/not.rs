use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::expression::Expression;

use crate::auto_const;
use crate::error::Error;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn not<E, CS>(cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        scalar
            .get_type()
            .assert_type(zinc_types::ScalarType::Boolean)?;

        let one_expr = Expression::u64::<CS>(1);
        let not_expr = one_expr - scalar.to_expression::<CS>();

        let not_num = not_expr.into_number(cs.namespace(|| "not_num"))?;

        Ok(Scalar::new_unchecked_variable(
            not_num.get_value(),
            not_num.get_variable(),
            zinc_types::ScalarType::Boolean,
        ))
    }

    auto_const!(inner, cs, scalar)
}
