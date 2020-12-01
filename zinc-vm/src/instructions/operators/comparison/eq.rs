//!
//! The `Equals` instruction.
//!

use num::bigint::ToBigInt;

use zinc_types::Eq;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Eq {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let right = vm.pop()?.try_into_value()?.to_bigint().unwrap_or_default();
        let left = vm.pop()?.try_into_value()?.to_bigint().unwrap_or_default();

        vm.push(Cell::Value(Scalar::new_constant_bool(left == right)))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;
    use num::One;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_eq() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_types::Push::new_field(BigInt::one()))
            .push(zinc_types::Push::new_field(BigInt::from(2)))
            .push(zinc_types::Eq)
            .push(zinc_types::Push::new_field(BigInt::from(2)))
            .push(zinc_types::Push::new_field(BigInt::from(2)))
            .push(zinc_types::Eq)
            .push(zinc_types::Push::new_field(BigInt::from(2)))
            .push(zinc_types::Push::new_field(BigInt::one()))
            .push(zinc_types::Eq)
            .test(&[0, 1, 0])
    }
}
