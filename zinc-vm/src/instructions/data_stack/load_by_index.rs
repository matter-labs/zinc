//!
//! The `LoadByIndex` instruction.
//!

use num::bigint::ToBigInt;
use num::ToPrimitive;

use zinc_types::LoadByIndex;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for LoadByIndex {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let index = vm.pop()?.try_into_value()?;

        let mut array = Vec::with_capacity(self.total_size);
        for i in 0..self.total_size {
            let value = vm.load(self.address + i)?.try_into_value()?;
            array.push(value);
        }

        let _condition = vm.condition_top()?;
        let mut values = Vec::with_capacity(self.value_size);
        for i in 0..self.value_size {
            let value = array
                .get(
                    i + index
                        .to_bigint()
                        .expect(zinc_const::panic::DATA_CONVERSION)
                        .to_usize()
                        .expect(zinc_const::panic::DATA_CONVERSION),
                )
                .cloned()
                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
            values.push(value);
        }

        for value in values.into_iter() {
            vm.push(Cell::Value(value))?;
        }

        Ok(())
    }
}
