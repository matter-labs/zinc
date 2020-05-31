use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Ne;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;

impl<VM: VirtualMachine> VMInstruction<VM> for Ne {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let ne = gadgets::comparison::not_equals(cs.namespace(|| "ne"), &left, &right)?;

        vm.push(Cell::Value(ne))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_ne() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Ne)
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Ne)
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Ne)
            .test(&[1, 0, 1])
    }
}
