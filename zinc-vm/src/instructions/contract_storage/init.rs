//!
//! The `StorageInit` instruction.
//!

use zinc_types::StorageInit;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for StorageInit {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let size: usize = self
            .field_types
            .iter()
            .map(|field| field.r#type.size())
            .sum();

        let mut values = Vec::with_capacity(size);
        for _ in 0..size - zinc_const::contract::IMPLICIT_FIELDS_SIZE {
            values.push(vm.pop()?.try_into_value()?);
        }
        values.reverse();

        let eth_address = vm.storage_init(self.project, values, self.field_types)?;
        vm.push(eth_address.into())?;

        Ok(())
    }
}
