use zinc_bytecode::Eq;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for Eq {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let eq = vm.gadgets().eq(left, right)?;

        vm.push(Cell::Value(eq))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_eq() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Eq)
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Eq)
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Eq)
            .test(&[0, 1, 0])
    }
}
