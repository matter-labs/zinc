//!
//! The `Push` instruction.
//!

use zinc_build::Push;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Push {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let value = Scalar::new_constant_bigint(self.value, self.scalar_type)?;

        vm.push(Cell::Value(value))
    }
}

#[cfg(test)]
mod tests {
    use num::BigInt;
    use num::Zero;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_push() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new_field(BigInt::zero()))
            .push(zinc_build::Push::new_field(BigInt::from(42)))
            .push(zinc_build::Push::new_field(0xABCD.into()))
            .push(zinc_build::Push::new(
                BigInt::from(-1),
                zinc_build::IntegerType::I8.into(),
            ))
            .push(zinc_build::Push::new(
                (-1000).into(),
                zinc_build::IntegerType::I16.into(),
            ))
            .test(&[-1000, -1, 0xABCD, 42, 0])
    }
}
