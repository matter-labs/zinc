use crate::core::{VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};


use zinc_bytecode::LoadSequenceGlobal;

impl<VM: VirtualMachine> VMInstruction<VM> for LoadSequenceGlobal {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.load_global(self.address + self.len - i - 1)?;
            vm.push(value)?;
        }

        Ok(())
    }
}
