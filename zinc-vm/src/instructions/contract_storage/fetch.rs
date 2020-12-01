//!
//! The `StorageFetch` instruction.
//!

use zinc_types::StorageFetch;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for StorageFetch {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let eth_address = vm.pop()?.try_into_value()?;

        vm.storage_fetch(eth_address.clone(), self.field_types)?;

        vm.push(eth_address.into())?;

        Ok(())
    }
}
