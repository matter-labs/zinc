use zinc_bytecode::Load;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for Load {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.load(self.address + i)?;
            vm.push(value)?;
        }

        Ok(())
    }
}
