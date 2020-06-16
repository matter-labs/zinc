//!
//! The `NotEquals` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Ne;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Ne {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let ne = gadgets::comparison::not_equals(cs.namespace(|| "ne"), &left, &right)?;

        vm.push(Cell::Value(ne))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_ne() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_bytecode::Push::new_field(1.into()))
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Ne)
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Ne)
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Push::new_field(1.into()))
            .push(zinc_bytecode::Ne)
            .test(&[1, 0, 1])
    }
}
