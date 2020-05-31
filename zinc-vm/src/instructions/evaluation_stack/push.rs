use zinc_bytecode::Push;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for Push {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm
            .gadgets()
            .constant_bigint(&self.value, self.scalar_type.to_owned())?;
        vm.push(Cell::Value(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    use zinc_bytecode::IntegerType;

    #[test]
    fn test_push() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new_field(0.into()))
            .add(zinc_bytecode::Push::new_field(42.into()))
            .add(zinc_bytecode::Push::new_field(0xABCD.into()))
            .add(zinc_bytecode::Push::new(
                (-1).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Push::new(
                (-1000).into(),
                IntegerType::I16.into(),
            ))
            .test(&[-1000, -1, 0xABCD, 42, 0])
    }
}
