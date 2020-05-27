use crate::core::{VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};


use zinc_bytecode::instructions::StoreSequence;

impl<VM: VirtualMachine> VMInstruction<VM> for StoreSequence {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.pop()?;
            vm.store(self.address + self.len - i - 1, value)?;
        }

        Ok(())
    }
}
