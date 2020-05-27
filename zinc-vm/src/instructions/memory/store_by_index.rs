use crate::core::{Cell, VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};


use zinc_bytecode::instructions::StoreByIndex;

impl<VM: VirtualMachine> VMInstruction<VM> for StoreByIndex {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?.value()?;
        let index = vm.pop()?.value()?;

        let mut array = Vec::new();
        for i in 0..self.len {
            array.push(vm.load(self.address + i)?.value()?);
        }

        let new_array = vm.operations().array_set(array.as_slice(), index, value)?;

        for (i, value) in new_array.into_iter().enumerate() {
            vm.store(self.address + i, Cell::Value(value))?;
        }

        Ok(())
    }
}
