extern crate franklin_crypto;

use crate::core::{RuntimeError, VMInstruction, VirtualMachine};

use zinc_bytecode::{LoopBegin, LoopEnd};

impl<VM: VirtualMachine> VMInstruction<VM> for LoopBegin {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.loop_begin(self.iterations)
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for LoopEnd {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.loop_end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::{Add, Load, PushConst, Store};

    #[test]
    fn test_loop() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(PushConst::new_field(0.into()))
            .add(Store::new(0, 1))
            .add(PushConst::new_field(0.into()))
            .add(Store::new(1, 1))
            .add(LoopBegin::new(10))
            .add(Load::new(0, 1))
            .add(PushConst::new_field(1.into()))
            .add(Add)
            .add(Store::new(0, 1))
            .add(Load::new(0, 1))
            .add(Load::new(1, 1))
            .add(Add)
            .add(Store::new(1, 1))
            .add(LoopEnd)
            .add(Load::new(0, 1))
            .add(Load::new(1, 1))
            .test(&[55, 10])
    }
}
