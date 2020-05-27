use crate::core::{VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};


use zinc_bytecode::instructions::Load;

impl<VM: VirtualMachine> VMInstruction<VM> for Load {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.load(self.address)?;
        vm.push(value)
    }
}
