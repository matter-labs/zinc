//!
//! The `Or` instruction.
//!

use zinc_build::Or;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Or {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let or = gadgets::logical::or::or(vm.constraint_system(), &left, &right)?;

        vm.push(Cell::Value(or))
    }
}

#[cfg(test)]
mod tests {
    use zinc_build::ScalarType;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_or() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_build::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_build::Or)
            .push(zinc_build::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_build::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_build::Or)
            .push(zinc_build::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_build::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_build::Or)
            .push(zinc_build::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_build::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_build::Or)
            .test(&[1, 1, 1, 0])
    }
}
