use crate::auto_const;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::{Scalar};
use crate::{Engine, Result};
use ff::Field;
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use franklin_crypto::circuit::Assignment;
use franklin_crypto::circuit::num::AllocatedNum;

pub fn inverse<E, CS>(cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>>
        where
            E: Engine,
            CS: ConstraintSystem<E>,
    {
        let expr = scalar.to_expression::<CS>();

        let inverse = AllocatedNum::alloc(
            cs.namespace(|| "inverse"),
            || expr
                .get_value()
                .grab()?
                .inverse()
                .ok_or(SynthesisError::Unsatisfiable)
        )?;

        cs.enforce(
            || "inverse constraint",
            |zero| zero + &scalar.lc::<CS>(),
            |zero| zero + inverse.get_variable(),
            |zero| zero + CS::one(),
        );

        Ok(inverse.into())
    }

    auto_const!(inner, cs, scalar)
}


#[cfg(test)]
mod tests {
    use super::*;

    use ff::Field;
    use pairing::bn256::{Bn256, Fr};
    use bellman::ConstraintSystem;
    use franklin_crypto::circuit::test::TestConstraintSystem;

    use crate::gadgets::Scalar;
    use zinc_bytecode::scalar::ScalarType;

    #[test]
    fn test_inverse() {
        let mut cs = TestConstraintSystem::<Bn256>::new();

        let zero = Scalar::new_constant_int(0, ScalarType::Field);
        let one = Scalar::new_constant_int(1, ScalarType::Field);

        assert!(inverse(cs.namespace(|| "zero"), &zero).is_err(), "zero");
        assert_eq!(inverse(cs.namespace(|| "one"), &one).unwrap().get_value().unwrap(), Fr::one(), "one");
    }
}
