use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::circuit::Assignment;

use crate::auto_const;
use crate::error::Error;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn inverse<E, CS>(cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(mut cs: CS, scalar: &Scalar<E>) -> Result<Scalar<E>, Error>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        let expr = scalar.to_expression::<CS>();

        let inverse = AllocatedNum::alloc(cs.namespace(|| "inverse"), || {
            expr.get_value()
                .grab()?
                .inverse()
                .ok_or(SynthesisError::Unsatisfiable)
        })?;

        cs.enforce(
            || "inverse constraint",
            |zero| zero + &scalar.to_linear_combination::<CS>(),
            |zero| zero + inverse.get_variable(),
            |zero| zero + CS::one(),
        );

        Ok(inverse.into())
    }

    auto_const!(inner, cs, scalar)
}

#[cfg(test)]
mod tests {
    use franklin_crypto::bellman::pairing::bn256::Bn256;
    use franklin_crypto::bellman::pairing::bn256::Fr;
    use franklin_crypto::bellman::pairing::ff::Field;
    use franklin_crypto::bellman::ConstraintSystem;
    use franklin_crypto::circuit::test::TestConstraintSystem;

    use crate::gadgets;
    use crate::gadgets::scalar::Scalar;

    #[test]
    fn test_inverse() {
        let mut cs = TestConstraintSystem::<Bn256>::new();

        let zero = Scalar::new_constant_usize(0, zinc_types::ScalarType::Field);
        let one = Scalar::new_constant_usize(1, zinc_types::ScalarType::Field);

        assert!(
            gadgets::arithmetic::field::inverse(cs.namespace(|| "zero"), &zero).is_err(),
            "zero"
        );
        assert_eq!(
            gadgets::arithmetic::field::inverse(cs.namespace(|| "one"), &one)
                .expect(zinc_const::panic::TEST_DATA_VALID)
                .get_value()
                .expect(zinc_const::panic::TEST_DATA_VALID),
            Fr::one(),
            "one"
        );
    }
}
