//!
//! The `Exit` instruction.
//!

use zinc_build::Exit;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Exit {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.exit(self.output_size)?;

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
            .push(zinc_build::Push::new_field(1.into()))
            .push(zinc_build::Exit::new(0))
            .push(zinc_build::Push::new_field(2.into()))
            .test(&[1])
    }
}
