//!
//! The `Store` instruction.
//!

use zinc_build::Store;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Store {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        for i in 0..self.size {
            let value = vm.pop()?;
            vm.store(self.address + self.size - i - 1, value)?;
        }

        Ok(())
    }
}
