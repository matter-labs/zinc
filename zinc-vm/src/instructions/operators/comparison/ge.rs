use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Ge;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;

impl<VM: VirtualMachine> VMInstruction<VM> for Ge {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let cs = vm.constraint_system();
        let ge = gadgets::comparison::ge(cs.namespace(|| "ge"), &left, &right)?;

        vm.push(Cell::Value(ge))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_ge() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Ge)
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Ge)
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Ge)
            .test(&[0, 1, 1])
    }
}
