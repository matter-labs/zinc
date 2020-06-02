use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Not;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Not {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let not = gadgets::logical::not::not(cs.namespace(|| "not"), &value)?;

        vm.push(Cell::Value(not))
    }
}

#[cfg(test)]
mod test {
    use zinc_bytecode::ScalarType;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_not() -> Result<(), TestingError> {
        TestRunner::new()
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Not)
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Not)
            .test(&[0, 1])
    }
}
