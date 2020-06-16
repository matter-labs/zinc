//!
//! The `And` instruction.
//!

use zinc_bytecode::And;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for And {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let and = gadgets::logical::and::and(vm.constraint_system(), &left, &right)?;

        vm.push(Cell::Value(and))
    }
}

#[cfg(test)]
mod tests {
    use zinc_bytecode::ScalarType;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_and() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_bytecode::And)
            .push(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_bytecode::And)
            .push(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .push(zinc_bytecode::And)
            .push(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .push(zinc_bytecode::And)
            .test(&[1, 0, 0, 0])
    }
}
