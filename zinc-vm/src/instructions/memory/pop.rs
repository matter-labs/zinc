use crate::core::RuntimeError;
use crate::core::{VMInstruction, VirtualMachine};

use zinc_bytecode::instructions::Pop;

impl<VM: VirtualMachine> VMInstruction<VM> for Pop {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        for _ in 0..self.count {
            vm.pop()?.value()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_pop() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_field(1.into()))
            .add(PushConst::new_field(2.into()))
            .add(Pop::new(1))
            .add(PushConst::new_field(3.into()))
            .add(PushConst::new_field(4.into()))
            .add(PushConst::new_field(5.into()))
            .add(Pop::new(2))
            .test(&[3, 1])
    }
}
