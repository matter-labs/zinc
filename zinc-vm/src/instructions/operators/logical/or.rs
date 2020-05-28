use zinc_bytecode::Or;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for Or {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let or = vm.operations().or(left, right)?;

        vm.push(Cell::Value(or))
    }
}

#[cfg(test)]
mod tests {
    use zinc_bytecode::ScalarType;

    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_or() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Or)
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Or)
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Or)
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Or)
            .test(&[1, 1, 1, 0])
    }
}
