//!
//! The `Xor` instruction.
//!

use num::bigint::ToBigInt;
use num::Zero;

use zinc_types::Xor;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Xor {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let right = vm.pop()?.try_into_value()?.to_bigint().unwrap_or_default();
        let left = vm.pop()?.try_into_value()?.to_bigint().unwrap_or_default();

        vm.push(Cell::Value(Scalar::new_constant_bool(
            left.is_zero() != right.is_zero(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use num::BigInt;
    use num::One;
    use num::Zero;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_xor() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_types::Push::new(
                BigInt::zero(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Push::new(
                BigInt::zero(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Xor)
            .push(zinc_types::Push::new(
                BigInt::zero(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Push::new(
                BigInt::one(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Xor)
            .push(zinc_types::Push::new(
                BigInt::one(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Push::new(
                BigInt::zero(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Xor)
            .push(zinc_types::Push::new(
                BigInt::one(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Push::new(
                BigInt::one(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Xor)
            .test(&[0, 1, 1, 0])
    }
}
