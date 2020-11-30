//!
//! The `StorageInit` instruction.
//!

use num::BigInt;

use zinc_build::StorageInit;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for StorageInit {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let mut size: usize = self
            .field_types
            .iter()
            .map(|field| field.r#type.size())
            .sum();
        size -= zinc_const::contract::IMPLICIT_FIELDS_SIZE;

        let eth_address = Scalar::new_constant_bigint(
            BigInt::from(vm.storages_count() + 1),
            zinc_build::ScalarType::eth_address(),
        )?;

        let mut values = Vec::with_capacity(size);
        for _ in 0..size {
            values.push(vm.pop()?.try_into_value()?);
        }
        values.push(eth_address.clone());
        values.reverse();

        vm.storage_init(eth_address.clone(), values, self.field_types)?;

        vm.push(eth_address.into())?;

        Ok(())
    }
}
