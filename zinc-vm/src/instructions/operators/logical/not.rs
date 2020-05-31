use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Not;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;

impl<VM: VirtualMachine> VMInstruction<VM> for Not {
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

    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_not() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Not)
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Not)
            .test(&[0, 1])
    }
}
