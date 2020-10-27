//!
//! The `Greater` instruction.
//!

use num::bigint::ToBigInt;

use zinc_build::Gt;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Gt {
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

        vm.push(Cell::Value(Scalar::new_constant_bool(left > right)))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;
    use num::One;

    use zinc_build::IntegerType;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_gt() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Push::new(BigInt::one(), IntegerType::I8.into()))
            .push(zinc_build::Gt)
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Gt)
            .push(zinc_build::Push::new(BigInt::one(), IntegerType::I8.into()))
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Gt)
            .test(&[0, 0, 1])
    }
}
