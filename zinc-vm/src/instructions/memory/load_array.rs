use crate::core::{VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};


use zinc_bytecode::instructions::LoadSequence;

impl<VM: VirtualMachine> VMInstruction<VM> for LoadSequence {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.load(self.address + i)?;
            vm.push(value)?;
        }

        Ok(())
    }
}
