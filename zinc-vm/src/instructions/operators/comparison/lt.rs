//!
//! The `Lesser` instruction.
//!

use num::bigint::ToBigInt;

use zinc_build::Lt;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Lt {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm
            .pop()?
            .try_into_value()?
            .to_bigint()
            .expect(zinc_const::panic::DATA_CONVERSION);
        let left = vm
            .pop()?
            .try_into_value()?
            .to_bigint()
            .expect(zinc_const::panic::DATA_CONVERSION);

        vm.push(Cell::Value(Scalar::new_constant_bool(left < right)))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;
    use num::One;
    use num::Zero;

    use franklin_crypto::bellman::pairing::bn256::Bn256;
    use franklin_crypto::bellman::pairing::bn256::Fr;
    use franklin_crypto::bellman::pairing::ff::Field;

    use zinc_build::IntegerType;
    use zinc_build::ScalarType;

    use crate::gadgets;
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn simple() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Push::new(BigInt::one(), IntegerType::I8.into()))
            .push(zinc_build::Lt)
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Lt)
            .push(zinc_build::Push::new(BigInt::one(), IntegerType::I8.into()))
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Lt)
            .test(&[1, 0, 0])
    }

    #[test]
    fn edge_cases() -> Result<(), TestingError> {
        let mut max_fr = Fr::zero();
        max_fr.sub_assign(&Fr::one());
        let max = gadgets::scalar::fr_bigint::fr_to_bigint::<Bn256>(&max_fr, false);

        TestRunner::new()
            .push(zinc_build::Push::new(max.clone(), ScalarType::Field))
            .push(zinc_build::Push::new(BigInt::zero(), ScalarType::Field))
            .push(zinc_build::Lt)
            .push(zinc_build::Push::new(BigInt::zero(), ScalarType::Field))
            .push(zinc_build::Push::new(max.clone(), ScalarType::Field))
            .push(zinc_build::Lt)
            .push(zinc_build::Push::new(BigInt::one(), ScalarType::Field))
            .push(zinc_build::Push::new(max.clone(), ScalarType::Field))
            .push(zinc_build::Lt)
            .test(&[1, 1, 0])
    }
}
