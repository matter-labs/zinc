//!
//! The `StorageStore` instruction.
//!

use num::bigint::ToBigInt;
use num::Signed;

use zinc_build::StorageStore;

use crate::core::contract::storage::leaf::LeafVariant;
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

        if let Some(condition) = vm.condition_top()?.to_bigint() {
            if condition.is_positive() {
                vm.storage_store(address, LeafVariant::Array(values))?;
            }
        }

        Ok(())
    }
}
