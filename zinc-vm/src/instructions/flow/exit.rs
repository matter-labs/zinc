use zinc_bytecode::Exit;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Exit {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.exit(self.outputs_count)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_exit() -> Result<(), TestingError> {
        TestRunner::new()
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Exit::new(0))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .test(&[1])
    }
}
