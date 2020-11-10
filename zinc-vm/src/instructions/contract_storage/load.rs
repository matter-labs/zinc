//!
//! The `StorageLoad` instruction.
//!

use zinc_build::StorageLoad;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for StorageLoad {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let address = vm.pop()?.try_into_value()?;

        let values = vm.storage_load(address, self.size)?;

        for value in values.into_iter().rev() {
            vm.push(value.into())?;
        }

        Ok(())
    }
}
