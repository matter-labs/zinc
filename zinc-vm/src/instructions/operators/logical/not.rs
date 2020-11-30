//!
//! The `Not` instruction.
//!

use num::bigint::ToBigInt;
use num::Zero;

use zinc_build::Not;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Not {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let value = vm.pop()?.try_into_value()?.to_bigint().unwrap_or_default();

        vm.push(Cell::Value(Scalar::new_constant_bool(value.is_zero())))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;
    use num::One;
    use num::Zero;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_not() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new(
                BigInt::zero(),
                zinc_build::ScalarType::Boolean,
            ))
            .push(zinc_build::Not)
            .push(zinc_build::Push::new(
                BigInt::one(),
                zinc_build::ScalarType::Boolean,
            ))
            .push(zinc_build::Not)
            .test(&[0, 1])
    }
}
