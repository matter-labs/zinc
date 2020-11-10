//!
//! The loop instructions.
//!

use zinc_build::LoopBegin;
use zinc_build::LoopEnd;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for LoopBegin {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        vm.loop_begin(self.iterations)
    }
}

impl<VM: IVirtualMachine> IExecutable<VM> for LoopEnd {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        vm.loop_end()
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;
    use num::One;
    use num::Zero;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_loop() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new_field(BigInt::zero()))
            .push(zinc_build::Store::new(0, 1))
            .push(zinc_build::Push::new_field(BigInt::zero()))
            .push(zinc_build::Store::new(1, 1))
            .push(zinc_build::LoopBegin::new(10))
            .push(zinc_build::Load::new(0, 1))
            .push(zinc_build::Push::new_field(BigInt::one()))
            .push(zinc_build::Add)
            .push(zinc_build::Store::new(0, 1))
            .push(zinc_build::Load::new(0, 1))
            .push(zinc_build::Load::new(1, 1))
            .push(zinc_build::Add)
            .push(zinc_build::Store::new(1, 1))
            .push(zinc_build::LoopEnd)
            .push(zinc_build::Load::new(0, 1))
            .push(zinc_build::Load::new(1, 1))
            .test(&[55, 10])
    }
}
