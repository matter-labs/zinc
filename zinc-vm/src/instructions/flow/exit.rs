use zinc_bytecode::Exit;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for Exit {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.exit(self.outputs_count)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_exit() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Exit::new(0))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .test(&[1])
    }
}
