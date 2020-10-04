//!
//! The `Greater` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::Gt;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Gt {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let gt = gadgets::comparison::greater_than(cs.namespace(|| "gt"), &left, &right)?;

        vm.push(Cell::Value(gt))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;
    use num::One;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    use zinc_build::IntegerType;

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
