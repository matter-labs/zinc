//!
//! The `Load` instruction.
//!

use zinc_build::Load;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Load {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        for i in 0..self.size {
            let value = vm.load(self.address + i)?;
            vm.push(value)?;
        }

        Ok(())
    }
}
