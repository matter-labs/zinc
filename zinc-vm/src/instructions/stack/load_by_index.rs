use zinc_bytecode::LoadByIndex;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for LoadByIndex {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let index = vm.pop()?.value()?;

        let mut array = Vec::with_capacity(self.array_len);
        for i in 0..self.array_len {
            let value = vm.load(self.address + i)?.value()?;
            array.push(value);
        }

        let condition = vm.condition_top()?;
        let mut values = Vec::with_capacity(self.value_len);
        for i in 0..self.value_len {
            let value = vm
                .operations()
                .conditional_array_get(&condition, &array[i..], &index)?;
            values.push(value);
        }

        for value in values.into_iter() {
            vm.push(Cell::Value(value))?;
        }

        Ok(())
    }
}
