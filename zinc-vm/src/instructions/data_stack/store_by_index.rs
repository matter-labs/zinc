//!
//! The `StoreByIndex` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_types::StoreByIndex;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for StoreByIndex {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let mut array = Vec::with_capacity(self.total_size);
        for i in 0..self.total_size {
            let value = vm.load(self.address + i)?.try_into_value()?;
            array.push(value);
        }

        let mut values = Vec::with_capacity(self.value_size);
        for _ in 0..self.value_size {
            let value = vm.pop()?.try_into_value()?;
            values.push(value);
        }
        values.reverse();

        let index = vm.pop()?.try_into_value()?;

        for (i, value) in values.into_iter().enumerate() {
            let mut cs = vm.constraint_system();
            let offset = Scalar::new_constant_usize(i, index.get_type());
            let address = gadgets::arithmetic::add::add(
                cs.namespace(|| format!("address {}", i)),
                &index,
                &offset,
            )?;
            array = gadgets::array::set(&mut cs, array.as_slice(), address, value)?;
        }

        for (i, value) in array.into_iter().enumerate() {
            vm.store(self.address + i, Cell::Value(value))?;
        }

        Ok(())
    }
}
