use crate::core::RuntimeError;
use crate::core::{Cell, VMInstruction, VirtualMachine};

use zinc_bytecode::LoadByIndexGlobal;

impl<VM: VirtualMachine> VMInstruction<VM> for LoadByIndexGlobal {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let index = vm.pop()?.value()?;

        let mut array = Vec::new();
        for i in 0..self.len {
            array.push(vm.load_global(self.address + i)?.value()?);
        }

        let condition = vm.condition_top()?;
        let value = vm
            .operations()
            .conditional_array_get(&condition, array.as_slice(), &index)?;
        vm.push(Cell::Value(value))
    }
}
