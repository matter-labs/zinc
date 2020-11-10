use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::expression::Expression;

use crate::auto_const;
use crate::error::Error;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn neg<E, CS>(cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        let expr = Expression::u64::<CS>(0) - scalar.to_expression::<CS>();
        let num = expr.into_number(cs.namespace(|| "into_number"))?;
        Ok(num.into())
    }

    auto_const!(inner, cs, scalar)
}
