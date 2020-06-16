//!
//! The loop instructions.
//!

use zinc_bytecode::LoopBegin;
use zinc_bytecode::LoopEnd;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for LoopBegin {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.loop_begin(self.iterations)
    }
}

impl<VM: IVirtualMachine> IExecutable<VM> for LoopEnd {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.loop_end()
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_loop() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        TestRunner::new()
            .push(zinc_bytecode::Push::new_field(0.into()))
            .push(zinc_bytecode::Store::new(0, 1))
            .push(zinc_bytecode::Push::new_field(0.into()))
            .push(zinc_bytecode::Store::new(1, 1))
            .push(zinc_bytecode::LoopBegin::new(10))
            .push(zinc_bytecode::Load::new(0, 1))
            .push(zinc_bytecode::Push::new_field(1.into()))
            .push(zinc_bytecode::Add)
            .push(zinc_bytecode::Store::new(0, 1))
            .push(zinc_bytecode::Load::new(0, 1))
            .push(zinc_bytecode::Load::new(1, 1))
            .push(zinc_bytecode::Add)
            .push(zinc_bytecode::Store::new(1, 1))
            .push(zinc_bytecode::LoopEnd)
            .push(zinc_bytecode::Load::new(0, 1))
            .push(zinc_bytecode::Load::new(1, 1))
            .test(&[55, 10])
    }
}
