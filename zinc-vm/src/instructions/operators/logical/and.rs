use zinc_bytecode::And;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for And {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let and = vm.operations().and(left, right)?;

        vm.push(Cell::Value(and))
    }
}

#[cfg(test)]
mod tests {
    use zinc_bytecode::ScalarType;

    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_and() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::And)
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::And)
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::And)
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::And)
            .test(&[1, 0, 0, 0])
    }
}
