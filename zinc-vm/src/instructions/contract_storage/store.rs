//!
//! The `StorageStore` instruction.
//!

use zinc_bytecode::StorageStore;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for StorageStore {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let address = vm.pop()?.try_into_value()?;

        let mut values = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            values.push(vm.pop()?.try_into_value()?);
        }

        vm.storage_store(address, values)?;

        Ok(())
    }
}
