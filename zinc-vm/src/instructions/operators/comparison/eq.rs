//!
//! The `Equals` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Eq;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Eq {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let eq = gadgets::comparison::equals(cs.namespace(|| "eq"), &left, &right)?;

        vm.push(Cell::Value(eq))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_eq() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_bytecode::Push::new_field(1.into()))
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Eq)
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Eq)
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Push::new_field(1.into()))
            .push(zinc_bytecode::Eq)
            .test(&[0, 1, 0])
    }
}
