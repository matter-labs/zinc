//!
//! The `StorageStore` instruction.
//!

use num::bigint::ToBigInt;
use num::Signed;

use zinc_types::StorageStore;

use crate::core::contract::storage::leaf::LeafVariant;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for StorageStore {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let index = vm.pop()?.try_into_value()?;
        let eth_address = vm.pop()?.try_into_value()?;

        let mut values = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            values.push(vm.pop()?.try_into_value()?);
        }

        if vm
            .condition_top()?
            .to_bigint()
            .expect(zinc_const::panic::DATA_CONVERSION)
            .is_positive()
        {
            vm.storage_store(eth_address, index, LeafVariant::Array(values))?;
        }

        Ok(())
    }
}
