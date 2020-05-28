use crate::core::RuntimeError;
use crate::core::{VMInstruction, VirtualMachine};

use zinc_bytecode::instructions::Exit;

impl<VM: VirtualMachine> VMInstruction<VM> for Exit {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.exit(self.outputs_count)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //    use super::*;
    //    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    //    use zinc_bytecode::*;
    //
    //    #[test]
    //    fn test_exit() -> Result<(), TestingError> {
    //        VMTestRunner::new()
    //            .add(PushConst::new_untyped(1.into()))
    //            .add(Exit::new(0))
    //            .add(PushConst::new_untyped(2.into()))
    //            .test(&[1])
    //    }
}
