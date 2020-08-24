//!
//! The `LoadByIndex` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::LoadByIndex;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for LoadByIndex {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let index = vm.pop()?.try_into_value()?;

        let mut array = Vec::with_capacity(self.total_size);
        for i in 0..self.total_size {
            let value = vm.load(self.address + i)?.try_into_value()?;
            array.push(value);
        }

        let condition = vm.condition_top()?;
        let mut values = Vec::with_capacity(self.value_size);
        for i in 0..self.value_size {
            let value = gadgets::array::conditional_get(
                vm.constraint_system().namespace(|| "array_get"),
                &condition,
                &array[i..],
                &index,
            )?;
            values.push(value);
        }

        for value in values.into_iter() {
            vm.push(Cell::Value(value))?;
        }

        Ok(())
    }
}
