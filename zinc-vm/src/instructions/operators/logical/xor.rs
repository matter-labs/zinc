//!
//! The `Xor` instruction.
//!

use zinc_bytecode::Xor;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Xor {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let xor = gadgets::logical::xor::xor(vm.constraint_system(), &left, &right)?;

        vm.push(Cell::Value(xor))
    }
}

#[cfg(test)]
mod tests {
    use zinc_bytecode::ScalarType;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_xor() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Xor)
            .push(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Xor)
            .push(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Xor)
            .push(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Xor)
            .test(&[0, 1, 1, 0])
    }
}
