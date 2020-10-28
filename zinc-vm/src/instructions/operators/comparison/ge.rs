//!
//! The `GreaterOrEquals` instruction.
//!

use num::bigint::ToBigInt;

use zinc_build::Ge;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Ge {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?.to_bigint().unwrap_or_default();
        let left = vm.pop()?.try_into_value()?.to_bigint().unwrap_or_default();

        vm.push(Cell::Value(Scalar::new_constant_bool(left >= right)))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;
    use num::One;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_ge() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new_field(BigInt::from(2)))
            .push(zinc_build::Push::new_field(BigInt::one()))
            .push(zinc_build::Ge)
            .push(zinc_build::Push::new_field(BigInt::from(2)))
            .push(zinc_build::Push::new_field(BigInt::from(2)))
            .push(zinc_build::Ge)
            .push(zinc_build::Push::new_field(BigInt::one()))
            .push(zinc_build::Push::new_field(BigInt::from(2)))
            .push(zinc_build::Ge)
            .test(&[0, 1, 1])
    }
}
