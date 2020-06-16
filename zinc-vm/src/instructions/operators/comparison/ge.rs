//!
//! The `GreaterOrEquals` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Ge;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Ge {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let ge = gadgets::comparison::greater_or_equals(cs.namespace(|| "ge"), &left, &right)?;

        vm.push(Cell::Value(ge))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_ge() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Push::new_field(1.into()))
            .push(zinc_bytecode::Ge)
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Ge)
            .push(zinc_bytecode::Push::new_field(1.into()))
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Ge)
            .test(&[0, 1, 1])
    }
}
